use serde::{Deserialize, Serialize};
use super::Timestamp;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Meta {
    created_on: Timestamp,
    last_modified: Timestamp,
    pub(crate) uuid: uuid::Uuid,
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

    pub fn update_modified_date(&mut self) {
        self.last_modified = chrono::Utc::now()
    }
}


impl PartialOrd<Timestamp> for Meta {
    fn partial_cmp(&self, other: &Timestamp) -> Option<std::cmp::Ordering> {
        self.last_modified.partial_cmp(&other)
    }
}

impl PartialEq<Timestamp> for Meta {
    fn eq(&self, other: &Timestamp) -> bool {
        self.last_modified == *other
    }
}
