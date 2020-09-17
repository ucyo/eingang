#![allow(dead_code)]
//! Library for the backend system of the `eingang` tool. Further, only used for
//! declaration of modules and configuration settings.

// Configuration
pub const HOST: &str = "127.0.0.1";
pub const PORT: &str = "8081";

// Modules
pub mod routes;
