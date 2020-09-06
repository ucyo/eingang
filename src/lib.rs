#![recursion_limit="256"]
use wasm_bindgen::prelude::*;
use yew::services::{ConsoleService, DialogService};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
    SubtractOne,
    ShowAlertDialog,
    ShowConfirmationDialog,
    ShowPromptDialog,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
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
            Msg::ShowAlertDialog => {
                DialogService::alert("Watch out dog!!")
            }
            Msg::ShowConfirmationDialog => {
                let accepted = DialogService::confirm("Yes or no?");
                if accepted {
                    ConsoleService::log("Yes!!");
                } else {
                    ConsoleService::log("Nooooo!!")
                }
            }
            Msg::ShowPromptDialog => {
                let current = self.value.to_string();
                match DialogService::prompt("Set value to?", Some(current.as_str())).unwrap().parse::<i64>() {
                    Ok(value) => {
                        self.value = value;
                        ConsoleService::log(format!("Set value to {}", value).as_str())
                    }
                    Err(_) => ConsoleService::log("Can not parse number")
                }
            }
        }
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
                <button onclick=self.link.callback(|_| Msg::ShowAlertDialog)>{ "Dialog please" }</button>
                <button onclick=self.link.callback(|_| Msg::ShowConfirmationDialog)>{ "Confirmation please" }</button>
                <button onclick=self.link.callback(|_| Msg::ShowPromptDialog)>{ "Set value" }</button>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<Model>();
}
