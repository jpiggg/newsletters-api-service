use actix_web::{App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;

use crate::routes;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(routes::health_check)
            .service(routes::subscribe)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
