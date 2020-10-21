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
use super::{parse_uuid, EingangResponse, EingangVecResponse};
use crate::io::{get_all_threads as gat, read_note, read_task};
use crate::io::{read_thread, save_thread, Location};
use actix_web::{web, HttpRequest, HttpResponse};
use eingang::models::{Idable, NoteUuid, TaskUuid};
use eingang::models::{Thread, ThreadFilter, ThreadQuery, ThreadResponse};

/// Configure routes for Threads
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/threads").route(web::get().to(get_all_threads)));
    cfg.service(web::resource("/threads/new").route(web::post().to(create_new_thread)));
    cfg.service(web::resource("/threads/{uuid}").route(web::get().to(get_thread)));
    cfg.service(web::resource("/threads/{uuid}/delete").route(web::delete().to(delete_thread)));
    cfg.service(web::resource("/threads/{uuid}/add").route(web::patch().to(extend_thread)));
}

async fn get_all_threads(
    _: HttpRequest,
    q: web::Query<ThreadQuery>,
) -> EingangVecResponse<ThreadResponse> {
    let result = gat().unwrap();
    let query = q.into_inner();
    let r = if query.filter.is_some() {
        match query.filter.unwrap() {
            ThreadFilter::Tasks => result
                .into_iter()
                .map(|f| ThreadResponse::Tasks(f.tasks))
                .collect(),
            ThreadFilter::Notes => result
                .into_iter()
                .map(|f| ThreadResponse::Notes(f.notes))
                .collect(),
        }
    } else {
        result
            .into_iter()
            .map(|f| ThreadResponse::Threads(f))
            .collect()
    };
    Ok(web::Json(r))
}

async fn create_new_thread(q: web::Json<ThreadQuery>) -> HttpResponse {
    // TODO breaks when UUID is not valid, should return BadRequest
    let tq = q.into_inner();
    let tasks: Vec<TaskUuid> = tq
        .tasks
        .unwrap_or_default()
        .iter()
        .filter_map(|uuid| read_task(&uuid).ok())
        .map(|uuid| uuid.get_uuid())
        .collect();
    let notes: Vec<NoteUuid> = tq
        .notes
        .unwrap_or_default()
        .iter()
        .filter_map(|uuid| read_note(&uuid).ok())
        .map(|uuid| uuid.get_uuid())
        .collect();
    let thread = Thread::with_tasks_and_notes(tasks, notes);
    save_thread(&thread);
    HttpResponse::Ok().json(thread)
}

async fn get_thread(
    req: HttpRequest,
    q: web::Query<ThreadQuery>,
) -> EingangResponse<ThreadResponse> {
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
    let uuid: String = parse_uuid(req);
    let query = q.into_inner();

    if query.task.is_some() && query.note.is_some() {
        return HttpResponse::BadRequest().json("Either remove task or note from Thread");
    }

    // TODO Move deletion in `eingang-base`
    if query.task.is_some() {
        let mut thread = read_thread(&uuid).unwrap();
        let task_uuid = query.task.unwrap();
        let pos = thread.tasks.iter().position(|t| t.to_string() == task_uuid);
        match pos {
            Some(p) => {
                thread.tasks.remove(p);
                thread.update_modified_date();
                save_thread(&thread);
                HttpResponse::NoContent().json("Successful")
            }
            None => HttpResponse::BadRequest().json("Task UUID is not associated"),
        }
    } else if query.note.is_some() {
        let mut thread = read_thread(&uuid).unwrap();
        let note_uuid = query.note.unwrap();
        let pos = thread.notes.iter().position(|n| n.to_string() == note_uuid);
        match pos {
            Some(p) => {
                thread.notes.remove(p);
                thread.update_modified_date();
                save_thread(&thread);
                HttpResponse::NoContent().json("Successful")
            }
            None => HttpResponse::BadRequest().json("Note UUID is not associated"),
        }
    } else {
        let file = Location::Thread.create_filename(&uuid);
        match std::fs::remove_file(file) {
            Ok(_) => HttpResponse::NoContent().json("Successful"),
            _ => HttpResponse::BadRequest().json("UUID is not associated"),
        }
    }
}

async fn extend_thread(req: HttpRequest, q: web::Query<ThreadQuery>) -> HttpResponse {
    let uuid: String = parse_uuid(req);
    let mut thread = read_thread(&uuid).unwrap();
    let query = q.into_inner();

    if query.task.is_some() && query.note.is_some() {
        return HttpResponse::BadRequest().json("Either remove task or note");
    }

    if query.task.is_some() {
        let task = read_task(&query.task.unwrap()).unwrap();
        thread.add_task(task.get_uuid());
        save_thread(&thread);
        return HttpResponse::NoContent().json("Task added");
    } else if query.note.is_some() {
        let note = read_note(&query.note.unwrap()).unwrap();
        thread.add_note(note.get_uuid());
        save_thread(&thread);
        return HttpResponse::NoContent().json("Note added");
    } else {
        HttpResponse::BadRequest().json("No task or note given to add")
    }
}
