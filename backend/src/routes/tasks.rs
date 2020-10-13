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
use actix_web::{web, HttpRequest, HttpResponse};
use eingang::models::{Task, TaskQuery};
use super::{EingangVecResponse, EingangResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/tasks").route(web::get().to(get_all_tasks)));
    cfg.service(web::resource("/tasks/new").route(web::post().to(create_new_task)));
    cfg.service(web::resource("/tasks/{uuid}").route(web::get().to(get_task)));
    cfg.service(web::resource("/tasks/{uuid}/delete").route(web::delete().to(delete_task)));
    cfg.service(web::resource("/tasks/{uuid}/update").route(web::patch().to(update_task)));
}

async fn get_all_tasks(q: web::Json<TaskQuery>) -> EingangVecResponse<Task> {
    unimplemented!()
}

async fn create_new_task(q: web::Json<TaskQuery>) -> HttpResponse {
    unimplemented!()
}

async fn get_task(req: HttpRequest) -> EingangResponse<Task> {
    unimplemented!()
}

async fn delete_task(req: HttpRequest) -> HttpResponse {
    unimplemented!()
}

async fn update_task(req: HttpRequest, q: web::Json<TaskQuery>) -> HttpResponse {
    unimplemented!()
}
