use super::{
    calendar::Calendar, history::History, requests::Requests, treasury::Treasury, users::Users,
};
use ai::Ai;
use chrono::{DateTime, Datelike as _, Local, NaiveDate};

mod advertising;

pub struct Statistics {
    calendar: Calendar,
    history: History,
    users: Users,
    requests: Requests,
    treasury: Treasury,
    ai: Ai,
}

impl Statistics {
    pub(crate) fn new(
        calendar: Calendar,
        history: History,
        users: Users,
        requests: Requests,
        ai: Ai,
        treasury: Treasury,
    ) -> Self {
        Self {
            calendar,
            history,
            users,
            requests,
            ai,
            treasury,
        }
    }
}

pub fn month_id(date: DateTime<Local>) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap()
}
