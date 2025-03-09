// Project ergonomic error and result types

pub type TlsError = Box<dyn std::error::Error>;

pub type TlsResult<T = (), E = TlsError> = Result<T, E>;

#[derive(Clone, Debug)]
pub enum DecodeError {
    InvalidMessage(String),
}
