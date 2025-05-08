use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use std::net::TcpListener;


#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String
}


#[actix_web::get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::post("/subscriptions")]
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(subscribe)
            .service(
                actix_web::web::resource(vec!["/", "/{name}"])
                .route(actix_web::web::get().to(greet))
                .route(actix_web::web::get().to(greet))
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
