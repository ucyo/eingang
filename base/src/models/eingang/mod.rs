pub trait Idable {
    fn get_uuid(&self) -> uuid::Uuid;
}

pub trait Marker {
    fn to_markdown(&self) -> String;
}

pub type Timestamp = chrono::DateTime<chrono::Utc>;

pub mod journal;
pub mod meta;
pub mod note;
pub mod task;
pub mod thread;
