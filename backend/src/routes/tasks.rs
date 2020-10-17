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
//!   - if `status` is nonsense, it will be ignored
//! - `/tasks/{uuid}`: Return a specific task
//! - `/tasks/{uuid}/update`: Edit a specific task
//! - `/tasks/{uuid}/delete`: Delete a specific task
//! - `/tasks/new`: Create new task
use super::{EingangResponse, EingangVecResponse,parse_uuid};
use actix_web::{web, HttpRequest, HttpResponse};
use eingang::models::{Task, TaskQuery, TaskStatus, Idable};
use crate::io::{Location, read_task, read_task_filepath, save_task};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/tasks").route(web::get().to(get_all_tasks)));
    cfg.service(web::resource("/tasks/new").route(web::post().to(create_new_task)));
    cfg.service(web::resource("/tasks/{uuid}").route(web::get().to(get_task)));
    cfg.service(web::resource("/tasks/{uuid}/delete").route(web::delete().to(delete_task)));
    cfg.service(web::resource("/tasks/{uuid}/update").route(web::patch().to(update_task)));
}

async fn get_all_tasks(_: HttpRequest, q: web::Query<TaskQuery>) -> EingangVecResponse<Task> {
    let (do_filter, filter) = match q.into_inner().status {
        Some(s) => {
            match TaskStatus::from(s) {
                Some(c) => (true, c),
                _ => (false, TaskStatus::default())
            }
        },
        _ => (false, TaskStatus::default()),
    };
    let folder = Location::Task.get_basefolder();
    let temp = std::fs::read_dir(folder)
        .unwrap()
        .map(|e| e.map(|d| d.path()))
        .map(|f| read_task_filepath(&f.unwrap()).unwrap());

    if do_filter {
        Ok(web::Json(temp.filter(|k| k.status == filter).collect()))
    } else {
        Ok(web::Json(temp.collect()))
    }
}

async fn create_new_task(q: web::Json<TaskQuery>) -> HttpResponse {
    let tq = q.into_inner();
    if tq.content.is_none() {
        return HttpResponse::BadRequest().json("Field 'content' is missing");
    };
    // TODO Write a better matching, maybe with list of accepted values
    let mut status = TaskStatus::default();
    if tq.status.is_some() {
        let stst = tq.status.unwrap();
        match TaskStatus::from(stst) {
            Some(c) => status = c,
            _ => return HttpResponse::BadRequest().json("Unknown status")
        }
    };
    let content = tq.content.unwrap();
    let title = tq.title.unwrap_or_default();
    let task = Task::with_title_and_status(content, title, status);
    save_task(&task);
    HttpResponse::Ok().json(task.get_uuid().to_string())
}

async fn get_task(req: HttpRequest) -> EingangResponse<Task> {
    let uuid: String = parse_uuid(req);
    let task = read_task(&uuid).unwrap();
    Ok(web::Json(task))
}

async fn delete_task(req: HttpRequest) -> HttpResponse {
    let uuid: String = parse_uuid(req);
    let file = Location::Task.create_filename(&uuid);
    match std::fs::remove_file(file) {
        Ok(_) => HttpResponse::NoContent().json("Successful"),
        _ => HttpResponse::BadRequest().json("UUID is not associated"),
    }
}

async fn update_task(req: HttpRequest, q: web::Json<TaskQuery>) -> HttpResponse {
    let uuid: String = parse_uuid(req);
    let mut task = read_task(&uuid).unwrap();
    let tq = q.into_inner();

    let mut task_changed = false;
    if let Some(c) = tq.content {
        task.content = c;
        task_changed = true;
    }
    if let Some(t) = tq.title {
        task.title = t;
        task_changed = true;
    }
    if let Some(s) = tq.status {
        match TaskStatus::from(s) {
            Some(c) => task.status = c,
            _ => return HttpResponse::BadRequest().json("Unknown status")
        }
        task_changed = true;
    }

    if task_changed {
        task.update_modified_date();
        save_task(&task);
    }
    HttpResponse::NoContent().json("Successful")
}
