use chrono::{DateTime, Datelike as _, Local, Utc, Weekday};
use time::at_midnight;

use crate::{training::TrainingId, week::WeekId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DayId(pub(super) DateTime<Utc>);

impl DayId {
    #[allow(deprecated)]
    fn new(date_time: DateTime<Local>) -> Self {
        DayId(at_midnight(date_time).with_timezone(&Utc))
    }

    /// Create DayId from Utc DateTime
    /// # Safety
    /// `date_time` must be midnight in UTC timezone
    pub unsafe fn from_utc(date_time: DateTime<Utc>) -> Self {
        DayId(date_time)
    }

    pub fn local(&self) -> DateTime<Local> {
        self.0.with_timezone(&Local)
    }

    pub fn id(&self) -> DateTime<Utc> {
        self.0
    }

    pub fn week_day(&self) -> Weekday {
        self.local().weekday()
    }

    pub fn week_id(&self) -> WeekId {
        WeekId::new(self.local())
    }

    pub fn next(&self) -> Self {
        DayId(self.0 + chrono::Duration::days(1))
    }

    pub fn prev(&self) -> Self {
        DayId(self.0 - chrono::Duration::days(1))
    }
}

impl From<DateTime<Local>> for DayId {
    fn from(date_time: DateTime<Local>) -> Self {
        DayId::new(date_time)
    }
}

impl From<DateTime<Utc>> for DayId {
    fn from(date_time: DateTime<Utc>) -> Self {
        DayId::from(date_time.with_timezone(&Local))
    }
}

impl From<TrainingId> for DayId {
    fn from(training_id: TrainingId) -> Self {
        DayId::from(training_id.start_at)
    }
}

impl Default for DayId {
    fn default() -> Self {
        DayId::new(Local::now())
    }
}
