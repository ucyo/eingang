//! # Routes: Note Element
//!
//! In this module are all the routes for working with Note elements.
//! It handles the routes and query parameters of these routes.
//! All of these are related to the Note element.
//!
//! The current list of elements are:
//! - Http responses send by the backend to the user
//! - Available routes for the user
//! - Helper functions for interaction with the underlying filesystem
use super::{EingangResponse, EingangVecResponse, parse_uuid};
use crate::io::{Location, read_note, read_note_filepath, save_note};
use actix_web::{web, HttpRequest, HttpResponse};
use eingang::models::{Note, NoteQuery, Idable};


/// Configure routes for Notes
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/notes").route(web::get().to(get_all_notes)));
    cfg.service(web::resource("/notes/new").route(web::post().to(create_new_note)));
    cfg.service(web::resource("/notes/{uuid}").route(web::get().to(get_note)));
    cfg.service(web::resource("/notes/{uuid}/delete").route(web::delete().to(delete_note)));
    cfg.service(web::resource("/notes/{uuid}/update").route(web::patch().to(update_note)));
}

/// Return all Notes
///
/// This route returns all notes saved on the filesystem.
async fn get_all_notes(_: HttpRequest) -> EingangVecResponse<Note> {
    let folder = Location::Note.get_basefolder();
    let result: Vec<_> = std::fs::read_dir(folder).unwrap()
        .map(|e| e.map(|d| d.path()))
        .filter_map(|f| read_note_filepath(&f.unwrap()).ok())
        .collect();
    Ok(web::Json(result))
}

async fn create_new_note(q: web::Json<NoteQuery>) -> HttpResponse {
    let nq = q.into_inner();
    if nq.content.is_none() {
        return HttpResponse::BadRequest().json("Field 'content' is missing");
    };
    let content = nq.content.unwrap();
    let title = nq.title.unwrap_or_default();
    let note = Note::with_title(content, title);
    save_note(&note);
    HttpResponse::Ok().json(note.get_uuid().to_string()) // TODO Better response messages. Maybe { http_code: 321, message: "" }
}

async fn get_note(req: HttpRequest) -> EingangResponse<Note> {
    let uuid: String = parse_uuid(req);
    let note = read_note(&uuid).unwrap();
    Ok(web::Json(note))
}

async fn delete_note(req: HttpRequest) -> HttpResponse {
    let uuid: String = parse_uuid(req);
    let file = Location::Note.create_filename(&uuid);
    match std::fs::remove_file(file) {
        Ok(_) => HttpResponse::NoContent().json("Successful"),
        _ => HttpResponse::BadRequest().json("UUID is not associated"),
    }
}

async fn update_note(req: HttpRequest, q: web::Json<NoteQuery>) -> HttpResponse {
    let uuid: String = parse_uuid(req);
    let mut note = read_note(&uuid).unwrap();
    let nq = q.into_inner();

    let mut note_changed = false;
    if let Some(c) = nq.content {
        note.content = c;
        note_changed = true;
    }
    if let Some(t) = nq.title {
        note.title = t;
        note_changed = true
    }
    if note_changed {
        note.update_modified_date();
        save_note(&note)
    }
    HttpResponse::NoContent().json("Successful")
}
