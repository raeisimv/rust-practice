use std::net::TcpListener;
use tokio;

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("cannot find a random port to bind");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener)
        .expect("failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{port}")
}
