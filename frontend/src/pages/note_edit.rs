use yew::{ComponentLink, Properties, Component, ShouldRender};
use yew::{html, Html};
use yew::services::fetch::FetchTask;
use yew::format::Json;
use eingang::models::{Note, Idable};
use anyhow::Error;
use crate::api::FetchJsonResponse;

pub struct SingleNoteEditPage {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}
#[derive(Properties, Clone)]
pub struct Props {
    pub uuid: u128,
}

struct State {
    note: Option<Note>,
    note_loaded: bool,
    note_loading_error: Option<Error>
}

pub enum Msg {
    GetNote,
    GetNoteSuccessful(Note),
    GetProductFailed(Error),
    ContentChanged(yew::KeyboardEvent),
    SaveSuccessful,
    SaveFailed(Error),
}

impl Component for SingleNoteEditPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetNote);

        Self {
            props,
            state: State {
                note: None,
                note_loaded: false,
                note_loading_error: None
            },
            link,
            task: None,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
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
                let key = content.key();
                let msg = format!("Registered Keypress: {}", key);
                yew::services::ConsoleService::warn(msg.as_str());
                true
            }
            Msg::SaveSuccessful => {
                yew::services::ConsoleService::warn("Successful save needs to be implemented.");
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
                    <textarea onkeypress=self.link.callback(move |key| Msg::ContentChanged(key))
                     rows=10 cols=50> {&note.content} </textarea>
                </div>
            }
        } else {
            html!{
                <div><p>{"Unknown Error"}</p></div>
            }
        }
    }
}
