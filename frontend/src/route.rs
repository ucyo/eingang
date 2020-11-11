use yew_router::Switch;
#[derive(Switch, Debug, Clone)]
pub enum Route {
    #[to = "/"]
    NotesPage,
}
