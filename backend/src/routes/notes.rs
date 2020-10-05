#![allow(unused_variables, unreachable_code)]
use eingang::models::Note;
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
    HttpResponse::Ok()
}

#[get("/notes/{uuid}")]
async fn get_note(req: HttpRequest) -> EingangResponse<Note> {
    unimplemented!()
}

#[delete("/notes/{uuid}/delete")]
async fn delete_note(req: HttpRequest) -> impl Responder {
    unimplemented!();
    HttpResponse::Ok()
}

#[patch("/notes/{uuid}/update")]
async fn update_note(req: HttpRequest) -> impl Responder {
    unimplemented!();
    HttpResponse::Ok()
}
