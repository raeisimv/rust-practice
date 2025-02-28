use protocol_dns::dns::*;
use std::io::Read;

fn main() -> Result {
    let mut f = std::fs::File::open("data/query_packet.txt")?;
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
