//! Main application for backend service.
use actix_web::HttpServer;
use actix_web::App;
use eingang_backend::{routes::index, HOST, PORT};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = format!("{}:{}", HOST, PORT);
    HttpServer::new(|| App::new().service(index)).bind(address.as_str())?.run().await
}
