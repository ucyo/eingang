use super::Timestamp;
use super::{note::Note, task::Task, thread::Thread};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

const TIME: &str = "%Y-%m-%d";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct JournalQuery {
    pub after: Option<String>,
    pub before: Option<String>,
    pub during: Option<Period>,
    pub untouched: Option<Period>,
    pub filter: Option<JournalFilter>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum JournalFilter {
    Threads,
    Notes,
    Tasks,
    All,
}

impl Default for JournalFilter {
    fn default() -> Self {
        JournalFilter::All
    }
}

impl JournalQuery {
    pub fn after_to_timestamp(&self) -> Option<Timestamp> {
        match &self.after {
            Some(s) => match NaiveDate::parse_from_str(s.as_str(), TIME).ok() {
                Some(s) => {
                    let ndt = s.and_hms(23, 59, 59);
                    Some(Timestamp::from_utc(ndt, chrono::Utc))
                }
                _ => None,
            },
            _ => None,
        }
    }
    pub fn before_to_timestamp(&self) -> Option<Timestamp> {
        match &self.before {
            Some(s) => match NaiveDate::parse_from_str(s.as_str(), TIME).ok() {
                Some(s) => {
                    let ndt = s.and_hms(0, 0, 1);
                    Some(Timestamp::from_utc(ndt, chrono::Utc))
                }
                _ => None,
            },
            _ => None,
        }
    }
}

#[derive(Serialize, Default, Deserialize, Debug, Copy, Clone)]
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
        chrono::Duration::days(self.year.unwrap_or_default() as i64 * 365)
            + chrono::Duration::days(self.month.unwrap_or_default() as i64 * 30)
            + chrono::Duration::weeks(self.week.unwrap_or_default() as i64)
            + chrono::Duration::days(self.day.unwrap_or_default() as i64)
            + chrono::Duration::hours(self.hour.unwrap_or_default() as i64)
    }

    pub fn to_timestamp(&self) -> Timestamp {
        let delta = self.to_timedelta();
        chrono::Utc::now().checked_sub_signed(delta).unwrap()
    }
}
