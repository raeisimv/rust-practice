use crate::shared::spawn_app;

#[path = "shared.rs"]
mod shared;

#[tokio::test]
async fn subscribe_should_return_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let body = "name=morteza%20rv&email=test_email_addr%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("unable to make the service call");

    // Assert
    assert_eq!(200, response.status().as_u16(), "server accepted the form-data");

    // Act
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("failed to fetch saved subscription");

    // Assert
    assert_eq!(saved.email, "test_email_addr@gmail.com");
    assert_eq!(saved.name, "morteza rv");
}

#[tokio::test]
async fn subscribe_should_return_a_400_when_data_is_missing() {
    // Arrange
    let app = shared::spawn_app().await;
    let client = reqwest::Client::new();
    let test_table_cases = vec![
        ("name=morteza%20rv", "missing the email"),
        ("email=test_email_addr%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    // Act
    for (body, err_msg) in test_table_cases {
        let response = client
            .post(format!("{}/subscriptions", app.address))
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

#[tokio::test]
async fn subscribe_should_return_a_200_when_fields_are_present_but_empty() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let table_test_cases = vec![
        ("name=&email=sample_345%40gmail.com", "empty name"),
        ("name=Jacob&email=", "empty email"),
        ("name=JacobY&email=definitely-not-an-email", "invalid email"),
    ];

    for (body, msg) in table_test_cases {
        // Act
        let resp = client
            .post(format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("failed to send request for subscriptions")
            ;
        // Assert
        assert_eq!(200, resp.status().as_u16(),
                   "The API did not return a 200 OK when payload was {}", msg
        );
    }
}