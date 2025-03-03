use protocol_dns::dns::Result;
use protocol_dns::service::query_handler;

fn main() -> Result {
    let addr = ("0.0.0.0", 2053);
    let socket = std::net::UdpSocket::bind(addr)?;
    println!("DNS server is listening to: {addr:?}");
    loop {
        match query_handler(&socket) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("error: {e:?}");
            }
        }
    }
}
