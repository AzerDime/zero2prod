// Ian Guy, 2022. Working along with a textbook!

// use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

// // Greet is now deprecated.
// // async fn greet(req: HttpRequest) -> impl Responder {
// //     let name = req.match_info().get("name").unwrap_or("World");
// //     format!("Hello {}!", &name)
// // }

// async fn health_check(req: HttpRequest) -> impl Responder {
//     HttpResponse::Ok()
// }

// #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
//         .bind("127.0.0.1:8000")?
//         .run()
//         .await
// }
use sqlx::postgres::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
