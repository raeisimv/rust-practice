use crate::DecodeError::InvalidMessage;
use crate::{BufReader, Codec, DecodeError, ProtocolVersion, TlsResult, create_random_u8_32};

#[derive(Copy, Clone, Debug)]
pub struct Random {
    buf: [u8; 32],
}
impl Random {
    pub fn new() -> Self {
        Self::fixed()
    }
    pub fn new_random() -> Self {
        Self {
            buf: create_random_u8_32(),
        }
    }
    pub fn from_slice(v: &[u8]) -> Self {
        let mut buf = [0; 32];
        for (i, x) in v.iter().enumerate() {
            buf[i] = *x;
        }

        Self { buf }
    }

    pub const fn fixed() -> Self {
        Self {
            buf: [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
                0x1C, 0x1D, 0x1E, 0x1F,
            ],
        }
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
            data: create_random_u8_32(),
            size: 32,
        }
    }
    pub fn fixed() -> Self {
        Self {
            data: [
                0xE0, 0xE1, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6, 0xE7, 0xE8, 0xE9, 0xEA, 0xEB, 0xEC, 0xED,
                0xEE, 0xEF, 0xF0, 0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7, 0xF8, 0xF9, 0xFA, 0xFB,
                0xFC, 0xFD, 0xFE, 0xFF,
            ],
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
        assert!(self.size <= 32);
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

pub fn create_client_hello(host: String, pub_key: &[u8]) -> Vec<u8> {
    let ver = u16::from(ProtocolVersion::TLSv1_2).to_be_bytes();
    let session_id = SessionId::fixed();
    let handshake = {
        let mut buf = Vec::new();

        // version
        buf.extend_from_slice(&u16::from(ProtocolVersion::TLSv1_2).to_be_bytes());
        // random
        buf.extend_from_slice(Random::new().as_ref());
        // session id
        buf.push(0x00); // empty session id
        // extend_with_prefix_length(ListLength::U8, &mut buf, |buf| {
        //     session_id.encode(buf);
        // });
        // cipher suite
        // buf.extend_from_slice(&[0x00, 0x02, 0x13, 0x01]); // cipher suites: TLS_AES_128_GCM_SHA256
        buf.extend_from_slice(&[0x00, 0x08, 0x13, 0x02, 0x13, 0x03, 0x13, 0x01, 0x00, 0xff]); // cipher suites

        // extend_with_prefix_length(ListLength::U16, &mut buf, |buf| {
        //    // buf.extend_from_slice(&[0x13, 0x01]);
        //     buf.extend_from_slice(&u16::from(CipherSuite::TLS_AES_128_GCM_SHA256).to_be_bytes());
        // });
        // compression
        buf.extend_from_slice(&[0x01, 0x00]);

        // extensions
        let ext_buf = create_client_hello_ext(host, pub_key);
        buf.extend_from_slice(&(ext_buf.len() as u16).to_be_bytes());
        buf.extend_from_slice(&ext_buf);

        buf
    };

    let pyl_size = handshake.len() as u16;
    let mut buf = Vec::new();
    buf.extend_from_slice(&[0x16]); // ClientHello
    buf.extend_from_slice(&ver); // version
    buf.extend_from_slice(&(pyl_size + 4).to_be_bytes());
    buf.extend_from_slice(&[0x01, 0x00]); // handshake
    buf.extend_from_slice(&pyl_size.to_be_bytes());
    buf.extend_from_slice(&handshake);

    buf
}

fn create_client_hello_ext(host: String, pub_key: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();

    // add ServerNam
    ExtServerName::from_host(host).encode(&mut buf);
    //
    // // ext EllipticCurves -> NamedGroup -> Groups
    // // add_extension(0x0a, &[0x00, 0x02, 0x00, 0x1d], &mut buf);
    // let pyl = u16::from(NamedGroup::X25519).to_be_bytes();
    // add_extension(ExtensionType::EllipticCurves.into(), &pyl, &mut buf);
    //
    // // ext SignatureAlgorithm
    // let pyl = [
    //     0x04, 0x03, 0x08, 0x04, 0x04, 0x01, 0x05, 0x03, 0x08, 0x05, 0x05, 0x01, 0x08, 0x06, 0x06,
    //     0x01, 0x02, 0x01,
    // ]; // TODO: remove hardcoded values
    // add_extension(ExtensionType::SignatureAlgorithms.into(), &pyl, &mut buf);
    //
    // add_extension(ExtensionType::KeyShare.into(), pub_key, &mut buf);
    //
    // let pyl = [0x01];
    // add_extension(ExtensionType::PSKKeyExchangeModes.into(), &pyl, &mut buf);
    //
    // let pyl = u16::from(ProtocolVersion::TLSv1_0).to_be_bytes();
    // add_extension(ExtensionType::SupportedVersions.into(), &pyl, &mut buf);

    buf
}

fn add_extension(id: u16, v: &[u8], buf: &mut Vec<u8>) {
    let len = v.len() as u16;
    buf.extend_from_slice(&id.to_be_bytes());
    buf.extend_from_slice(&len.to_be_bytes());
    buf.extend_from_slice(v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_encode_ext_server_name() {
        let host = "example.ulfheim.net";
        let mut buf = Vec::new();
        ExtServerName::from_host(host.into()).encode(&mut buf);

        assert_eq!(
            buf,
            &[
                0x0, 0x0, // ext ID
                0x00, 0x18, // total ext length
                0x00, 0x16, // first entry length (and only)
                0x00, // DNS Host name (sni)
                0x00, 0x13, // entry length
                // actual SNI (domain name)
                0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2e, 0x75, 0x6c, 0x66, 0x68, 0x65, 0x69,
                0x6d, 0x2e, 0x6e, 0x65, 0x74
            ]
        );
        assert!(buf.iter().skip(9).eq(host.as_bytes()));
    }
}
