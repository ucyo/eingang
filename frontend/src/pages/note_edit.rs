use yew::{ComponentLink, Properties, Component, ShouldRender};
use yew::{html, Html};
use yew::services::fetch::FetchTask;
use yew::services::storage::{Area, StorageService};
use yew::services::ConsoleService;
use yew::format::Json;
use eingang::models::{Note, Idable};
use anyhow::Error;
use crate::api::FetchJsonResponse;
use eingang::config::frontend::KEY;

pub struct SingleNoteEditPage {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
    storage: StorageService,
    task: Option<FetchTask>,
    storage_key: String,
}
#[derive(Properties, Clone)]
pub struct Props {
    pub uuid: u128,
}

struct State {
    note: Option<Note>,
    note_loaded: bool,
    note_loading_error: Option<Error>,
    preview_mode: bool,
}

pub enum Msg {
    GetNote,
    GetNoteSuccessful(Note),
    GetProductFailed(Error),
    ContentChanged(String),
    PreviewMode,
    Save,
    SaveSuccessful,
    SaveFailed(Error),
    Cancel,
}

impl Component for SingleNoteEditPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("Allocation not possible");
        let key = format!("{}.{}", KEY, props.uuid);
        let (note, note_loaded): (Option<Note>, bool) = {
            if let Json(Ok(note)) = storage.restore(key.as_str()) {
                ConsoleService::info("Restored from session");
                (Some(note), true)
            } else {
                ConsoleService::warn("Restoring did not work");
                link.send_message(Msg::GetNote);
                (None, false)
            }
        };

        Self {
            props,
            state: State {
                note,
                note_loaded,
                note_loading_error: None,
                preview_mode: false,
            },
            storage,
            link,
            task: None,
            storage_key: key,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PreviewMode => {
                yew::services::ConsoleService::warn("Preview mode needs to be implemented.");
                true
            }
            Msg::Cancel => {
                self.storage.remove(self.storage_key.as_str());
                self.link.send_message(Msg::GetNote);
                true
            }
            Msg::GetNote => {
                let callback = self
                    .link
                    .callback(|response: FetchJsonResponse<Note>| {
                        let (meta, Json(result)) = response.into_parts();
                        if meta.status.is_success() {
                            Msg::GetNoteSuccessful(result.ok().unwrap())
                        } else {
                            Msg::GetProductFailed(result.err().unwrap())
                        }
                });
                let task = crate::api::get_single_note(callback, uuid::Uuid::from_u128_le(self.props.uuid));
                self.task = Some(task);
                self.state.note = None;
                self.state.note_loaded = false;
                true
            }
            Msg::GetNoteSuccessful(note) => {
                self.state.note = Some(note);
                self.state.note_loaded = true;
                true
            }
            Msg::GetProductFailed(err) => {
                self.state.note_loading_error = Some(err);
                self.state.note_loaded = true;
                true
            }
            Msg::ContentChanged(content) => {
                let msg = format!("Registered Keypress: {:?}", content);
                ConsoleService::warn(msg.as_str());
                if let Some(ref mut note) = self.state.note {
                    note.content = content;
                }
                self.storage.store(self.storage_key.as_str(), Json(&self.state.note));
                // TODO Additionally safe in Session Storage in case the browser window is closed
                // TODO Try loading from session storage first
                true
            }
            Msg::SaveSuccessful => {
                yew::services::ConsoleService::warn("Successful save needs to be implemented.");
                true
            }
            Msg::Save => {
                yew::services::ConsoleService::warn("Save needs to be implemented.");
                true
            }
            Msg::SaveFailed(_err) => {
                yew::services::ConsoleService::warn("Error Note save needs to be implemented.");
                true
            }
        }
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        if let Some(ref err) = self.state.note_loading_error {
            html! {
                <div>
                    <p> {format!("Error: {}", err)} </p>
                </div>
            }
        } else if !self.state.note_loaded {
            html! {
                <div>
                    <p> {"Loading..."} </p>
                </div>
            }
        } else if let Some(ref note) = self.state.note {
            html! {
                <div>
                    <p> {&note.get_uuid()} </p>
                    <button onclick=self.link.callback(move |_| Msg::PreviewMode) type="submit">{ "Preview" }</button>
                    <textarea oninput=self.link.callback(move |v: yew::InputData| Msg::ContentChanged(v.value)) value=note.content/>
                    <button onclick=self.link.callback(move |_| Msg::Save) type="submit">{ "Save" }</button>
                    <button onclick=self.link.callback(move |_| Msg::Cancel) type="submit">{ "Cancel" }</button>
                    </div>
            }
        } else {
            html!{
                <div><p>{"Unknown Error"}</p></div>
            }
        }
    }
}
