use std::net::TcpListener;
use tokio;
use zero2prod;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Starting the server....");
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("cannot find an TcpListener to bind");

    zero2prod::startup::run(listener)?.await
}
