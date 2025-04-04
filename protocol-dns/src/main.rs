use protocol_dns::dns::DnsResult;
use protocol_dns::service::query_handler;

const DEFAULT_NS_ADDR: &str = "198.41.0.4";

fn main() -> DnsResult {
    let addr = ("0.0.0.0", 2053);
    let ns_addr = DEFAULT_NS_ADDR.parse().unwrap();
    let socket = std::net::UdpSocket::bind(addr)?;
    println!("DNS server is listening to: {addr:?}");
    loop {
        match query_handler(&socket, ns_addr) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("error in handling the query: {e:?}");
            }
        }
    }
}
