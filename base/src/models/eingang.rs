//! # Note model
//!
//! In this crate all model and struct defintions regarding the
//! `Note` componented are saved.
#![allow(dead_code)]
use serde::{Deserialize, Serialize};

pub trait Idable {
    fn get_uuid(&self) -> uuid::Uuid;
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NoteQuery {
    pub title: Option<String>,
    pub content: Option<String>,
}

impl NoteQuery {
    fn new() -> Self {
        NoteQuery::default()
    }
}

/// The Note struct.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Note {
    pub title: String,
    pub content: String,
    meta: Meta,
}

impl Note {
    /// Returns a new Note object by using only the content field. The title
    /// can be left empty and added later on.
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
    pub fn update_modified_date(&mut self) {
        self.meta.update_modified_date()
    }
}

impl Idable for Note {
    fn get_uuid(&self) -> uuid::Uuid {
        self.meta.uuid
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
        Default::default()
    }
    pub fn update_modified_date(&mut self) {
        self.last_modified = chrono::Utc::now()
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TaskQuery {
    pub title: Option<String>,
    pub content: Option<String>,
    pub status: Option<String>,
}

impl TaskQuery {
    fn new() -> Self {
        TaskQuery::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Task {
    pub title: String,
    pub content: String,
    pub status: TaskStatus,
    meta: Meta,
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

pub type NoteUuid = uuid::Uuid;
pub type TaskUuid = uuid::Uuid;

// TODO Make Thread to actually only save UUID, and not(!) the note or thread
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Thread {
    pub notes: Vec<NoteUuid>,
    pub tasks: Vec<TaskUuid>,
    meta: Meta,
}

impl Thread {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn with_notes(notes: Vec<NoteUuid>) -> Self {
        Thread {
            notes,
            ..Default::default()
        }
    }
    pub fn with_tasks(tasks: Vec<TaskUuid>) -> Self {
        Thread {
            tasks,
            ..Default::default()
        }
    }
    pub fn with_tasks_and_notes(tasks: Vec<TaskUuid>, notes: Vec<NoteUuid>) -> Self {
        Thread {
            tasks,
            notes,
            ..Default::default()
        }
    }
    pub fn add_note(&mut self, note: NoteUuid) {
        self.notes.push(note);
        self.update_modified_date();
    }
    pub fn add_task(&mut self, task: TaskUuid) {
        self.tasks.push(task);
        self.update_modified_date();
    }
    pub fn update_modified_date(&mut self) {
        self.meta.update_modified_date()
    }
}

impl Idable for Thread {
    fn get_uuid(&self) -> uuid::Uuid {
        self.meta.uuid
    }
}


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ThreadQuery {
    pub task: Option<String>,
    pub tasks: Option<Vec<String>>,
    pub note: Option<String>,
    pub notes: Option<Vec<String>>,
    pub filter: Option<ThreadFilter>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ThreadFilter {
    Tasks,
    Notes,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ThreadResponse {
    Threads(Thread),
    Tasks(Vec<TaskUuid>),
    Notes(Vec<NoteUuid>)
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct JournalQuery {
    pub after:  Option<String>,
    pub before: Option<String>,
    pub during: Option<Period>,
    pub untouched: Option<Period>,
}

#[derive(Serialize, Default, Deserialize, Debug)]
pub struct Period {
    year: Option<u32>,
    month: Option<u32>,
    week: Option<u32>,
    day: Option<u32>,
    hour: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum JournalResponse {
    Thread(Thread),
    Note(Note),
    Task(Task),
}

#[cfg(test)]
mod tests {
    use super::{Note, Task, TaskStatus, Thread, Idable};

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

    #[test]
    fn create_thread() {
        let note = Note::new("note".to_string());
        let task = Task::new("task".to_string());
        let mut thread = Thread::new();
        thread.add_note(note.get_uuid());
        thread.add_task(task.get_uuid());
        assert_eq!(thread.tasks[0], task.get_uuid());
        assert_eq!(thread.notes[0], note.get_uuid());
    }
}
