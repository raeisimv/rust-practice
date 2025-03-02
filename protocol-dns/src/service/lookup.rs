use crate::dns::{BytePacketBuffer, DnsPacket, DnsQuestion, QueryType, Result};

pub fn lookup(qname: &str, qtype: QueryType) -> Result<DnsPacket> {
    let server = ("8.8.8.8", 53);
    let socket = std::net::UdpSocket::bind(("0.0.0.0", 0))?;

    let mut packet = DnsPacket::new();
    packet.header.id = 6868;
    packet.header.recursion_desired = true;
    packet.questions.push(DnsQuestion::new(qname.into(), qtype));

    let mut buf = BytePacketBuffer::new();
    packet.write(&mut buf)?;
    socket.send_to(&buf.buf[..buf.pos], server)?;

    let mut buf = BytePacketBuffer::new();
    socket.recv(&mut buf.buf)?;
    let packet = DnsPacket::from_buffer(&mut buf)?;

    Ok(packet)
}
