use anyhow::Error;
use eingang::models::Note;
use eingang::models::Idable;
use yew::format::Json;
use yew::services::fetch::FetchTask;
use yew::services::storage::{Area, StorageService};
use yew::services::{ConsoleService, DialogService};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use eingang::config::frontend::KEY;
use crate::api::{FetchJsonResponse, FetchStringResponse};

struct State {
    notes: Vec<Note>,
    get_notes_loaded: bool,
    get_notes_error: Option<Error>
}

pub struct Home {
    state: State,
    link: ComponentLink<Self>,
    storage: StorageService,
    ft: Option<FetchTask>,
}

pub enum Msg {
    GetNotes,
    GetNotesSuccessful(Vec<Note>),
    GetNotesFailed(Error),
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
                let callback = self.link.callback(move | response: FetchStringResponse |{
                    let (meta, _) = response.into_parts();
                    if meta.status.is_success() {
                        Msg::DeleteNoteSuccessful(id)
                    } else {
                        Msg::DeleteNoteFailed(id)
                    }
                });
                let task = crate::api::delete_single_note(callback, note_id);
                self.ft = Some(task)
            }
            Msg::DeleteNoteSuccessful(id) => {
                let note_id = uuid::Uuid::from_u128_le(id);
                let message = format!("Note {} deleted", note_id);
                ConsoleService::info(message.as_str());

                self.ft = None;
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
                let callback = self.link.callback(move |response: FetchJsonResponse<Vec<Note>>| {
                    let (meta, Json(result)) = response.into_parts();
                    if meta.status.is_success() {
                        Msg::GetNotesSuccessful(result.ok().unwrap())
                    } else {
                        Msg::GetNotesFailed(result.err().unwrap())
                    }
                });
                let task = crate::api::get_all_notes(callback);
                self.ft = Some(task);
                self.state.get_notes_loaded = false;
                self.state.get_notes_error = None;
            }
            Msg::GetNotesSuccessful(data) => {
                ConsoleService::log("Fetching of data successful!!!");
                self.state.notes = data;
                self.ft = None;
                self.state.get_notes_loaded = true;
                self.state.get_notes_error = None;
            }
            Msg::GetNotesFailed(err) => {
                ConsoleService::log("Fetching of data failed!!!");
                self.state.get_notes_error = Some(err);
                self.ft = None;
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
        if let Some(err) = &self.state.get_notes_error {
            return html! {
                <div> {format!("Error: {}", err)} </div>
            }
        } else if !self.state.get_notes_loaded {
            return html! {
                <div>{"Loading..."}</div>
            }
        }
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
