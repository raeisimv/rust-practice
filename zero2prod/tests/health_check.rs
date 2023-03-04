#[path = "shared.rs"]
mod shared;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = shared::spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", app.address))
        .send()
        .await
        .expect("unable to make the service call");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
