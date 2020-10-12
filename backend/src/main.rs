//! Main application for backend service.
use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use eingang_backend::routes::*;
use eingang_backend::{HOST, PORT, FRONTEND_HOST, FRONTEND_PORT};
use actix_web::http::header;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = format!("{}:{}", HOST, PORT);
    let frontend = format!("{}:{}", FRONTEND_HOST, FRONTEND_PORT);

    HttpServer::new(move || {
        App::new().wrap(Cors::new()
        .allowed_origin(&frontend)
        .allowed_methods(vec!["GET", "POST", "DELETE", "PATCH"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .max_age(3600)
        .finish())
            .service(web::resource("/json/{value}").route(web::get().to(index)))
            .service(saving)
            .service(permanent)
            .service(serialize)
            .configure(notes::config)
            .service(loading)
    })
    .bind(&address)?
    .run()
    .await
}
