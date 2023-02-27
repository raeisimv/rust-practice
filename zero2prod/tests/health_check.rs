use std::net::TcpListener;
use std::time::Duration;
use tokio;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{address}/health_check"))
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to send the request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("cannot find a random port to bind");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener)
        .expect("failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{port}")
}