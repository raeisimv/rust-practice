use crate::DecodeError::InvalidMessage;
use crate::{BufReader, Codec, DecodeError, TlsResult};

#[derive(Copy, Clone, Debug)]
pub struct Random {
    pub buf: [u8; 32],
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
        buf.extend(self.buf)
    }

    fn decode(buf: &mut BufReader<'_>) -> TlsResult<Self, DecodeError> {
        let Some(bytes) = buf.take(32) else {
            return Err(InvalidMessage("missing Random".into()));
        };

        Ok(Self::from_slice(bytes))
    }
}

#[derive(Copy, Clone, Debug)]
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
            buf.extend(&self.data[..self.size])
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
