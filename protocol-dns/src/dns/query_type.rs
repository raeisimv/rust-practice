#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum QueryType {
    UNKNOWN(u16),
    A, //1
}

impl From<u16> for QueryType {
    fn from(value: u16) -> Self {
        match value {
            1 => QueryType::A,
            x => QueryType::UNKNOWN(x),
        }
    }
}

impl Into<u16> for QueryType {
    fn into(self) -> u16 {
        match self {
            QueryType::UNKNOWN(x) => x,
            QueryType::A => 1,
        }
    }
}
