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
use crate::io::{read_note, read_task};
use eingang::models::{Idable, TaskUuid, NoteUuid};

/// Configure routes for Threads
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/threads").route(web::get().to(get_all_threads)));
    cfg.service(web::resource("/threads/new").route(web::post().to(create_new_thread)));
    cfg.service(web::resource("/threads/{uuid}").route(web::get().to(get_thread)));
    cfg.service(web::resource("/threads/{uuid}/delete").route(web::delete().to(delete_thread)));
    cfg.service(web::resource("/threads/{uuid}/new").route(web::post().to(extend_thread)));
}

async fn get_all_threads(_: HttpRequest, q: web::Query<ThreadQuery>) -> EingangVecResponse<ThreadResponse> {
    let folder = Location::Thread.get_basefolder();
    let result: Vec<_> = std::fs::read_dir(folder).unwrap()
        .map(|e| e.map(|d| d.path()))
        .filter_map(|f| read_thread_filepath(&f.unwrap()).ok())
        .collect();
    let query = q.into_inner();
    let r = if query.filter.is_some() {
       match query.filter.unwrap() {
        ThreadFilter::Tasks => result.into_iter().map(|f| ThreadResponse::Tasks(f.tasks)).collect(),
        ThreadFilter::Notes => result.into_iter().map(|f| ThreadResponse::Notes(f.notes)).collect(),
       }
    } else {
        result.into_iter().map(|f| ThreadResponse::Threads(f)).collect()
    };
    Ok(web::Json(r))
}

async fn create_new_thread(q: web::Json<ThreadQuery>) -> HttpResponse {
    // TODO breaks when UUID is not valid, should return BadRequest
    let tq = q.into_inner();
    let tasks: Vec<TaskUuid> = tq.tasks
        .unwrap_or_default()
        .iter()
        .filter_map(|uuid| read_task(&uuid).ok())
        .map(|uuid| uuid.get_uuid())
        .collect();
    let notes: Vec<NoteUuid> = tq.notes
        .unwrap_or_default()
        .iter()
        .filter_map(|uuid| read_note(&uuid).ok())
        .map(|uuid| uuid.get_uuid())
        .collect();
    let thread = Thread::with_tasks_and_notes(tasks, notes);
    save_thread(&thread);
    HttpResponse::Ok().json(thread)
}

async fn get_thread(req: HttpRequest, q: web::Query<ThreadQuery>) -> EingangResponse<ThreadResponse> {
    let uuid: String = parse_uuid(req);
    let thread = read_thread(&uuid).unwrap();
    let query = q.into_inner();
    let r = if query.filter.is_some() {
       match query.filter.unwrap() {
        ThreadFilter::Tasks => ThreadResponse::Tasks(thread.tasks),
        ThreadFilter::Notes => ThreadResponse::Notes(thread.notes),
       }
    } else {
        ThreadResponse::Threads(thread)
    };
    Ok(web::Json(r))
}

async fn delete_thread(req: HttpRequest, q: web::Query<ThreadQuery>) -> HttpResponse {
    unimplemented!()
}

async   fn extend_thread(req: HttpRequest, q: web::Query<ThreadQuery>) -> HttpResponse {
    unimplemented!()
}

