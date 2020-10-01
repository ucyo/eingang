#![allow(dead_code)]
//! Library for the backend system of the `eingang` tool. Further, only used for
//! declaration of modules and configuration settings.

// Configuration
pub const HOST: &str = "127.0.0.1";
pub const PORT: &str = "8081";
pub const STORAGE: &str = "/tmp/db.json";
pub const FRONTEND_HOST: &str = "http://localhost";
pub const FRONTEND_PORT: &str = "8080";

// Modules
pub mod routes;
