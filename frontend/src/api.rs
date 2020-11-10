use yew::services::fetch::{Response, Request};
use yew::format::{Json, Nothing};
use yew::services::fetch::FetchTask;
use anyhow::Error;
use yew::callback::Callback;
use eingang::models::Note;
use eingang::config::backend::{HOST as BACKEND_HOST, PORT as BACKEND_PORT};

pub type FetchJsonResponse<T> = Response<Json<Result<T, Error>>>;
type FetchJsonCallback<T> = Callback<FetchJsonResponse<T>>;

pub type FetchStringResponse = Response<Result<String, Error>>;
type FetchStringCallback = Callback<FetchStringResponse>;


pub fn get_all_notes(callback: FetchJsonCallback<Vec<Note>>) -> FetchTask {
    let uri = format!("http://{}:{}/notes", BACKEND_HOST, BACKEND_PORT);
    let request = Request::get(uri)
        .body(Nothing)
        .unwrap();
    yew::services::FetchService::fetch(request, callback).unwrap()
}

pub fn delete_single_note(callback: FetchStringCallback, note_id: uuid::Uuid) -> FetchTask {
    let uri = format!("http://{}:{}/notes/{}/delete", BACKEND_HOST, BACKEND_PORT, note_id);
    let request = Request::delete(uri).body(Nothing).unwrap();
    yew::services::FetchService::fetch(request, callback).unwrap()
}
