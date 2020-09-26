//! Routes of the backend service
use actix_web::{get, web, HttpRequest, Result};
use serde_qs as qs;
use eingang::models::Data;

// Use route to (un)serialize information about the object
// e.g. http://localhost:8081/json/2342214/1422
#[get("/json/{id}/{name}")]
async fn index(req: HttpRequest) -> Result<web::Json<Data>> {
    let value = req
        .match_info()
        .get("name")
        .unwrap_or("Not found")
        .parse()
        .unwrap_or_default();
    let id: usize = req
        .match_info()
        .get("id")
        .unwrap_or("Not found")
        .parse()
        .unwrap_or_default();
    let d = Data::new(value, id);
    Ok(web::Json(d))
}

// Use native query struct `actix_web::web::Query` to (un)serialize information about the object
// e.g. http://localhost:8081/query?name=1422&id=2342214
#[get("/query")]
async fn saving(q: web::Query<Data>) -> Result<web::Json<Data>> {
    let d = q.clone();
    Ok(web::Json(d))
}

// Use serde_qs for query string to (un)serialize information about the object
// e.g. http://localhost:8081/serde/query?name=1422&id=2342214
#[get("/serde/query")]
async fn serialize(req: HttpRequest) -> Result<web::Json<Data>> {
    let q = req.query_string();
    let d: Data = qs::from_str(q).unwrap_or_default();
    Ok(web::Json(d))
}
