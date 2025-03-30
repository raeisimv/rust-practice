// Project ergonomic error and result types

use std::fmt::{Display, Formatter};

pub type TlsError = Box<dyn std::error::Error>;

pub type TlsResult<T = (), E = TlsError> = Result<T, E>;

#[derive(Clone, Debug)]
pub enum DecodeError {
    InvalidMessage(String),
    InvalidConversion(String),
    CryptoError(String),
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let txt = match self {
            DecodeError::InvalidMessage(x) => format!("Invalid Message: {x}"),
            DecodeError::InvalidConversion(x) => format!("Invalid Conversion: {x}"),
            DecodeError::CryptoError(x) => format!("Crypto Error: {x}"),
        };
        write!(f, "{}", txt)
    }
}
