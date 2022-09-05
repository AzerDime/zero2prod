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
use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    run(listener)?.await
}
