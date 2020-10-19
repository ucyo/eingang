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

async fn journal(_: HttpRequest, d: web::Json<JournalQuery>) -> EingangVecResponseError<JournalResponse> {
    // TODO Use query and not json object
    let data = d.into_inner();
    if data.during.is_some() && data.untouched.is_some() {
        return Err(HttpResponse::BadRequest().json("Either during OR untouched"))
    }
    if (data.during.is_some() || data.untouched.is_some()) && (data.before.is_some() || data.after.is_some()) {
        return Err(HttpResponse::BadRequest().json("Either time period OR moment"))
    }

    // testing

    if data.during.is_some() {
        let reference = &data.during.unwrap();
        let ts = reference.to_timestamp();
        println!("During {:#?}", ts);
    }
    // testing
    if data.before.is_some() {
        let reference = &data.before_to_timestamp().unwrap();
        println!("Untouched {:#?}", reference);
    }
    println!("{:#?}", data.filter.unwrap_or_default());
    // from here on further the queries should be valid
    unimplemented!()
}
