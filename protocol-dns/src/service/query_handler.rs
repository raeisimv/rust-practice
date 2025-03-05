use crate::dns::{BytePacketBuffer, DnsPacket, Result, ResultCode};
use crate::service::lookup_recursively;
use std::net::UdpSocket;

pub fn query_handler(socket: &UdpSocket) -> Result {
    let mut req_buf = BytePacketBuffer::new();

    let (_, src) = socket.recv_from(&mut req_buf.buf)?;
    let mut request = DnsPacket::try_from(&mut req_buf)?;

    let mut packet = DnsPacket::new();
    packet.header.id = request.header.id;
    packet.header.recursion_desired = true;
    packet.header.recursion_available = true;
    packet.header.response = true;

    if let Some(question) = request.questions.pop() {
        if let Ok(result) = lookup_recursively(&question.name, question.qtype) {
            packet.questions.push(question.clone());
            packet.header.rescode = result.header.rescode;

            for r in result.answers {
                packet.answers.push(r);
            }
            for r in result.authorities {
                packet.authorities.push(r);
                break;
            }
            for r in result.resources {
                packet.resources.push(r);
                break;
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
