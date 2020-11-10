use anyhow::Error;
use eingang::models::Note;
use eingang::models::Idable;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::storage::{Area, StorageService};
use yew::services::{ConsoleService, DialogService};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use eingang::config::backend::{HOST as BACKEND_HOST, PORT as BACKEND_PORT};
use eingang::config::frontend::KEY;

type FetchResponse<T> = Response<Json<Result<T, Error>>>;

struct State {
    notes: Vec<Note>,
    get_notes_loaded: bool,
    get_notes_error: Option<Error>
}

pub struct Home {
    state: State,
    link: ComponentLink<Self>,
    storage: StorageService,
    ft: Option<FetchTask>,  // currently active FetchTask is saved here
}

pub enum Msg {
    GetNotes,
    GetNotesSuccessful(Vec<Note>),
    GetNotesFailed,
    DeleteNote(u128),
    DeleteNoteSuccessful(u128),
    DeleteNoteFailed(u128),
    CreateNote,
    ViewNote(u128),
    EditNote(u128),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Save data to localStorage (persistent across sessions)
        // Area::Session gets deleted after the tab or window is closed
        // details: https://stackoverflow.com/questions/19867599/what-is-the-difference-between-localstorage-sessionstorage-session-and-cookies
        let storage = StorageService::new(Area::Local).expect("Allocation error");
        let products = {
            if let Json(Ok(val)) = storage.restore(KEY) {
                ConsoleService::log("Restored!");
                val
            } else {
                ConsoleService::log("Failed to restore!");
                Default::default()
            }
        };
        let state = State {
            notes: products,
            get_notes_loaded: false,
            get_notes_error: None,
        };
        link.send_message(Msg::GetNotes);
        Self {
            state,
            link,
            storage,
            ft: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DeleteNote(id) => {
                let note_id = uuid::Uuid::from_u128_le(id);
                let msg = format!("Do you really wanna delete: {}", note_id);
                let confirmed = DialogService::confirm(msg.as_str());
                if !confirmed {
                    let message = format!("Aborting deletion of {}", note_id);
                    ConsoleService::info(message.as_str());
                    return false
                }
                let  callback = self.link.callback(move | response: Response<Result<String, Error>>|{
                    let (meta, _) = response.into_parts();
                    if meta.status.is_success() {
                        Msg::DeleteNoteSuccessful(id)
                    } else {
                        Msg::DeleteNoteFailed(id)
                    }
                });
                let uri = format!("http://{}:{}/notes/{}/delete", BACKEND_HOST, BACKEND_PORT, note_id);
                let request = Request::delete(uri).body(Nothing).unwrap();
                let task = yew::services::FetchService::fetch(request, callback).unwrap();
                self.ft = Some(task)
            }
            Msg::DeleteNoteSuccessful(id) => {
                let note_id = uuid::Uuid::from_u128_le(id);
                let message = format!("Note {} deleted", note_id);
                self.ft = None;
                ConsoleService::info(message.as_str());
                // TODO Since Calls operate on IO level, the self object needs to be updated from disk
                self.link.send_message(Msg::GetNotes);
            }
            Msg::DeleteNoteFailed(id) => {
                let note_id = uuid::Uuid::from_u128_le(id);
                let message = format!("Deleting Note {} failed", note_id);
                ConsoleService::info(message.as_str());
                self.ft = None;
            }
            Msg::EditNote(id) => {
                let obj = uuid::Uuid::from_u128_le(id);
                let message = format!("Edit: {}", obj);
                ConsoleService::info(message.as_str())
            }
            Msg::ViewNote(id) => {
                let obj = uuid::Uuid::from_u128_le(id);
                let message = format!("View: {}", obj);
                ConsoleService::info(message.as_str())
            }
            Msg::CreateNote => {
                let message = format!("Creating a new Note");
                ConsoleService::info(message.as_str())
            }
            Msg::GetNotes => {
                // set up what to do if the FetchResponse finishes
                let callback = self.link.callback(move |response: FetchResponse<Vec<Note>>| {
                    let (meta, Json(result)) = response.into_parts();
                    if meta.status.is_success() {
                        Msg::GetNotesSuccessful(result.ok().unwrap())
                    } else {
                        Msg::GetNotesFailed
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
            Msg::GetNotesSuccessful(data) => {
                ConsoleService::log("Fetching of data successful!!!");
                self.state.notes = data;
                self.ft = None
            }
            Msg::GetNotesFailed => {
                ConsoleService::log("Fetching of data failed!!!");
                self.ft = None
            }
        }
        self.storage.store(KEY, Json(&self.state.notes));
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let notes: Vec<Html> = self.state.notes.iter().map(|note: &Note| {
        let id = note.get_uuid().to_u128_le();
                html! {
                <div>
                <p>{&note.get_uuid()}{":"}</p>
                <p>{&note}</p>
                <button onclick=self.link.callback(move |_| Msg::ViewNote(id)) type="submit">{ "View" }</button>
                <button onclick=self.link.callback(move |_| Msg::EditNote(id)) type="submit">{ "Edit" }</button>
                <button onclick=self.link.callback(move |_| Msg::DeleteNote(id)) type="submit">{ "Delete" }</button>
            </div>
            }
        })
        .collect();
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::GetNotes) type="submit">{ "Load Notes" }</button>
                <button onclick=self.link.callback(|_| Msg::CreateNote) type="submit">{ "Create Note" }</button>
                <span>{notes}</span>
            </div>
        }
    }
}
