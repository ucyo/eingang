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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
    pub fn with_notes(notes: Vec<Note>) -> Self {
        Thread {
            notes,
            ..Default::default()
        }
    }
    pub fn with_tasks(tasks: Vec<Task>) -> Self {
        Thread {
            tasks,
            ..Default::default()
        }
    }
    pub fn with_tasks_and_notes(tasks: Vec<Task>, notes: Vec<Note>) -> Self {
        Thread {
            tasks,
            notes,
            ..Default::default()
        }
    }
    pub fn add_note(&mut self, note: Note) {
        self.notes.push(note)
    }
    pub fn add_task(&mut self, task:Task) {
        self.tasks.push(task)
    }
}

#[cfg(test)]
mod tests {
    use super::{Note, Task, TaskStatus};
    #[test]
    fn create_note_and_cast_to_task() {
        let c = "content".to_string();
        let t = "title".to_string();
        let note = Note::with_title(c.clone(), t.clone());
        assert_eq!(note.content, c);
        assert_eq!(note.title, t);
        let task = Task::from(note);
        assert_eq!(task.status, TaskStatus::Open)
    }
}
