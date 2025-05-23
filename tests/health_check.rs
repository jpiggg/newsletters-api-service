use reqwest;
use sqlx::{PgConnection,Connection};
use std::net::TcpListener;
use newsletter_api_service::configuration::get_configuration;


fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = newsletter_api_service::run(listener).expect("Failed to load server");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();
    let response = client.get(&format!("{}/health_check", &address)).send().await.expect("Failed to send a request to health_check endpoint");
    
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    
    let mut connection = PgConnection::connect(&connection_string).await.expect("Failed to connect to database");

    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to subscribe a user");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "ursula_le_guin@40gmail,com");
    assert_eq!(saved.name, "le guin");

}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
// Arrange
let app_address = spawn_app();
let client = reqwest::Client::new(); let test_cases = vec![
    ("name=le%20guin", "missing the email"), ("email=ursula_le_guin%40gmail.com", "missing the name"), ("", "missing both name and email")
];
for (invalid_body, error_message) in test_cases { // Act
let response = client
.post(&format!("{}/subscriptions", &app_address)) .header("Content-Type", "application/x-www-form-urlencoded") .body(invalid_body)
.send()
.await
.expect("Failed to execute request.");
        // Assert
        assert_eq!(
400,
response.status().as_u16(),
// Additional customised error message on test failure
"The API did not fail with 400 Bad Request when the payload was {}.", error_message
); }
}
