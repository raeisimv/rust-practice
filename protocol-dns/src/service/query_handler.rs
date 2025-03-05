use crate::dns::{BytePacketBuffer, DnsPacket, Result, ResultCode};
use crate::service::lookup_recursively;
use std::net::UdpSocket;

pub fn query_handler(socket: &UdpSocket) -> Result<DnsPacket> {
    let mut req_buf = BytePacketBuffer::new();

    let (_, src) = socket.recv_from(&mut req_buf.buf)?;
    let mut request = DnsPacket::try_from(&mut req_buf)?;

    println!("query_handler: {}", request.header.id);
    let mut packet = DnsPacket::new();
    packet.header.id = request.header.id;
    packet.header.recursion_desired = true;
    packet.header.recursion_available = true;
    packet.header.response = true;

    if let Some(question) = request.questions.pop() {
        println!("question: {question:?}");
        if let Ok(result) = lookup_recursively(&question.name, question.qtype) {
            packet.questions.push(question.clone());
            packet.header.rescode = result.header.rescode;

            for r in result.answers {
                println!("answer: {r:?}");
                packet.answers.push(r);
            }
            for r in result.authorities {
                println!("authority: {r:?}");
                packet.authorities.push(r);
                break;
            }
            for r in result.resources {
                println!("resource: {r:?}");
                packet.resources.push(r);
                break;
            }
        } else {
            packet.header.rescode = ResultCode::SERVFAIL;
        }
    } else {
        packet.header.rescode = ResultCode::FORMERR;
    }

    let mut res_buf = BytePacketBuffer::new();
    packet.write(&mut res_buf)?;

    let len = res_buf.pos();
    let data = res_buf.get_range(0, len)?;
    socket.send_to(data, src)?;

    Ok(packet)
}
