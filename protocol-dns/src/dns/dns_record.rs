use crate::dns::{BytePacketBuffer, QueryType, Result};
use std::net::Ipv4Addr;

pub enum DnsRecord {
    UNKNOWN {
        domain: String,
        qtype: u16,
        data_len: u16,
        ttl: u32,
    },
    A {
        domain: String,
        addr: Ipv4Addr,
        ttl: u32,
    },
}

impl DnsRecord {
    pub fn read(buf: &mut BytePacketBuffer) -> Result<Self> {
        let mut domain = String::new();
        buf.read_qname(&mut domain)?;
        let qtype: QueryType = buf.read_u16()?.into();
        let _ = buf.read_u16()?;
        let ttl = buf.read_u32()?;
        let data_len = buf.read_u16()?;

        match qtype {
            QueryType::A => {
                let raw_adr = buf.read_u32()?;
                let addr = Ipv4Addr::new(
                    ((raw_adr >> 24) & 0xFF) as u8,
                    ((raw_adr >> 16) & 0xFF) as u8,
                    ((raw_adr >> 8) & 0xFF) as u8,
                    ((raw_adr >> 0) & 0xFF) as u8,
                );

                Ok(DnsRecord::A { domain, addr, ttl })
            }
            QueryType::UNKNOWN(x) => {
                buf.step(data_len as usize)?;

                Ok(DnsRecord::UNKNOWN {
                    domain,
                    data_len,
                    ttl,
                    qtype: qtype.into(),
                })
            }
        }
    }
}
