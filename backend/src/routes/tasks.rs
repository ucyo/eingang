//! # Routes: Task Element
//!
//! In this module are all the routes for working with Task elements.
//! It handles the routes and query parameters of these routes.
//! All of these are related to the Task element.
//!
//! ## Tasks
//! - `/tasks`: Return all tasks in the database
//!   - `status=done`: Return all done tasks
//!   - `status=open`: Return all open tasks
//!   - `status=waiting`: Return all waiting tasks
//! - `/tasks/{uuid}`: Return a specific task
//! - `/tasks/{uuid}/update`: Edit a specific task
//! - `/tasks/{uuid}/delete`: Delete a specific task
//! - `/tasks/new`: Create new task
#![allow(unused_variables, unreachable_code)]
use super::{EingangResponse, EingangVecResponse,parse_uuid};
use actix_web::{web, HttpRequest, HttpResponse};
use eingang::models::{Task, TaskQuery, TaskStatus};
use crate::io::{Location, read_task, read_task_filepath, save_task};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/tasks").route(web::get().to(get_all_tasks)));
    cfg.service(web::resource("/tasks/new").route(web::post().to(create_new_task)));
    cfg.service(web::resource("/tasks/{uuid}").route(web::get().to(get_task)));
    cfg.service(web::resource("/tasks/{uuid}/delete").route(web::delete().to(delete_task)));
    cfg.service(web::resource("/tasks/{uuid}/update").route(web::patch().to(update_task)));
}

async fn get_all_tasks(req: HttpRequest) -> EingangVecResponse<Task> {
    let folder = Location::Task.get_basefolder();
    let temp: Vec<_> = std::fs::read_dir(folder)
        .unwrap()
        .map(|e| e.map(|d| d.path()))
        .collect();
    let result: Vec<Task> = temp
        .into_iter()
        .map(|f| read_task_filepath(&f.unwrap()))
        .collect();  // TODO Add filter based on q
    Ok(web::Json(result))
}

async fn create_new_task(q: web::Json<TaskQuery>) -> HttpResponse {
    let tq = q.into_inner();
    if let None = tq.content {
        return HttpResponse::BadRequest().json("Field 'content' is missing");
    };
    // TODO Write a better matching, maybe with list of accepted values
    let mut status = TaskStatus::default();
    if let Some(stst) = tq.status {
        status = TaskStatus::from(stst)
    };
    let content = tq.content.unwrap();
    let title = tq.title.unwrap_or_default();
    let task = Task::with_title_and_status(content, title, status);
    save_task(&task);
    HttpResponse::Ok().json(task.meta.uuid)
}

async fn get_task(req: HttpRequest) -> EingangResponse<Task> {
    let uuid: String = parse_uuid(req);
    let task = read_task(uuid);
    Ok(web::Json(task))
}

async fn delete_task(req: HttpRequest) -> HttpResponse {
    unimplemented!()
}

async fn update_task(req: HttpRequest, q: web::Json<TaskQuery>) -> HttpResponse {
    unimplemented!()
}
