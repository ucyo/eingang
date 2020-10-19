use serde::{Deserialize, Serialize};
use super::{meta::Meta, Idable};
use super::note::Note;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TaskQuery {
    pub title: Option<String>,
    pub content: Option<String>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Task {
    pub title: String,
    pub content: String,
    pub status: TaskStatus,
    pub(crate) meta: Meta,
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
    pub fn update_modified_date(&mut self) {
        self.meta.update_modified_date()
    }
}

impl Idable for Task {
    fn get_uuid(&self) -> uuid::Uuid {
        self.meta.uuid
    }
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Open,
    Waiting,
    Closed,
    Deactivated,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Open
    }
}

impl TaskStatus {
    pub fn from(stst: String) -> Option<Self> {
        match stst.to_lowercase().as_str() {
            "closed" | "done" => Some(TaskStatus::Closed),
            "deactivated" | "expired" => Some(TaskStatus::Deactivated),
            "open" => Some(TaskStatus::Open),
            "waiting" | "delegated" | "scheduled" => Some(TaskStatus::Waiting),
            _ => None
        }
    }
}

use super::Timestamp;

impl PartialOrd<Timestamp> for Task {
    fn partial_cmp(&self, other: &Timestamp) -> Option<std::cmp::Ordering> {
        self.meta.partial_cmp(&other)
    }
}

impl PartialEq<Timestamp> for Task {
    fn eq(&self, other: &Timestamp) -> bool {
        self.meta == *other
    }
}
