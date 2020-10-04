use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub title: String,
    pub content: String,
    meta: Meta,
}

#[derive(Serialize, Deserialize, Debug)]
struct Meta {
    created_on: chrono::NaiveDate,
    last_modified: chrono::NaiveDate,
    uuid: uuid::Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub title: String,
    pub content: String,
    pub status: TaskStatus,
    meta: Meta
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskStatus {
    Open,
    Waiting,
    Done,
    Deactivated,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thread {
    pub notes: Vec<Note>,
    pub tasks: Vec<Task>,
    meta: Meta
}
