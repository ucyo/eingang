//! Routes of the backend service
use crate::STORAGE;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder, Result};
use eingang::models::Data;
use serde_qs as qs;
use std::{fs::File, io::Write};

// Use route to (un)serialize information about the object
// e.g. http://localhost:8081/json/1422
#[get("/json/{value}")]
async fn index(req: HttpRequest) -> Result<web::Json<Data>> {
    let value = req
        .match_info()
        .get("value")
        .unwrap_or("Not found")
        .parse()
        .unwrap_or_default();
    let d = Data::new(value);
    Ok(web::Json(d))
}

// Use native query struct `actix_web::web::Query` to (un)serialize information about the object
// e.g. http://localhost:8081/query?value=2423&id=746217fd-da9c-4139-8b8e-cf4089dd680e
#[get("/query")]
async fn saving(q: web::Query<Data>) -> Result<web::Json<Data>> {
    let d = q.clone();
    Ok(web::Json(d))
}

// Use serde_qs for query string to (un)serialize information about the object
// e.g. http://localhost:8081/serde/query?value=2423&id=746217fd-da9c-4139-8b8e-cf4089dd680e
#[get("/serde/query")]
async fn serialize(req: HttpRequest) -> Result<web::Json<Data>> {
    let q = req.query_string();
    let d: Data = qs::from_str(q).unwrap_or_default();
    Ok(web::Json(d))
}

// Send data to server and safe it on disk
// e.g. curl -v -d '{"value":2423,"id":"746217fd-da9c-4139-8b8e-cf4089dd680e"}' -H 'Content-Type: application/json' http://localhost:8081/save
#[post("/save")]
async fn permanent(data: web::Json<Data>) -> impl Responder {
    let buffer = File::create(STORAGE).unwrap();
    let mut writer = std::io::BufWriter::new(buffer);
    let _ = serde_json::to_writer_pretty(&mut writer, &data.into_inner()).unwrap();
    writer.flush().unwrap();
    HttpResponse::Ok()
}

#[get("/load")]
async fn loading(_: HttpRequest) -> Result<web::Json<Data>> {
    let buffer = File::open(STORAGE).unwrap();
    let rdr = std::io::BufReader::new(buffer);
    let data: Data = serde_json::from_reader(rdr).unwrap();
    Ok(web::Json(data))
}

pub mod notes;
