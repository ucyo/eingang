#![allow(unused_variables, unreachable_code)]
use eingang::models::{Note, NoteQuery};
use actix_web::{get, post, delete, patch};
use actix_web::{HttpRequest, HttpResponse, Responder, Result, web};

type EingangVecResponse<T> = Result<web::Json<Vec<T>>>;
type EingangResponse<T> = Result<web::Json<T>>;


#[get("/notes")]
async fn get_all_notes(req: HttpRequest) -> EingangVecResponse<Note> {
    unimplemented!()
}

#[post("/notes/new")]
async fn create_new_note(note: web::Json<Note>) -> impl Responder {
    unimplemented!();
    HttpResponse::NoContent()
}

#[get("/notes/{uuid}")]
async fn get_note(req: HttpRequest) -> EingangResponse<Note> {
    unimplemented!()
}

#[delete("/notes/{uuid}/delete")]
async fn delete_note(req: HttpRequest) -> impl Responder {
    unimplemented!();
    HttpResponse::NoContent()
}

#[patch("/notes/{uuid}/update")]
async fn update_note(req: HttpRequest) -> impl Responder {
    unimplemented!();
    HttpResponse::NoContent()
}

use crate::{BASE_FOLDER, NOTE_FOLDER};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Write;

fn create_filepath(uuid: String) -> PathBuf {
    let basename = format!("{}.json", uuid);
    Path::new(BASE_FOLDER)
        .join(NOTE_FOLDER)
        .join(basename)
}

fn save_note(note: Note) {
    let file = create_filepath(note.meta.uuid.to_string());
    let buffer = File::create(file).unwrap();
    let mut writer = std::io::BufWriter::new(buffer);
    let _ = serde_json::to_writer_pretty(&mut writer, &note).unwrap();
    writer.flush().unwrap();
}

fn read_note_filepath(file: PathBuf) -> Note {
    let buffer = File::open(file).unwrap();
    let rdr = std::io::BufReader::new(buffer);
    let note: Note = serde_json::from_reader(rdr).unwrap();
    note
}

fn read_note(uuid: String) -> Note {
    let file = create_filepath(uuid);
    read_note_filepath(file)
}
