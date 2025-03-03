use crate::dns::{BytePacketBuffer, QueryType, Result};
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Clone, Debug)]
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
    NS {
        domain: String,
        host: String,
        ttl: u32,
    },
    CNAME {
        domain: String,
        host: String,
        ttl: u32,
    },
    MX {
        domain: String,
        priority: u16,
        host: String,
        ttl: u32,
    },
    AAA {
        domain: String,
        addr: Ipv6Addr,
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
            QueryType::AAA => {
                let raw = [
                    buf.read_u32()?,
                    buf.read_u32()?,
                    buf.read_u32()?,
                    buf.read_u32()?,
                ];
                let addr = Ipv6Addr::new(
                    ((raw[0] >> 16) & 0xFFFF) as u16,
                    ((raw[0] >> 0) & 0xFFFF) as u16,
                    ((raw[1] >> 16) & 0xFFFF) as u16,
                    ((raw[1] >> 0) & 0xFFFF) as u16,
                    ((raw[2] >> 16) & 0xFFFF) as u16,
                    ((raw[2] >> 0) & 0xFFFF) as u16,
                    ((raw[3] >> 16) & 0xFFFF) as u16,
                    ((raw[3] >> 0) & 0xFFFF) as u16,
                );
                Ok(DnsRecord::AAA { domain, addr, ttl })
            }
            QueryType::NS => {
                let mut host = String::new();
                buf.read_qname(&mut host)?;

                Ok(DnsRecord::NS { domain, host, ttl })
            }
            QueryType::CNAME => {
                let mut host = String::new();
                buf.read_qname(&mut host)?;

                Ok(DnsRecord::CNAME { domain, host, ttl })
            }
            QueryType::MX => {
                let priority = buf.read_u16()?;
                let mut host = String::new();
                buf.read_qname(&mut host)?;

                Ok(DnsRecord::MX {
                    domain,
                    priority,
                    host,
                    ttl,
                })
            }
            QueryType::UNKNOWN(_) => {
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
    pub fn write(&self, buf: &mut BytePacketBuffer) -> Result<usize> {
        let start_pos = buf.pos();
        match *self {
            DnsRecord::UNKNOWN { .. } => {
                println!("skipping writing UNKNOWN DNS Record: {self:?}");
            }
            DnsRecord::A {
                ref domain,
                ref addr,
                ttl,
            } => {
                buf.write_qname(domain)?;
                buf.write_u16(QueryType::A.into())?;
                buf.write_u16(1)?;
                buf.write_u32(ttl)?;
                buf.write_u16(4)?;

                let octets = addr.octets();
                buf.write_u8(octets[0])?;
                buf.write_u8(octets[1])?;
                buf.write_u8(octets[2])?;
                buf.write_u8(octets[3])?;
            }
            DnsRecord::NS {
                ref domain,
                ref host,
                ttl,
            } => {
                buf.write_qname(domain)?;
                buf.write_u16(QueryType::NS.into())?;
                buf.write_u16(1)?;
                buf.write_u32(ttl)?;

                let pos = buf.pos();
                buf.write_u16(0)?;
                buf.write_qname(host)?;
                let size = buf.pos() - (pos + 2);
                buf.set_u16(pos, size as u16)?;
            }
            DnsRecord::CNAME {
                ref domain,
                ref host,
                ttl,
            } => {
                buf.write_qname(domain)?;
                buf.write_u16(QueryType::CNAME.into())?;
                buf.write_u16(1)?;
                buf.write_u32(ttl)?;

                let pos = buf.pos();
                buf.write_u16(0)?;
                buf.write_qname(host)?;
                let size = buf.pos() - (pos + 2);
                buf.set_u16(pos, size as u16)?;
            }
            DnsRecord::MX {
                ref domain,
                ref host,
                priority,
                ttl,
            } => {
                buf.write_qname(domain)?;
                buf.write_u16(QueryType::MX.into())?;
                buf.write_u16(1)?;
                buf.write_u32(ttl)?;

                let pos = buf.pos();
                buf.write_u16(0)?;
                buf.write_u16(priority)?;
                buf.write_qname(host)?;
                let size = buf.pos() - (pos + 2);
                buf.set_u16(pos, size as u16)?;
            }
            DnsRecord::AAA {
                ref addr,
                ref domain,
                ttl,
            } => {
                buf.write_qname(domain)?;
                buf.write_u16(QueryType::AAA.into())?;
                buf.write_u16(1)?;
                buf.write_u32(ttl)?;
                buf.write_u16(16)?;

                for segment in addr.segments() {
                    buf.write_u16(segment)?;
                }
            }
        }
        Ok(buf.pos() - start_pos)
    }
}
