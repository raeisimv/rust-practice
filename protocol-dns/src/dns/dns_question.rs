use crate::dns::{BytePacketBuffer, DnsResult, QueryType};

#[derive(Clone, Debug)]
pub struct DnsQuestion {
    pub name: String,
    pub qtype: QueryType,
}

impl DnsQuestion {
    pub fn new(name: String, qtype: QueryType) -> Self {
        Self { name, qtype }
    }

    pub fn read(&mut self, buf: &mut BytePacketBuffer) -> DnsResult {
        buf.read_qname(&mut self.name)?;
        let qtype = buf.read_u16()?;
        self.qtype = qtype.into();

        let _class = buf.read_u16()?;

        Ok(())
    }
    pub fn write(&self, buf: &mut BytePacketBuffer) -> DnsResult {
        buf.write_qname(&self.name)?;
        buf.write_u16(self.qtype.into())?;
        buf.write_u16(1)?;

        Ok(())
    }
}
