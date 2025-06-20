use bson::oid::ObjectId;
use chrono::{DateTime, Datelike, Local, Utc};
use decimal::Decimal;
use ident::{day::DayId, slot::Slot};
use serde::{Deserialize, Serialize};
use trainings::model::{Training, statistics::Statistics};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Day {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    date_time: DateTime<Utc>,
    pub weekday: chrono::Weekday,
    pub training: Vec<Training>,
    #[serde(default)]
    pub version: u64,
}

impl Day {
    pub fn new(day: DayId) -> Day {
        Day {
            weekday: day.local().weekday(),
            training: Vec::new(),
            id: ObjectId::new(),
            date_time: day.id(),
            version: 0,
        }
    }

    pub fn get_training(&self, slot: Slot) -> Option<&Training> {
        self.training.iter().find(|t| t.get_slot() == slot)
    }

    pub fn copy_day(id: DayId, day: Day) -> Day {
        let training = day
            .training
            .into_iter()
            .filter(|t| !t.is_one_time)
            .map(|t| Training::with_day_and_training(id, t))
            .collect::<Vec<_>>();

        Day {
            id: ObjectId::new(),
            date_time: id.id(),
            weekday: id.week_day(),
            training,
            version: 0,
        }
    }

    pub fn day_id(&self) -> DayId {
        unsafe { DayId::from_utc(self.date_time) }
    }

    pub fn day_date(&self) -> DateTime<Local> {
        self.date_time.with_timezone(&Local)
    }

    pub fn has_conflict_with(self, slot: Slot) -> Option<Training> {
        self.training
            .into_iter()
            .filter(|t| !t.is_canceled)
            .find(|t| t.get_slot().has_conflict(&slot))
    }

    pub fn has_conflict(&self) -> bool {
        let mut slots: Vec<Slot> = Vec::new();
        for training in &self.training {
            let slot = training.get_slot();
            if slots.iter().any(|s| s.has_conflict(&slot)) {
                return true;
            }

            slots.push(slot);
        }

        false
    }

    pub fn statistic(&self) -> StatisticsSummary {
        StatisticsSummary::new(
            self.training
                .iter()
                .filter_map(|t| t.statistics.as_ref().map(|s| (s, t.clients.len() as u32))),
        )
    }
}

pub struct Collision {
    pub day_id: DayId,
    pub training_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StatisticsSummary {
    pub earned: Decimal,
    pub couch_rewards: Decimal,
    pub training_count: u32,
    pub training_without_rewards: u32,
    pub clients_count: u32,
    pub sub_avg: Decimal,
}

impl StatisticsSummary {
    pub fn new<'s>(stat: impl Iterator<Item = (&'s Statistics, u32)>) -> StatisticsSummary {
        let mut stat = stat.fold(
            StatisticsSummary::default(),
            |mut acc, (s, clients_count)| {
                acc.earned += s.earned;
                acc.couch_rewards += s.couch_rewards;
                acc.training_count += 1;
                acc.clients_count += clients_count;
                if clients_count == 0 {
                    acc.training_without_rewards += 1;
                }
                acc
            },
        );

        stat.sub_avg = if stat.clients_count == 0 || stat.earned.is_zero() {
            Decimal::zero()
        } else {
            stat.earned / Decimal::int(stat.clients_count as i64)
        };
        stat
    }
}
