use serde::{Deserialize, Serialize};
use super::{meta::Meta, Idable};

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
