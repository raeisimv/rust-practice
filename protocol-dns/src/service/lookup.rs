use crate::dns::{BytePacketBuffer, DnsPacket, DnsQuestion, QueryType, Result};
use std::net::Ipv4Addr;

pub fn lookup(qname: &str, qtype: QueryType, server: (Ipv4Addr, u16)) -> Result<DnsPacket> {
    let socket = std::net::UdpSocket::bind(("0.0.0.0", 0))?;

    let mut packet = DnsPacket::new();
    packet.header.id = 6868;
    packet.header.recursion_desired = true;
    packet.header.questions = 1;
    packet.questions.push(DnsQuestion::new(qname.into(), qtype));

    let mut buf = BytePacketBuffer::new();
    packet.write(&mut buf)?;
    socket.send_to(&buf.buf[..buf.pos], server)?;

    let mut buf = BytePacketBuffer::new();
    socket.recv(&mut buf.buf)?;
    let packet = DnsPacket::try_from(&mut buf)?;

    Ok(packet)
}
