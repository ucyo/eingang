//! Main application for backend service.
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{middleware, App, HttpServer};
use eingang_backend::routes::{config, notes, tasks};
use eingang_backend::{FRONTEND_HOST, FRONTEND_PORT, HOST, PORT};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = format!("{}:{}", HOST, PORT);
    let frontend = format!("{}:{}", FRONTEND_HOST, FRONTEND_PORT);
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin(&frontend)
                    .allowed_methods(vec!["GET", "POST", "DELETE", "PATCH"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .configure(config)
            .configure(notes::config)
            .configure(tasks::config)
    })
    .bind(&address)?
    .run()
    .await
}
