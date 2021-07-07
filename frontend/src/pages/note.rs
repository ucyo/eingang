use crate::api::FetchJsonResponse;
use anyhow::Error;
use eingang::models::{Idable, Note};
use yew::format::Json;
use yew::services::fetch::FetchTask;
use yew::{html, Html};
use yew::{Component, ComponentLink, Properties, ShouldRender};

pub struct SingleNotePage {
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
    note_loading_error: Option<Error>,
}

pub enum Msg {
    GetNote,
    GetNoteSuccessful(Note),
    GetProductFailed(Error),
}

impl Component for SingleNotePage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetNote);

        Self {
            props,
            state: State {
                note: None,
                note_loaded: false,
                note_loading_error: None,
            },
            link,
            task: None,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetNote => {
                let callback = self.link.callback(|response: FetchJsonResponse<Note>| {
                    let (meta, Json(result)) = response.into_parts();
                    if meta.status.is_success() {
                        Msg::GetNoteSuccessful(result.ok().unwrap())
                    } else {
                        Msg::GetProductFailed(result.err().unwrap())
                    }
                });
                let task = crate::api::get_single_note(
                    callback,
                    uuid::Uuid::from_u128_le(self.props.uuid),
                );
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
                    <p> {&note} </p>
                </div>
            }
        } else {
            html! {
                <div><p>{"Unknown Error"}</p></div>
            }
        }
    }
}
