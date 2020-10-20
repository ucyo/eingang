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
use eingang::models::{JournalResponse, JournalQuery, JournalFilter};

/// Return a vector of json serializeable data
pub type EingangVecResponseError<T> = Result<web::Json<Vec<T>>, HttpResponse>;  // TODO Apply this setup also to the others

/// Configure routes for Journal view
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/journal").route(web::get().to(journal)));
}

// TODO Use query and not json object
async fn journal(_: HttpRequest, d: web::Json<JournalQuery>) -> EingangVecResponseError<JournalResponse> {
    let data = d.into_inner();
    if data.during.is_some() && data.untouched.is_some() {
        return Err(HttpResponse::BadRequest().json("Either during OR untouched"))
    }
    if (data.during.is_some() || data.untouched.is_some()) && (data.before.is_some() || data.after.is_some()) {
        return Err(HttpResponse::BadRequest().json("Either time period OR moment"))
    }
    let filter = data.filter.unwrap_or_default();

    match filter {
        JournalFilter::All => {
            return Err(HttpResponse::BadRequest().json("All filtering not yet implemented"))
        },
        JournalFilter::Notes => {
            return Err(HttpResponse::BadRequest().json("Note filtering not yet implemented"))
        },
        JournalFilter::Tasks => {
            return Err(HttpResponse::BadRequest().json("Task filtering not yet implemented"))
        },
        JournalFilter::Threads => {
            return Err(HttpResponse::BadRequest().json("Thread filtering not yet implemented"))
        },
    }
}
