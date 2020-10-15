//! # Routes: Thread Element
//!
//! In this module are all the routes for working with Thread elements.
//! It handles the routes and query parameters of these routes.
//! All of these are related to the Thread element.
//!
//! ## Threads
//! Threads
//! - `/threads`: Returns a list of all threads
//! - `/threads/{uuid}`: Returns a specific thread
//!   - `filter=tasks`: Return only tasks from a specific thread
//!   - `filter=notes`: Return only notes from a specific thread
//! - `/threads/{uuid}/update`: Edit a specific note
//! - `/threads/{uuid}/delete`: Delete a specific note
//! - `/threads/new`: Create new thread
//!
//! Auto-Threads
//! - `/journal`: Entries within a certain time period
//! - `/graph`: Graph view of all threads in the system
