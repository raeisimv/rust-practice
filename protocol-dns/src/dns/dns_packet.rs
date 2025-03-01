use crate::dns::{BytePacketBuffer, DnsHeader, DnsQuestion, DnsRecord, QueryType, Result};

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
}
