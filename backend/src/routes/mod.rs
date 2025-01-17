//! Routes of the backend service
//!
//! **Ideally all routes are just one/two liner and simple function calls to the backend**
//!
use crate::STORAGE;
use actix_web::{web, HttpRequest, HttpResponse, Responder, Result};
use eingang::models::Data;
use serde_qs as qs;
use std::{fs::File, io::Write};

// TODO Make oneliner to io
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("json/{value}").route(web::get().to(index)));
    cfg.service(web::resource("/query").route(web::get().to(saving)));
    cfg.service(web::resource("/serde/query").route(web::get().to(serialize)));
    cfg.service(web::resource("/save").route(web::post().to(permanent)));
    cfg.service(web::resource("/load").route(web::get().to(loading)));
}

/// Index method for simple counter
///
/// Use route to (un)serialize information about the object
/// e.g. `http://localhost:8081/json/1422`
pub async fn index(req: HttpRequest) -> Result<web::Json<Data>> {
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
async fn saving(q: web::Query<Data>) -> Result<web::Json<Data>> {
    let d = q.clone();
    Ok(web::Json(d))
}

// Use serde_qs for query string to (un)serialize information about the object
// e.g. http://localhost:8081/serde/query?value=2423&id=746217fd-da9c-4139-8b8e-cf4089dd680e
async fn serialize(req: HttpRequest) -> Result<web::Json<Data>> {
    let q = req.query_string();
    let d: Data = qs::from_str(q).unwrap_or_default();
    Ok(web::Json(d))
}

// Send data to server and safe it on disk
// e.g. curl -v -d '{"value":2423,"id":"746217fd-da9c-4139-8b8e-cf4089dd680e"}' -H 'Content-Type: application/json' http://localhost:8081/save
async fn permanent(data: web::Json<Data>) -> impl Responder {
    let buffer = File::create(STORAGE).unwrap();
    let mut writer = std::io::BufWriter::new(buffer);
    let _ = serde_json::to_writer_pretty(&mut writer, &data.into_inner()).unwrap();
    writer.flush().unwrap();
    HttpResponse::NoContent()
}

async fn loading(_: HttpRequest) -> Result<web::Json<Data>> {
    let buffer = File::open(STORAGE).unwrap();
    let rdr = std::io::BufReader::new(buffer);
    let data: Data = serde_json::from_reader(rdr).unwrap();
    Ok(web::Json(data))
}

pub mod notes;
pub mod tasks;
pub mod threads;
pub mod journal;

/// Return a vector of json serializeable data
pub type EingangVecResponse<T> = Result<web::Json<Vec<T>>, web::HttpResponse>;

/// Return a json representation of serializable data
pub type EingangResponse<T> = Result<web::Json<T>, web::HttpResponse>;

fn parse_uuid(req: HttpRequest) -> String {
    req.match_info()
        .get("uuid")
        .unwrap() // TODO Better parsing, since this could panic
        .parse()
        .unwrap()
}
