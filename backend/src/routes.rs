//! Routes of the backend service
use super::STORAGE;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder, Result};
use eingang::models::Data;
use serde_qs as qs;
use std::{fs::File, io::Write};

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

// Send data to server and safe it on disk
// e.g. curl -v -d '{"value": 213, "id":32}' -H 'Content-Type: application/json' http://localhost:8081/save
#[post("/save")]
async fn permanent(data: web::Json<Data>) -> impl Responder {
    let buffer = File::create(STORAGE).unwrap();
    let mut writer = std::io::BufWriter::new(buffer);
    let _ = serde_json::to_writer_pretty(&mut writer, &data.into_inner()).unwrap();
    writer.flush().unwrap();
    HttpResponse::Ok()
}
