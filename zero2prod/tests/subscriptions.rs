#[path = "shared.rs"]
mod shared;

#[tokio::test]
async fn subscribe_should_return_a_200_for_valid_form_data() {
    // Arrange
    let app_addr = shared::spawn_app();
    let client = reqwest::Client::new();

    // Act
    let body = "name=morteza%20rv&email=test_email_addr%40gmail.com";
    let response = client
        .post(format!("{app_addr}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("unable to make the service call");

    // Assert
    assert_eq!(200, response.status().as_u16(), "server accepted the form-data");
}

#[tokio::test]
async fn subscribe_should_return_a_400_when_data_is_missing() {
    // Arrange
    let app_addr = shared::spawn_app();
    let client = reqwest::Client::new();
    let test_table_cases = vec![
        ("name=morteza%20rv", "missing the email"),
        ("email=test_email_addr%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    // Act
    for (body, err_msg) in test_table_cases {
        let response = client
            .post(format!("{app_addr}/subscriptions"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("unable to make the service call");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The api did not fail with 400 Bad Request when payload was {err_msg}"
        );
    }
}