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
use eingang::models::{JournalFilter, JournalQuery, JournalResponse};
use crate::io::{get_all_notes, filter_notes};
use crate::io::{get_all_tasks, filter_tasks};
use crate::io::{get_all_threads, filter_threads};

/// Return a vector of json serializeable data
pub type EingangVecResponseError<T> = Result<web::Json<Vec<T>>, HttpResponse>; // TODO Apply this setup also to the others

/// Configure routes for Journal view
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/journal").route(web::get().to(journal)));
}

// TODO Use query and not json object
async fn journal(
    _: HttpRequest,
    d: web::Json<JournalQuery>,
) -> EingangVecResponseError<JournalResponse> {
    let data = d.into_inner();
    if data.during.is_some() && data.untouched.is_some() {
        return Err(HttpResponse::BadRequest().json("Either during OR untouched"));
    }
    if (data.during.is_some() || data.untouched.is_some())
        && (data.before.is_some() || data.after.is_some())
    {
        return Err(HttpResponse::BadRequest().json("Either time period OR moment"));
    }
    let filter = data.filter.unwrap_or_default();
    let result = filtering(filter, &data);
    Ok(web::Json(result))

}


fn filtering(filter: JournalFilter, data: &JournalQuery) -> Vec<JournalResponse> {
    match filter {
        JournalFilter::All => {
            let mut notes = filtering(JournalFilter::Notes, data);
            let mut tasks = filtering(JournalFilter::Tasks, data);
            let mut threads = filtering(JournalFilter::Threads, data);
            notes.append(&mut tasks);
            notes.append(&mut threads);
            notes
        }
        JournalFilter::Notes => {
            let notes = get_all_notes().unwrap();
            if data.during.is_some() {
                let tstamp = data.during.unwrap().to_timestamp();
                let filtered = filter_notes(notes, None, Some(tstamp));
                filtered.into_iter().map(|n| JournalResponse::Note(n)).collect()
        } else if data.untouched.is_some() {
                let tstamp = data.untouched.unwrap().to_timestamp();
                let filtered = filter_notes(notes, Some(tstamp), None);
                let result = filtered.into_iter().map(|n| JournalResponse::Note(n)).collect();
                result
        } else {
                let filtered = filter_notes(notes, data.before_to_timestamp(), data.after_to_timestamp());
                let result = filtered.into_iter().map(|n| JournalResponse::Note(n)).collect();
                result
            }
        }
        JournalFilter::Tasks => {
            let tasks = get_all_tasks().unwrap();
            if data.during.is_some() {
                let tstamp = data.during.unwrap().to_timestamp();
                let filtered = filter_tasks(tasks, None, Some(tstamp));
                let result = filtered.into_iter().map(|t| JournalResponse::Task(t)).collect();
                result
        } else if data.untouched.is_some() {
                let tstamp = data.untouched.unwrap().to_timestamp();
                let filtered = filter_tasks(tasks, Some(tstamp), None);
                let result = filtered.into_iter().map(|t| JournalResponse::Task(t)).collect();
                result
        } else {
                let filtered = filter_tasks(tasks, data.before_to_timestamp(), data.after_to_timestamp());
                let result = filtered.into_iter().map(|t| JournalResponse::Task(t)).collect();
                result
            }
        }
        JournalFilter::Threads => {
            let threads = get_all_threads().unwrap();
            if data.during.is_some() {
                let tstamp = data.during.unwrap().to_timestamp();
                let filtered = filter_threads(threads, None, Some(tstamp));
                let result = filtered.into_iter().map(|t| JournalResponse::Thread(t)).collect();
                result
        } else if data.untouched.is_some() {
                let tstamp = data.untouched.unwrap().to_timestamp();
                let filtered = filter_threads(threads, Some(tstamp), None);
                let result = filtered.into_iter().map(|t| JournalResponse::Thread(t)).collect();
                result
        } else {
                let filtered = filter_threads(threads, data.before_to_timestamp(), data.after_to_timestamp());
                let result = filtered.into_iter().map(|t| JournalResponse::Thread(t)).collect();
                result
            }
        }
    }
}
