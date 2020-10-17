//! # Routes: Thread Element
//!
//! In this module are all the routes for working with Thread elements.
//! It handles the routes and query parameters of these routes.
//! All of these are related to the Thread element.
//!
//! ## Threads
//! Threads
//! - `/threads`: Returns a list of all threads
//! - `/threads/new`: Create new thread
//!   - `tasks=[uuid]`: Add a vector of tasks from thread
//!   - `notes=[uuid]`: Add a vector of notes from thread
//! - `/threads/{uuid}`: Returns a specific thread
//!   - `filter=tasks`: Return only tasks from a specific thread
//!   - `filter=notes`: Return only notes from a specific thread
//! - `/threads/{uuid}/add`: Edit a specific note
//!   - `task={uuid}`: Add task to thread
//!   - `note={uuid}`: Add note to thread
//! - `/threads/{uuid}/delete`: Delete a specific note
//!   - `task={uuid}`: Delete a task from thread
//!   - `note={uuid}`: Delete a note from thread
#![allow(unused_variables, unreachable_code)]
use super::{EingangResponse, EingangVecResponse, parse_uuid};
use crate::io::{Location, read_thread, read_thread_filepath, save_thread};
use actix_web::{web, HttpRequest, HttpResponse};
use eingang::models::{Thread, ThreadFilter, ThreadQuery};
