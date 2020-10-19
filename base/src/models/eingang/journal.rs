use serde::{Deserialize, Serialize};
use super::{note::Note, task::Task, thread::Thread};

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

impl Period {

    fn to_timedelta(&self) -> chrono::Duration {
        let delta = chrono::Duration::weeks(self.year.unwrap_or_default() as i64 * 52) +
            chrono::Duration::weeks(self.month.unwrap_or_default() as i64 * 4) +
            chrono::Duration::weeks(self.week.unwrap_or_default() as i64) +
            chrono::Duration::days(self.day.unwrap_or_default() as i64) +
            chrono::Duration::hours(self.hour.unwrap_or_default() as i64);
        - delta
    }

    pub fn to_timestamp(&self) -> Timestamp {
        let now = chrono::Utc::now();
        now + self.to_timedelta()
    }
}
