/// Project ergonomic error and result types
use std::fmt::{Display, Formatter};

/// The general error type
pub type TlsError = Box<dyn std::error::Error>;

/// The standard result type
pub type TlsResult<T = (), E = TlsError> = Result<T, E>;

#[derive(Clone, Debug)]
pub enum DecodeError {
    CryptoError(String),
    InvalidMessage(String),
    InvalidConversion(String),
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let txt = match self {
            DecodeError::CryptoError(x) => format!("Crypto Error: {x}"),
            DecodeError::InvalidMessage(x) => format!("Invalid Message: {x}"),
            DecodeError::InvalidConversion(x) => format!("Invalid Conversion: {x}"),
        };
        write!(f, "{}", txt)
    }
}
