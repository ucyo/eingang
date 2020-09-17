//! Routes of the backend service
use serde::Serialize;

#[derive(Debug, Serialize, Default)]
struct Data {
    name: String,
    id: usize,
}

#[get("/{name}")]
async fn index(web::Path(name): web::Path<String>) -> impl Responder {
    format!("Hello {}", name)
}
