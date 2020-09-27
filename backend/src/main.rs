//! Main application for backend service.
use actix_web::{App, HttpServer};
use eingang_backend::routes::*;
use eingang_backend::{HOST, PORT};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = format!("{}:{}", HOST, PORT);

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(saving)
            .service(permanent)
            .service(serialize)
            .service(loading)
    })
    .bind(address.as_str())?
    .run()
    .await
}
