//! Routes of the backend service

use actix_web::{get, web, Responder};

#[get("/{name}")]
async fn index(web::Path(name): web::Path<String>) -> impl Responder {
    format!("Hello {}", name)
}
