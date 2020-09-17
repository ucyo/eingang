//! Routes of the backend service
use actix_web::{web, get, Result};
use serde::Serialize;

#[derive(Debug, Serialize, Default)]
struct Data {
    name: String,
    id: usize,
}

#[get("/{name}")]
async fn index(web::Path(name): web::Path<String>) -> Result<web::Json<Data>> {
    let d = Data {
        name,
        ..Default::default()
    };
    Ok(web::Json(d))
}
