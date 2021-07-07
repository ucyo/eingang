#![recursion_limit = "256"]
use wasm_bindgen::prelude::*;
mod api;
mod app;
mod pages;
mod route;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<app::App>();
}
