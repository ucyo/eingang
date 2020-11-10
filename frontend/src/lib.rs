use wasm_bindgen::prelude::*;
mod pages;
mod api;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<pages::notes::Home>();
}
