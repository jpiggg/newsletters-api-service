use reqwest;
use std::net::TcpListener;

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