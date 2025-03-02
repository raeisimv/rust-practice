#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum QueryType {
    UNKNOWN(u16),
    A,     //1
    NS,    // 2
    CNAME, // 5
    MX,    // 15
    AAA,   // 28
}

impl From<u16> for QueryType {
    fn from(value: u16) -> Self {
        match value {
            1 => QueryType::A,
            2 => QueryType::NS,
            5 => QueryType::CNAME,
            15 => QueryType::MX,
            28 => QueryType::AAA,
            x => QueryType::UNKNOWN(x),
        }
    }
}

impl Into<u16> for QueryType {
    fn into(self) -> u16 {
        match self {
            QueryType::UNKNOWN(x) => x,
            QueryType::A => 1,
            QueryType::NS => 2,
            QueryType::CNAME => 5,
            QueryType::MX => 15,
            QueryType::AAA => 28,
        }
    }
}
