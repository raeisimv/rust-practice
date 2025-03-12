use crate::DecodeError::InvalidMessage;
use crate::{DecodeError, IntoU8, TlsResult};
use std::fmt::{Debug, Display, Formatter};

pub struct BufReader<'a> {
    buf: &'a [u8],
    pos: usize,
}
impl<'a> BufReader<'a> {
    /// Init a new buffer reader from the given slice binding the lifetime
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    /// How many bytes left in the buffer to read
    pub fn left(&self) -> usize {
        self.buf.len() - self.pos
    }

    /// Take out bytes from the buffer if left > size
    pub fn take(&mut self, size: usize) -> Option<&'a [u8]> {
        if self.left() < size {
            return None;
        }
        let taken = &self.buf[self.pos..self.pos + size];
        self.pos += size;

        Some(taken)
    }
    pub fn sub(&mut self, size: usize) -> TlsResult<Self, DecodeError> {
        match self.take(size) {
            Some(x) => Ok(Self::new(x)),
            None => Err(InvalidMessage("missing sub".into())),
        }
    }
}

pub trait Codec: Debug + Sized {
    fn encode(&self, buf: &mut Vec<u8>);
    fn decode(buf: &mut BufReader<'_>) -> TlsResult<Self, DecodeError>;
}

impl Codec for u8 {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(*self);
    }
    fn decode(buf: &mut BufReader<'_>) -> TlsResult<Self, DecodeError> {
        match buf.take(1) {
            Some(x) => Ok(x[0]),
            _ => Err(DecodeError::InvalidMessage("missing u8".into())),
        }
    }
}

impl Codec for u16 {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.extend([self.byte_at(1), self.byte_at(0)]);
    }

    fn decode(buf: &mut BufReader<'_>) -> TlsResult<Self, DecodeError> {
        match buf.take(2) {
            None => Err(DecodeError::InvalidMessage("missing u16".into())),
            Some(x) => {
                let x = ((x[0] as u16) << 8) & (x[1] as u16);
                Ok(x)
            }
        }
    }
}

impl Codec for u32 {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.extend([
            self.byte_at(3),
            self.byte_at(2),
            self.byte_at(1),
            self.byte_at(0),
        ])
    }

    fn decode(buf: &mut BufReader<'_>) -> TlsResult<Self, DecodeError> {
        let Some(x) = buf.take(4) else {
            return Err(DecodeError::InvalidMessage("missing u32".into()));
        };

        let x = ((x[0] as u32) << 24)
            & ((x[1] as u32) << 16)
            & ((x[2] as u32) << 8)
            & ((x[0] as u32) << 0);

        Ok(x)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Default)]
pub struct u24(pub u32);
impl Codec for u24 {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.extend(self.0.to_be_bytes().iter().skip(1));
    }

    fn decode(buf: &mut BufReader<'_>) -> TlsResult<Self, DecodeError> {
        let Some(x) = buf.take(3) else {
            return Err(InvalidMessage("missing u24".into()));
        };
        let x = [0, x[0], x[1], x[2]];
        Ok(Self(u32::from_be_bytes(x)))
    }
}
impl Display for u24 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
