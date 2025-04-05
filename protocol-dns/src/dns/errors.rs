pub type DnsError = Box<dyn std::error::Error>;
pub type DnsResult<T = (), E = DnsError> = Result<T, E>;
