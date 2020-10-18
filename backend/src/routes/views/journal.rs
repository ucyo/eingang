//! # View: Journal
//!
//! ## Routes
//!
//! - `/journal`: No outout without query parameters.
//!   - `after=moment`:       Return all data last modified after certain date  (either/or `untouched`)
//!   - `before=moment`:      Return all data last modified before certain date
//!   - `during=period`:      Return all data last modified during the last period (either/or `untouched`, `moments`)
//!   - `untouched=period`:   Return all data last modified before the last period
//!   - `kind=[notes|tasks]`: Return only data on Notes or Tasks (default: Threads)
//!
//! The actual request for a period can be translated to the same function.
//! Therefore only the cases `after`, `before`, and `after` and `before` needs
//! to be implemented.
use actix_web::{web, HttpRequest, HttpResponse};
use eingang::models::{JournalResponse, JournalQuery};

/// Return a vector of json serializeable data
pub type EingangVecResponseError<T> = Result<web::Json<Vec<T>>, HttpResponse>;  // TODO Apply this setup also to the others

/// Configure routes for Journal view
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/journal").route(web::get().to(journal)));
}

async fn journal(_: HttpRequest, q: web::Json<JournalQuery>) -> EingangVecResponseError<JournalResponse> {
    // TODO Use query and not json object
    let query = q.into_inner();
    if query.during.is_some() && query.untouched.is_some() {
        return Err(HttpResponse::BadRequest().json("Either during OR untouched"))
    }
    if (query.during.is_some() || query.untouched.is_some()) && (query.before.is_some() || query.after.is_some()) {
        return Err(HttpResponse::BadRequest().json("Either time period OR moment"))
    }
    // from here on further the queries should be valid
    unimplemented!()
}
