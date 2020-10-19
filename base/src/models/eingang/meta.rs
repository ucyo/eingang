use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Meta {
    created_on: chrono::DateTime<chrono::Utc>,
    last_modified: chrono::DateTime<chrono::Utc>,
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
