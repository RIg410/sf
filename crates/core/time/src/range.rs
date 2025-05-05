use chrono::{DateTime, Datelike as _, Days, Local, Months, Timelike as _};
use eyre::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
pub struct MonthRange(DateTime<Local>);

impl Default for MonthRange {
    fn default() -> Self {
        MonthRange(Local::now())
    }
}

impl MonthRange {
    pub fn new(date: DateTime<Local>) -> MonthRange {
        MonthRange(date)
    }


    pub fn next(&self) -> Result<Self, Error> {
        fn inner(range: MonthRange) -> Option<MonthRange> {
            let base_date = range.base_date();
            let next = base_date.checked_add_months(Months::new(1))?;
            Some(MonthRange(next))
        }
        inner(*self).ok_or_else(|| eyre::eyre!("Failed to calculate next range for {:?}", self))
    }

    pub fn prev(&self) -> Result<Self, Error> {
        fn inner(range: MonthRange) -> Option<MonthRange> {
            let base_date = range.base_date();
            let prev = base_date.checked_sub_months(Months::new(1))?;
            Some(MonthRange(prev))
        }
        inner(*self).ok_or_else(|| eyre::eyre!("Failed to calculate prev range for {:?}", self))
    }

    pub fn base_date(&self) -> DateTime<Local> {
        self.0
    }

    pub fn range(&self) -> Result<(DateTime<Local>, DateTime<Local>), Error> {
        fn inner(range: MonthRange) -> Option<(DateTime<Local>, DateTime<Local>)> {
            let base_date = range.base_date();
            let to = base_date
                .with_day0(0)?
                .checked_add_months(Months::new(1))?
                .checked_sub_days(Days::new(1))?
                .with_hour(23)?
                .with_minute(59)?
                .with_second(59)?;

            let from = to
                .with_day0(0)?
                .with_hour(0)?
                .with_minute(0)?
                .with_second(0)?;
            Some((from, to))
        }

        inner(*self).ok_or_else(|| eyre::eyre!("Failed to calculate range for {:?}", self))
    }
}
