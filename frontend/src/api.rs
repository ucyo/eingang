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

/// This macro takes four arguments is defined for none uuid notes.
/// $func_name: Name of the function to be generated
/// $callback_ty: Type of the callback
/// $uri: URL of the backend with 2 braces
/// $method: HTTP protocol
macro_rules! create_api {
    ($func_name:ident,
        $callback_type:ty,
        $uri:expr,
        $method:ident
       ) => {
        pub fn $func_name(callback: $callback_type) -> FetchTask {
            let uri = format!($uri, BACKEND_HOST, BACKEND_PORT);
            let request = Request::builder()
                .method(stringify!($method))
                .uri(uri)
                .body(Nothing)
                .unwrap();
            yew::services::FetchService::fetch(request, callback).unwrap()
        }
    };
}

macro_rules! create_single_api_json {
    ($func_name:ident,
        $callback_type:ty,
        $uri:expr,
        $method:ident
       ) => {
        pub fn $func_name(
            callback: $callback_type,
            id: uuid::Uuid,
            obj: Json<&serde_json::Value>,
        ) -> FetchTask {
            let uri = format!($uri, BACKEND_HOST, BACKEND_PORT, id);
            let request = Request::builder()
                .method(stringify!($method))
                .uri(uri)
                .header("Content-Type", "application/json")
                .body(obj)
                .unwrap();
            yew::services::FetchService::fetch(request, callback).unwrap()
        }
    };
}

macro_rules! create_api_uuid {
    ($func_name:ident,
        $callback_type:ty,
        $uri:expr,
        $method:ident
       ) => {
        pub fn $func_name(callback: $callback_type, id: uuid::Uuid) -> FetchTask {
            let uri = format!($uri, BACKEND_HOST, BACKEND_PORT, id);
            let request = Request::builder()
                .method(stringify!($method))
                .uri(uri)
                .body(Nothing)
                .unwrap();
            yew::services::FetchService::fetch(request, callback).unwrap()
        }
    };
}

create_single_api_json! {
    save_single_note, FetchStringCallback, "http://{}:{}/notes/{}/update",
    PATCH
}

create_api! {
    get_all_notes, FetchJsonCallback<Vec<Note>>, "http://{}:{}/notes",
    GET
}

create_api_uuid! {
    get_single_note,
    FetchJsonCallback<Note>,
    "http://{}:{}/notes/{}",
    GET
}

create_api_uuid! {
    delete_single_note,
    FetchStringCallback,
    "http://{}:{}/notes/{}/delete",
    DELETE
}
