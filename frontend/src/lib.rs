#![recursion_limit = "256"]
use wasm_bindgen::prelude::*;
use yew::format::Json;
use yew::services::storage::{Area, StorageService};
use yew::services::{ConsoleService, DialogService};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use eingang::models::Data;

const KEY: &str = "eingang.model.store";

struct Model {
    link: ComponentLink<Self>,
    storage: StorageService,
    value: Data,
}

enum Msg {
    AddOne,
    SubtractOne,
    SetValue,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Save data to localStorage (persistent across sessions)
        // Area::Session gets deleted after the tab or window is closed
        // details: https://stackoverflow.com/questions/19867599/what-is-the-difference-between-localstorage-sessionstorage-session-and-cookies
        let storage = StorageService::new(Area::Local).expect("Alocation error");
        let value = {
            if let Json(Ok(val)) = storage.restore(KEY) {
                ConsoleService::log("Restored!");
                val
            } else {
                ConsoleService::log("Failed to restore!");
                0
            }
        };
        Self {
            link,
            storage,
            value: Data::new(value, 0usize),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                ConsoleService::log("Increment")
            }
            Msg::SubtractOne => {
                self.value -= 1;
                ConsoleService::log("Decrement")
            }
            Msg::SetValue => {
                let current = self.value.to_string();
                let input = DialogService::prompt("Set value to?", Some(current.as_str()))
                    .unwrap()
                    .parse::<i64>();
                match input {
                    Ok(value) => {
                        if value == self.value {
                            return false;
                        }
                        let msg = format!("Do you want to change the value to {}?", value);
                        let confirmed = DialogService::confirm(msg.as_str());
                        if confirmed {
                            let msg = format!("Changed {} to {}.", self.value, value);
                            self.value.update(value);
                            DialogService::alert(msg.as_str());
                            ConsoleService::log(msg.as_str())
                        } else {
                            DialogService::alert("Did not change value.")
                        }
                    }
                    Err(_) => ConsoleService::log("Can not parse number"),
                }
            }
        }
        self.storage.store(KEY, Json(&self.value));
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{ self.value }</h1>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <button onclick=self.link.callback(|_| Msg::SubtractOne)>{ "-1" }</button>
                <button onclick=self.link.callback(|_| Msg::SetValue)>{ "Set value" }</button>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<Model>();
}
