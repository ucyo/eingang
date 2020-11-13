use yew_router::Switch;

#[derive(Switch, Debug, Clone)]
pub enum Route {
    #[to = "/notes/{uuid}/edit"]
    NoteEditPage(u128),

    #[to = "/notes/{uuid}"]
    Note(u128),

    #[to = "/"]
    NotesPage,
}
