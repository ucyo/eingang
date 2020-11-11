use crate::pages::NotesPage;
use crate::route::Route;
use yew::{Component, ComponentLink, ShouldRender, Html, html};
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
        let render = Router::render(|switch|
            match switch {
                Route::NotesPage => html!{<NotesPage/>}
            }
        );

        html! {
            <Router<Route, ()> render=render/>
        }
    }
}
