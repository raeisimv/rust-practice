#[path = "shared.rs"]
mod shared;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = shared::spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("unable to make the service call");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
