#![recursion_limit = "256"]
use anyhow::Error;
use eingang::models::Data;
use wasm_bindgen::prelude::*;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::storage::{Area, StorageService};
use yew::services::{ConsoleService, DialogService};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

const KEY: &str = "eingang.model.store";

type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type SendResponse = Response<Result<String, Error>>;

struct Model {
    link: ComponentLink<Self>,
    storage: StorageService,
    value: Data,
    ft: Option<FetchTask>, // currently active FetchTask is saved here
    st: Option<FetchTask>,
}

enum Msg {
    AddOne,
    // SubtractOne,
    // SetValue,
    FetchStart,
    FetchSuccess(Data),
    FetchFail,
    SendStart,
    SendSuccess,
    SendFailed,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Save data to localStorage (persistent across sessions)
        // Area::Session gets deleted after the tab or window is closed
        // details: https://stackoverflow.com/questions/19867599/what-is-the-difference-between-localstorage-sessionstorage-session-and-cookies
        let storage = StorageService::new(Area::Local).expect("Allocation error");
        let value = {
            if let Json(Ok(val)) = storage.restore(KEY) {
                ConsoleService::log("Restored!");
                val
            } else {
                ConsoleService::log("Failed to restore!");
                Default::default()
            }
        };
        Self {
            link,
            storage,
            value,
            ft: None,
            st: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SendStart => {
                let callback = self.link.callback(move |response: SendResponse| {
                    let (meta, _) = response.into_parts();
                    if meta.status.is_success() {
                        Msg::SendSuccess
                    } else {
                        Msg::SendFailed
                    }
                });
                let request = Request::post("http://localhost:8081/save")
                    .header("Content-Type", "application/json")
                    .body(Json(&self.value))
                    .unwrap();
                let task = yew::services::FetchService::fetch(request, callback).unwrap();
                self.st = Some(task);
            }
            Msg::SendSuccess => {
                ConsoleService::log("Saved data!");
                self.st = None
            }
            Msg::SendFailed => {
                ConsoleService::log("Could not save data!");
                DialogService::alert("Could not save data!");
                self.st = None
            }
            Msg::FetchStart => {
                // set up what to do if the FetchResponse finishes
                let callback = self.link.callback(move |response: FetchResponse<Data>| {
                    let (meta, Json(result)) = response.into_parts();
                    if meta.status.is_success() {
                        Msg::FetchSuccess(result.ok().unwrap())
                    } else {
                        Msg::FetchFail
                    }
                });

                // actual request body
                let request = Request::get("http://localhost:8081/load")
                    .body(Nothing)
                    .unwrap();

                // Setting out the request
                let task = yew::services::FetchService::fetch(request, callback).unwrap();

                // Saving the request on the model
                self.ft = Some(task)
            }
            Msg::FetchSuccess(data) => {
                self.value = data;
                self.ft = None
            }
            Msg::FetchFail => {
                ConsoleService::log("Fetching of data failed!!!");
                self.ft = None
            }
            Msg::AddOne => {
                self.value += 1;
                ConsoleService::log("Increment")
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
                <button onclick=self.link.callback(|_| Msg::FetchStart)>{ "Load" }</button>
                <button onclick=self.link.callback(|_| Msg::SendStart)>{ "Save" }</button>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<Model>();
}
