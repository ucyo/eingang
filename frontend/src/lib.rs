use wasm_bindgen::prelude::*;
mod pages;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<pages::notes::Notes>();
}
