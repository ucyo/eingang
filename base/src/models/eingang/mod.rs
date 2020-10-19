//! # Note model
//!
//! In this crate all model and struct defintions regarding the
//! `Note` componented are saved.

pub trait Idable {
    fn get_uuid(&self) -> uuid::Uuid;
}

pub mod note;
pub mod task;
pub mod thread;
pub mod meta;
pub mod journal;



// TODO Figure out ordering of methods
// Notes should be compared by content
// Tasks compared by content
// threads not at all (for the moment)
// Time and Notes.meta.lastmodified
// Sortable by status for Tasks

// use std::cmp::Ordering;

// impl Ord for Meta {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.last_modified.cmp(&other.last_modified)
//     }
// }

// impl PartialOrd for Meta {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.last_modified.cmp(&other.last_modified))
//     }
// }

// impl Eq for Meta {}

// impl PartialEq for Meta {
//     fn eq(&self, other: &Self) -> bool {
//         self.last_modified == other.last_modified
//     }
// }
