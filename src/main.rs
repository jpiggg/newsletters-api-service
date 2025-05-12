use newsletter_api_service::{configuration, run};
use std::net::TcpListener;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = configuration::get_configuration().expect("Failed to read configuration");
    let port = config.application_port;
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}
