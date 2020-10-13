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
use actix_web::{web, HttpRequest, HttpResponse, Result};
use eingang::models::{Task, TaskQuery};