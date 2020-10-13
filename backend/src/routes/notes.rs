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
#![allow(unused_variables, unreachable_code)]
use actix_web::{web, HttpRequest, HttpResponse, Result};
use eingang::models::{Note, NoteQuery};

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
async fn get_all_notes(req: HttpRequest) -> EingangVecResponse<Note> {
    let folder = Path::new(BASE_FOLDER).join(NOTE_FOLDER);
    let temp: Vec<_> = std::fs::read_dir(folder)
        .unwrap()
        .map(|e| e.map(|d| d.path()))
        .collect();
    let result: Vec<Note> = temp
        .into_iter()
        .map(|f| read_note_filepath(&f.unwrap()))
        .collect();
    Ok(web::Json(result))
}

async fn create_new_note(q: web::Json<NoteQuery>) -> HttpResponse {
    let nq = q.into_inner();
    if let None = nq.content {
        return HttpResponse::BadRequest().json("Field 'content' is missing");
    };
    let content = nq.content.unwrap();
    let title = nq.title.unwrap_or_default();
    let note = Note::with_title(content, title);
    save_note(&note);
    HttpResponse::Ok().json(note.meta.uuid) // TODO Better response messages. Maybe { http_code: 321, message: "" }
}

async fn get_note(req: HttpRequest) -> EingangResponse<Note> {
    let uuid: String = parse_uuid(req);
    let note = read_note(uuid);
    Ok(web::Json(note))
}

async fn delete_note(req: HttpRequest) -> HttpResponse {
    let uuid: String = parse_uuid(req);
    let file = create_filepath(uuid);
    match std::fs::remove_file(file) {
        Ok(_) => HttpResponse::NoContent().json("Successful"),
        _ => HttpResponse::BadRequest().json("UUID is not associated"),
    }
}

async fn update_note(req: HttpRequest, q: web::Json<NoteQuery>) -> HttpResponse {
    let uuid: String = parse_uuid(req);
    let mut note = read_note(uuid);
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
        note.meta.update_modified_date();
        save_note(&note)
    }
    HttpResponse::NoContent().json("Successful")
}

use crate::{BASE_FOLDER, NOTE_FOLDER};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

fn create_filepath(uuid: String) -> PathBuf {
    let basename = format!("{}.json", uuid);
    Path::new(BASE_FOLDER).join(NOTE_FOLDER).join(basename)
}

fn save_note(note: &Note) {
    let file = create_filepath(note.meta.uuid.to_string());
    let buffer = File::create(file).unwrap();
    let mut writer = std::io::BufWriter::new(buffer);
    let _ = serde_json::to_writer_pretty(&mut writer, &note).unwrap();
    writer.flush().unwrap();
}

fn read_note_filepath(file: &PathBuf) -> Note {
    let buffer = File::open(file).unwrap();
    let rdr = std::io::BufReader::new(buffer);
    let note: Note = serde_json::from_reader(rdr).unwrap();
    note
}

fn read_note(uuid: String) -> Note {
    let file = create_filepath(uuid);
    read_note_filepath(&file)
}

fn parse_uuid(req: HttpRequest) -> String {
    req.match_info()
        .get("uuid")
        .unwrap() // TODO Better parsing, since this could panic
        .parse()
        .unwrap()
}
