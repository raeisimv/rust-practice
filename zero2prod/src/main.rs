use std::net::TcpListener;
use tokio;
use zero2prod;
use zero2prod::conf;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("server -> initializing");
    let conf = conf::get_configuration()
        .expect("failed to get_configuration");

    let address = format!("127.0.0.1:{}", conf.application_port);
    println!("server -> starting at: {}", address);

    let listener = TcpListener::bind(address)
        .expect("cannot find an TcpListener to bind");

    zero2prod::startup::run(listener)?.await
}
