use protocol_dns::dns::Result;
use protocol_dns::service::query_handler;

fn main() -> Result {
    let socket = std::net::UdpSocket::bind(("0.0.0.0", 2053))?;
    println!("DNS server is listening to: 0.0.0.0:2053");
    loop {
        match query_handler(&socket) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("error: {e:?}");
            }
        }
    }
}
