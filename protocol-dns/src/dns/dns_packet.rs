use crate::dns::{BytePacketBuffer, DnsHeader, DnsQuestion, DnsRecord, QueryType, Result};
use std::net::Ipv4Addr;

#[derive(Clone, Debug)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authorities: Vec<DnsRecord>,
    pub resources: Vec<DnsRecord>,
}

impl DnsPacket {
    pub fn new() -> Self {
        Self {
            header: DnsHeader::new(),
            questions: vec![],
            answers: vec![],
            authorities: vec![],
            resources: vec![],
        }
    }
    pub fn from_buffer(buf: &mut BytePacketBuffer) -> Result<Self> {
        let mut result = DnsPacket::new();
        result.header.read(buf)?;

        for _ in 0..result.header.questions {
            let mut q = DnsQuestion::new("".into(), QueryType::UNKNOWN(0));
            q.read(buf)?;
            result.questions.push(q);
        }
        for _ in 0..result.header.answers {
            let answer = DnsRecord::read(buf)?;
            result.answers.push(answer);
        }
        for _ in 0..result.header.authoritative_entries {
            let authority = DnsRecord::read(buf)?;
            result.authorities.push(authority);
        }
        for _ in 0..result.header.resource_entries {
            let resource = DnsRecord::read(buf)?;
            result.resources.push(resource);
        }
        Ok(result)
    }
    pub fn write(&mut self, buf: &mut BytePacketBuffer) -> Result {
        self.header.questions = self.questions.len() as u16;
        self.header.answers = self.answers.len() as u16;
        self.header.authoritative_entries = self.authorities.len() as u16;
        self.header.resource_entries = self.resources.len() as u16;
        self.header.write(buf)?;

        for q in self.questions.iter() {
            q.write(buf)?;
        }
        for record in self.answers.iter() {
            record.write(buf)?;
        }
        for record in self.authorities.iter() {
            record.write(buf)?;
        }
        for record in self.resources.iter() {
            record.write(buf)?;
        }
        Ok(())
    }
    pub fn get_random_a(&self) -> Option<Ipv4Addr> {
        self.answers
            .iter()
            .filter_map(|x| match x {
                DnsRecord::A { addr, .. } => Some(*addr),
                _ => None,
            })
            .next()
    }
    pub fn get_ns<'a>(&'a self, qname: &'a str) -> impl Iterator<Item = (&'a str, &'a str)> {
        self.authorities
            .iter()
            .filter_map(|x| match x {
                DnsRecord::NS { domain, host, .. } => Some((domain.as_str(), host.as_str())),
                _ => None,
            })
            .filter(|(domain, _)| qname.starts_with(*domain))
    }
    pub fn get_resolve_ns(&self, qname: &str) -> Option<Ipv4Addr> {
        self.get_ns(qname)
            .flat_map(|(_, host)| {
                self.resources.iter().filter_map(move |x| match x {
                    DnsRecord::A { domain, addr, .. } if domain == host => Some(addr),
                    _ => None,
                })
            })
            .map(|x| *x)
            .next()
    }
    pub fn get_unresolved_ns<'a>(&'a self, qname: &'a str) -> Option<&'a str> {
        self.get_ns(qname).map(|(_, host)| host).next()
    }
}
