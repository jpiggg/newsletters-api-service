use newsletter_api_service::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run(TcpListener::bind("127.0.0.1:8000").unwrap())?.await
}
