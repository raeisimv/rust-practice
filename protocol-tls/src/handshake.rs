use crate::DecodeError::InvalidMessage;
use crate::{BufReader, CipherSuite, Codec, DecodeError, ProtocolVersion, TlsResult};

#[derive(Copy, Clone, Debug)]
pub struct Random {
    buf: [u8; 32],
}
impl Random {
    pub fn new() -> Self {
        use std::time::SystemTime;
        use std::time::UNIX_EPOCH;
        let mut seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let mut buf = [0_u8; 32];
        for i in 0..32_usize {
            seed ^= seed.rotate_left(13);
            buf[i] = seed as u8;
            if buf[i] == 0 {
                buf[i] = 128;
            }
        }
        Self { buf }
    }
    pub fn from_slice(v: &[u8]) -> Self {
        let mut buf = [0; 32];
        for (i, x) in v.iter().enumerate() {
            buf[i] = *x;
        }

        Self { buf }
    }
}
impl Codec for Random {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.buf)
    }

    fn decode(buf: &mut BufReader<'_>) -> TlsResult<Self, DecodeError> {
        let Some(bytes) = buf.take(32) else {
            return Err(InvalidMessage("missing Random".into()));
        };

        Ok(Self::from_slice(bytes))
    }
}
impl AsRef<[u8]> for Random {
    fn as_ref(&self) -> &[u8] {
        &self.buf
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct SessionId {
    data: [u8; 32],
    size: usize,
}
impl SessionId {
    pub fn random() -> Self {
        Self {
            data: Random::new().buf,
            size: 32,
        }
    }
    pub fn from_slice(buf: &[u8], size: usize) -> Self {
        assert!(size < 32);
        let mut id = Self {
            data: [0; 32],
            size,
        };
        for (i, x) in buf.iter().take(size).enumerate() {
            id.data[i] = *x;
        }
        id
    }
}
impl Codec for SessionId {
    fn encode(&self, buf: &mut Vec<u8>) {
        assert!(self.size < 32);
        buf.push(self.size as u8);
        if self.size > 0 {
            buf.extend_from_slice(&self.data[..self.size])
        }
    }

    fn decode(buf: &mut BufReader<'_>) -> TlsResult<Self, DecodeError> {
        let Ok(size) = u8::decode(buf) else {
            return Err(InvalidMessage("missing size of SessionId".into()));
        };
        if size > 32 {
            return Err(InvalidMessage("session size exceed 32 bytes".into()));
        }
        let Some(data) = buf.take(size as usize) else {
            return Err(InvalidMessage("missing session data".into()));
        };
        Ok(Self::from_slice(data, size as usize))
    }
}
impl AsRef<[u8]> for SessionId {
    fn as_ref(&self) -> &[u8] {
        &self.data[..self.size]
    }
}

#[derive(Clone, Debug)]
pub struct ExtServerName {
    // typ: 0x00 0x00
    host: String,
}
impl ExtServerName {
    pub fn from_host(host: String) -> Self {
        Self { host }
    }
}
impl Codec for ExtServerName {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&[0x00, 0x00]); // The ext id

        let host = self.host.as_bytes();
        let host_len = host.len() as u16;
        let the_list_entry_len = host_len + 3;
        let the_ext_payload_len = the_list_entry_len + 2;

        // insert the extension's sizes
        // total and final data that comes along
        buf.extend_from_slice(&the_ext_payload_len.to_be_bytes());
        // the +2 in the above calculation is the count of the following 2 bytes
        buf.extend_from_slice(&the_list_entry_len.to_be_bytes());

        // the +3 in above calculation is the count of the following 3 bytes
        buf.extend_from_slice(&[0x00]); // DNS Hostname,
        buf.extend_from_slice(&host_len.to_be_bytes());

        // the real hostname in bytes
        buf.extend_from_slice(host);
    }

    fn decode(buf: &mut BufReader<'_>) -> TlsResult<Self, DecodeError> {
        let size = u16::decode(buf)? as usize;
        let Some(chunk) = buf.take(size) else {
            return Err(InvalidMessage("missing ext.ServerName chunk".into()));
        };

        Ok(Self {
            host: String::from_utf8_lossy(chunk).into(),
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ExtKeyShare {
    // typ: 0x00 0x33
    // x25519: 0x00 0x1d Curve25519
    alg: u16,
    key: [u8; 32],
}
impl ExtKeyShare {
    pub fn new() -> Self {
        Self {
            alg: 0x001d, // Curve25519
            key: Random::new().buf,
        }
    }
}
impl Codec for ExtKeyShare {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&[
            0x00, 0x33, // id
            0x00, 0x26, // data len
            0x00, 0x24, // data len
        ]);
        buf.extend_from_slice(&self.alg.to_be_bytes());
        buf.extend_from_slice(&[0x00, 0x20]); // key len = 32
        buf.extend_from_slice(&self.key);
    }

    fn decode(buf: &mut BufReader<'_>) -> TlsResult<Self, DecodeError> {
        let size = u16::decode(buf)? as usize;
        let Ok(mut data) = buf.sub(size) else {
            return Err(InvalidMessage("missing ExtKeyShare".into()));
        };

        let size = u16::decode(&mut data)? as usize - 2;
        let alg = u16::decode(&mut data)?;
        let Some(key_org) = data.take(size) else {
            return Err(InvalidMessage("missing ExtKeyShare key".into()));
        };

        // Copy
        let mut key = [0; 32];
        for (i, x) in key_org.iter().enumerate() {
            key[i] = *x;
        }
        Ok(Self { alg, key })
    }
}

pub struct ClientHello {
    client_version: ProtocolVersion,
    random: Random,
    session_id: SessionId,
    cipher_suite: Vec<CipherSuite>,
    compression_method: u16,
    extensions: Vec<u8>,
}
