use protocol_dns::dns::*;
use std::io::Read;

fn main() -> Result {
    let qname = "gmail.com";
    let qtype = QueryType::MX;
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

    println!("{:#?}", packet.header);

    for q in packet.questions {
        println!("{:#?}", q);
    }
    for rec in packet.answers {
        println!("{:#?}", rec);
    }
    for rec in packet.authorities {
        println!("{:#?}", rec);
    }
    for rec in packet.resources {
        println!("{:#?}", rec);
    }

    println!("app ran successfully.");
    Ok(())
}

fn _main() -> Result {
    let mut f = std::fs::File::open("data/query_packet.txt")?;
    // let mut f = std::fs::File::open("data/query_resp.txt")?;
    let mut buf = BytePacketBuffer::new();
    f.read(&mut buf.buf)?;

    let packet = DnsPacket::from_buffer(&mut buf)?;

    println!("{:#?}", packet.header);

    for q in packet.questions {
        println!("{:#?}", q);
    }
    for rec in packet.answers {
        println!("{:#?}", rec);
    }
    for rec in packet.authorities {
        println!("{:#?}", rec);
    }
    for rec in packet.resources {
        println!("{:#?}", rec);
    }

    println!("app ran successfully.");
    Ok(())
}
