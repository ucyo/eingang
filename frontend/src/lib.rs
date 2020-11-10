use wasm_bindgen::prelude::*;
mod components;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<components::notes::Home>();
}
