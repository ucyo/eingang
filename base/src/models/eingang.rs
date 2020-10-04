#![allow(dead_code)]
use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Note {
    pub title: String,
    pub content: String,
    meta: Meta,
}

impl Note {
    pub fn new(content: String) -> Self {
        Note {
            content,
            ..Default::default()
        }
    }
    pub fn with_title(content: String, title: String) -> Self {
        Note {
            title,
            content,
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

impl Meta {
    pub fn new() -> Self {
        Meta::default()
    }
    pub fn update_modified_date(&mut self) {
        self.last_modified = chrono::Utc::now()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Task {
    pub title: String,
    pub content: String,
    pub status: TaskStatus,
    meta: Meta
}

impl From<Note> for Task {
    fn from(note: Note) -> Self {
        Task {
            content: note.content,
            title: note.title,
            meta: note.meta,
            ..Default::default()
        }
    }
}

impl Task {
    pub fn new(content: String) -> Self {
        Task {
            content,
            ..Default::default()
        }
    }
    pub fn with_title(content: String, title: String) -> Self {
        Task {
            content,
            title,
            ..Default::default()
        }
    }
    pub fn with_title_and_status(content: String, title: String, status: TaskStatus) -> Self {
        Task {
            content,
            title,
            status,
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

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Open
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Thread {
    pub notes: Vec<Note>,
    pub tasks: Vec<Task>,
    meta: Meta
}

impl Thread {
    pub fn new() -> Self {
        Thread::default()
    }
}
