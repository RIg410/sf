use crate::day::DayId;
use chrono::{DateTime, Datelike as _, Local, Utc, Weekday};
use serde::{Deserialize, Serialize};
use time::at_mondays_midnight;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WeekId(DateTime<Utc>);

impl WeekId {
    pub fn new(date_time: DateTime<Local>) -> Self {
        WeekId(at_mondays_midnight(date_time).with_timezone(&Utc))
    }

    pub fn local(&self) -> DateTime<Local> {
        self.0.with_timezone(&Local)
    }

    pub fn id(&self) -> DateTime<Utc> {
        self.0
    }

    pub fn next(&self) -> Self {
        WeekId(self.0 + chrono::Duration::days(7))
    }

    pub fn prev(&self) -> Self {
        WeekId(self.0 - chrono::Duration::days(7))
    }

    pub fn has_week(&self) -> bool {
        let now = Utc::now();
        let max_year = now.year() + 2;
        let current_year = self.0.year();
        current_year <= max_year && self.next().0 > now
    }

    pub fn day(&self, weekday: Weekday) -> DayId {
        let date = self.local() + chrono::Duration::days(weekday.num_days_from_monday() as i64);
        DayId(date.with_timezone(&Utc))
    }
}

impl Default for WeekId {
    fn default() -> Self {
        WeekId::new(Local::now())
    }
}

impl From<DateTime<Local>> for WeekId {
    fn from(date_time: DateTime<Local>) -> Self {
        WeekId::new(date_time)
    }
}
