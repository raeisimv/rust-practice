use crate::dns::{BytePacketBuffer, DnsPacket, DnsResult, ResultCode};
use crate::service::lookup_recursively;
use std::net::{Ipv4Addr, UdpSocket};

pub fn query_handler(socket: &UdpSocket, ns_addr: Ipv4Addr) -> DnsResult {
    let mut req_buf = BytePacketBuffer::new();

    let (_, src) = socket.recv_from(&mut req_buf.buf)?;
    let mut request = DnsPacket::try_from(&mut req_buf)?;

    let mut packet = DnsPacket::default();
    packet.header.id = request.header.id;
    packet.header.recursion_desired = true;
    packet.header.recursion_available = true;
    packet.header.response = true;

    if let Some(question) = request.questions.pop() {
        if let Ok(result) = lookup_recursively(&question.name, question.qtype, ns_addr) {
            packet.questions.push(question.clone());
            packet.header.rescode = result.header.rescode;

            for r in result.answers {
                packet.answers.push(r);
            }
            for r in result.authorities {
                packet.authorities.push(r);
            }
            for r in result.resources {
                packet.resources.push(r);
            }
        } else {
            packet.header.rescode = ResultCode::SERVFAIL;
        }
    } else {
        packet.header.rescode = ResultCode::FORMERR;
    }

    let buf: BytePacketBuffer = packet.try_into()?;
    socket.send_to(&buf, src)?;

    Ok(())
}
