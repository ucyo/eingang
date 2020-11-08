#![recursion_limit = "256"]
use anyhow::Error;
use eingang::models::Note;
use eingang::models::Idable;
use wasm_bindgen::prelude::*;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::storage::{Area, StorageService};
use yew::services::{ConsoleService, DialogService};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use eingang::config::backend::{HOST as BACKEND_HOST, PORT as BACKEND_PORT};

use eingang::config::frontend::KEY;

type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type SendResponse = Response<Result<String, Error>>;

struct Model {
    link: ComponentLink<Self>,
    storage: StorageService,
    value: Vec<Note>,
    ft: Option<FetchTask>, // currently active FetchTask is saved here
    st: Option<FetchTask>,
}

enum Msg {
    FetchStart,
    FetchSuccess(Vec<Note>),
    FetchFail,
    StartDelete(u128),
    StartEdit(u128),
    StartView(u128),
    SendStart,
    SendSuccess,
    SendFailed,
    CreateNote,
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
            Msg::StartDelete(id) => {
                let note_id = uuid::Uuid::from_u128_le(id);
                let msg = format!("Do you really wanna delete: {}", note_id);
                let confirmed = DialogService::confirm(msg.as_str());
                if confirmed {
                    let message = format!("Deleting: {}", note_id);
                    ConsoleService::info(message.as_str());
                } else {
                    let message = format!("Aborting deletion of {}", note_id);
                    ConsoleService::info(message.as_str())
                }
            }
            Msg::StartEdit(id) => {
                let obj = uuid::Uuid::from_u128_le(id);
                let message = format!("Edit: {}", obj);
                ConsoleService::info(message.as_str())
            }
            Msg::StartView(id) => {
                let obj = uuid::Uuid::from_u128_le(id);
                let message = format!("View: {}", obj);
                ConsoleService::info(message.as_str())
            }
            Msg::CreateNote => {
                let message = format!("Creating a new Note");
                ConsoleService::info(message.as_str())
            }
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
                let callback = self.link.callback(move |response: FetchResponse<Vec<Note>>| {
                    let (meta, Json(result)) = response.into_parts();
                    if meta.status.is_success() {
                        Msg::FetchSuccess(result.ok().unwrap())
                    } else {
                        Msg::FetchFail
                    }
                });

                // actual request body
                let uri = format!("http://{}:{}/notes", BACKEND_HOST, BACKEND_PORT);
                let request = Request::get(uri)
                    .body(Nothing)
                    .unwrap();

                // Setting out the request
                let task = yew::services::FetchService::fetch(request, callback).unwrap();

                // Saving the request on the model
                self.ft = Some(task)
            }
            Msg::FetchSuccess(data) => {
                ConsoleService::log("Fetching of data successful!!!");
                self.value = data;
                self.ft = None
            }
            Msg::FetchFail => {
                ConsoleService::log("Fetching of data failed!!!");
                self.ft = None
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
    let notes: Vec<Html> = self.value.iter().map(|note: &Note| {
            let id = note.get_uuid().to_u128_le();
                 html! {
                  <div>
                    <p>{&note.get_uuid()}{":"}</p>
                    <p>{&note}</p>
                    <button onclick=self.link.callback(move |_| Msg::StartView(id)) type="submit">{ "View" }</button>
                    <button onclick=self.link.callback(move |_| Msg::StartEdit(id)) type="submit">{ "Edit" }</button>
                    <button onclick=self.link.callback(move |_| Msg::StartDelete(id)) type="submit">{ "Delete" }</button>
                </div>
                }
            })
            .collect();
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::FetchStart) type="submit">{ "Load Notes" }</button>
                <button onclick=self.link.callback(|_| Msg::CreateNote) type="submit">{ "Create Note" }</button>
                <span>{notes}</span>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<Model>();
}
