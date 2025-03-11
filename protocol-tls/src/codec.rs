use crate::{DecodeError, IntoU8, TlsResult};
use std::fmt::Debug;

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
    pub fn sub(&mut self, size: usize) -> Option<Self> {
        let buf = self.take(size)?;
        Some(Self::new(buf))
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
