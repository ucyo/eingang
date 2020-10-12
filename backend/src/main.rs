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
    let cors = Cors::new()
        // .allowed_origin(frontend.as_str())
        .allowed_origin("http://127.0.0.1:8080")  // TODO update to take frontend variable 
        .allowed_methods(vec!["GET", "POST", "DELETE", "PATCH"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .max_age(3600)
        .finish();

    HttpServer::new(move || {
        App::new().wrap(cors)
            .service(web::resource("/json/{value}").route(web::get().to(index)))
            .service(saving)
            .service(permanent)
            .service(serialize)
            .service(notes::create_new_note)
            .service(notes::get_note)
            .service(notes::delete_note)
            .service(notes::update_note)
            .service(notes::get_all_notes)
            .service(loading)
    })
    .bind(address.as_str())?
    .run()
    .await
}
