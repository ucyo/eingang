pub trait Idable {
    fn get_uuid(&self) -> uuid::Uuid;
}

pub type Timestamp = chrono::DateTime<chrono::Utc>;

pub mod journal;
pub mod meta;
pub mod note;
pub mod task;
pub mod thread;
