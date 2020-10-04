use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub title: String,
    pub content: String,
    meta: Meta,
}

impl Default for Note {
    fn default() -> Self {
        Note {
            title: String::new(),
            content: String::new(),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Meta {
    created_on: chrono::DateTime<chrono::Utc>,
    last_modified: chrono::DateTime<chrono::Utc>,
    uuid: uuid::Uuid,
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            created_on: chrono::Utc::now(),
            last_modified: chrono::Utc::now(),
            uuid: uuid::Uuid::new_v4(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub title: String,
    pub content: String,
    pub status: TaskStatus,
    meta: Meta
}

impl Default for Task {
    fn default() -> Self {
        Task {
            title: String::new(),
            content: String::new(),
            status: TaskStatus::Open,
            ..Default::default()
        }
    }
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

impl Default for Thread {
    fn default() -> Self {
        Thread {
            notes: Vec::new(),
            tasks: Vec::new(),
            ..Default::default()
        }
    }
}
