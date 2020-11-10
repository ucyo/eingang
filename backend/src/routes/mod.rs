//! Routes of the backend service
//!
//! **Ideally all routes are just one/two liner and simple function calls to the backend**
//!
use actix_web::{web, HttpRequest, Result};
pub mod journal;
pub mod notes;
pub mod tasks;
pub mod threads;

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
