use serde::{Deserialize, Serialize};
use super::{meta::Meta, Idable};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NoteQuery {
    pub title: Option<String>,
    pub content: Option<String>,
}

/// The Note struct.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Note {
    pub title: String,
    pub content: String,
    pub(crate) meta: Meta,
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

#[cfg(test)]
mod tests {
    use super::{Note, Idable};
    use crate::models::eingang::{task::Task, task::TaskStatus, thread::Thread};

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

use super::Timestamp;

impl PartialOrd<Timestamp> for Note {
    fn partial_cmp(&self, other: &Timestamp) -> Option<std::cmp::Ordering> {
        self.meta.partial_cmp(&other)
    }
}

impl PartialEq<Timestamp> for Note {
    fn eq(&self, other: &Timestamp) -> bool {
        self.meta == *other
    }
}
