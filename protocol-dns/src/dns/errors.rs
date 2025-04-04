pub type Error = Box<dyn std::error::Error>;
pub type DnsResult<T = (), E = Error> = Result<T, E>;
