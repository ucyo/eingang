use anyhow::Error;
use eingang::config::backend::{HOST as BACKEND_HOST, PORT as BACKEND_PORT};
use eingang::models::Note;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::FetchTask;
use yew::services::fetch::{Request, Response};

pub type FetchJsonResponse<T> = Response<Json<Result<T, Error>>>;
type FetchJsonCallback<T> = Callback<FetchJsonResponse<T>>;

pub type FetchStringResponse = Response<Result<String, Error>>;
type FetchStringCallback = Callback<FetchStringResponse>;


pub fn get_all_notes(callback: FetchJsonCallback<Vec<Note>>) -> FetchTask {
    let uri = format!("http://{}:{}/notes", BACKEND_HOST, BACKEND_PORT);
    let request = Request::get(uri).body(Nothing).unwrap();
    yew::services::FetchService::fetch(request, callback).unwrap()
}

pub fn save_single_note(callback: FetchStringCallback, note_id: uuid::Uuid, obj: Json<&serde_json::Value>) -> FetchTask {
    let uri = format!(
        "http://{}:{}/notes/{}/update",
        BACKEND_HOST, BACKEND_PORT, note_id
    );
    let request = Request::builder()
        .method("PATCH")
        .uri(uri)
        .header("Content-Type", "application/json")
        .body(obj)
        .unwrap();
    yew::services::FetchService::fetch(request, callback).unwrap()
}

pub fn get_single_note(callback: FetchJsonCallback<Note>,  note_id: uuid::Uuid) -> FetchTask {
    let uri = format!(
        "http://{}:{}/notes/{}",
        BACKEND_HOST, BACKEND_PORT, note_id
    );
    let request = Request::get(uri).body(Nothing).unwrap();
    yew::services::FetchService::fetch(request, callback).unwrap()
}

pub fn delete_single_note(callback: FetchStringCallback, note_id: uuid::Uuid) -> FetchTask {
    let uri = format!(
        "http://{}:{}/notes/{}/delete",
        BACKEND_HOST, BACKEND_PORT, note_id
    );
    let request = Request::delete(uri).body(Nothing).unwrap();
    yew::services::FetchService::fetch(request, callback).unwrap()
}
