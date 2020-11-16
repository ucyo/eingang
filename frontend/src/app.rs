use crate::pages::{NotesPage, SingleNoteEditPage, SingleNotePage};
use crate::route::Route;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::router::Router;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render = Router::render(|switch| match switch {
            Route::Note(uuid) => html! {<SingleNotePage uuid=uuid/>},
            Route::NotesPage => html! {<NotesPage/>},
            Route::NoteEditPage(uuid) => html! {<SingleNoteEditPage uuid=uuid/>},
        });

        html! {
            <Router<Route, ()> render=render/>
        }
    }
}
