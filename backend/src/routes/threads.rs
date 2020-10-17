//! # Routes: Thread Element
//!
//! In this module are all the routes for working with Thread elements.
//! It handles the routes and query parameters of these routes.
//! All of these are related to the Thread element.
//!
//! ## Threads
//! Threads
//! - `/threads`: Returns a list of all threads
//!   - `filter=tasks`: Return all tasks from any thread
//!   - `filter=notes`: Return all notes from any thread
//! - `/threads/new`: Create new thread
//!   - `tasks=[uuid]`: Vector of tasks to add for thread
//!   - `notes=[uuid]`: Vector of notes to add for thread
//! - `/threads/{uuid}`: Returns a specific thread
//!   - `filter=tasks`: Return only tasks from a specific thread
//!   - `filter=notes`: Return only notes from a specific thread
//! - `/threads/{uuid}/new`: Edit a specific note
//!   - `task={uuid}`: Add task to thread
//!   - `note={uuid}`: Add note to thread
//! - `/threads/{uuid}/delete`: Delete whole thread
//!   - `task={uuid}`: Delete a task from thread
//!   - `note={uuid}`: Delete a note from thread
use super::{EingangResponse, EingangVecResponse, parse_uuid};
use crate::io::{Location, read_thread, read_thread_filepath, save_thread};
use actix_web::{web, HttpRequest, HttpResponse};
use eingang::models::{Thread, ThreadFilter, ThreadQuery, ThreadResponse};


/// Configure routes for Threads
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/threads").route(web::get().to(get_all_threads)));
    cfg.service(web::resource("/threads/new").route(web::post().to(create_new_thread)));
    cfg.service(web::resource("/threads/{uuid}").route(web::get().to(get_thread)));
    cfg.service(web::resource("/threads/{uuid}/delete").route(web::delete().to(delete_thread)));
    cfg.service(web::resource("/threads/{uuid}/new").route(web::post().to(extend_thread)));
}

async fn get_all_threads(req: HttpRequest) -> EingangVecResponse<Thread> {
    unimplemented!()
}

async fn create_new_thread(tq: web::Json<Thread>) -> HttpResponse {
    unimplemented!()
}

async fn get_thread(req: HttpRequest, filter: web::Query<ThreadFilter>) -> EingangResponse<ThreadResponse> {
    unimplemented!()
}

async fn delete_thread(req: HttpRequest, q: web::Query<ThreadQuery>) -> HttpResponse {
    unimplemented!()
}

async   fn extend_thread(req: HttpRequest, q: web::Query<ThreadQuery>) -> HttpResponse {
    unimplemented!()
}

