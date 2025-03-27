use crate::DecodeError::InvalidMessage;
use crate::{DecodeError, TlsResult};
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
            _ => Err(InvalidMessage("missing u8".into())),
        }
    }
}

impl Codec for u16 {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_be_bytes());
    }

    fn decode(buf: &mut BufReader<'_>) -> TlsResult<Self, DecodeError> {
        match buf.take(2) {
            None => Err(InvalidMessage("missing u16".into())),
            Some(x) => {
                let x = ((x[0] as u16) << 8) & (x[1] as u16);
                Ok(x)
            }
        }
    }
}

impl Codec for u32 {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_be_bytes())
    }

    fn decode(buf: &mut BufReader<'_>) -> TlsResult<Self, DecodeError> {
        let Some(x) = buf.take(4) else {
            return Err(InvalidMessage("missing u32".into()));
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
impl u24 {
    pub fn to_be_bytes(&self) -> [u8; 3] {
        let it = self.0.to_be_bytes();
        let mut bytes = [0; 3];
        for (i, x) in it.iter().skip(1).enumerate() {
            bytes[i] = *x;
        }
        bytes
    }
}

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

#[derive(Copy, Clone, Debug)]
pub enum ListLength {
    U8,
    U16,
    U24,
}
pub fn extend_with_prefix_length<F>(size: ListLength, buf: &mut Vec<u8>, appender: F)
where
    F: FnOnce(&mut Vec<u8>),
{
    let offset = buf.len();
    match size {
        ListLength::U8 => buf.extend_from_slice(&[0xFF]),
        ListLength::U16 => buf.extend_from_slice(&[0xFF, 0xFF]),
        ListLength::U24 => buf.extend_from_slice(&[0xFF, 0xFF, 0xFF]),
    }

    appender(buf);

    match size {
        ListLength::U8 => {
            let len = buf.len() - offset - 1;
            buf[offset] = len as u8;
        }
        ListLength::U16 => {
            let len = buf.len() - offset - 2;
            buf.splice(offset.., (len as u16).to_be_bytes());
        }
        ListLength::U24 => {
            let len = u24((buf.len() - offset - 3) as u32);
            buf.splice(offset.., len.to_be_bytes());
        }
    }
}

pub trait TlsListElement {
    const SIZE_LEN: ListLength;
}

impl<T: Codec + TlsListElement> Codec for Vec<T> {
    fn encode(&self, buf: &mut Vec<u8>) {
        extend_with_prefix_length(T::SIZE_LEN, buf, |buf| {
            for x in self {
                x.encode(buf);
            }
        })
    }

    fn decode(buf: &mut BufReader<'_>) -> TlsResult<Self, DecodeError> {
        let len = match T::SIZE_LEN {
            ListLength::U8 => u8::decode(buf)? as usize,
            ListLength::U16 => u16::decode(buf)? as usize,
            ListLength::U24 => u24::decode(buf)?.0 as usize,
        };
        let mut sub = buf.sub(len)?;
        let mut ret = Self::new();
        while sub.left() > 0 {
            let x = T::decode(&mut sub)?;
            ret.push(x);
        }

        Ok(ret)
    }
}
