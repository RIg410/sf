use bson::oid::ObjectId;
use chrono::{DateTime, Local, Timelike, Utc};
use std::fmt::Debug;

use crate::{day::DayId, training::TrainingId};

#[derive(Clone, PartialEq, Eq, Copy)]
pub struct Slot {
    room: ObjectId,
    start_at: DateTime<Utc>,
    duration_min: u32,
}

impl Slot {
    pub fn new(start_at: DateTime<Utc>, duration_min: u32, room: ObjectId) -> Slot {
        Slot {
            start_at,
            duration_min,
            room,
        }
    }

    pub fn training_id(&self) -> TrainingId {
        TrainingId {
            start_at: self.start_at,
            room: self.room,
        }
    }

    pub fn in_slot(&self, time: DateTime<Local>) -> bool {
        let start = self.start_at.with_timezone(&Local);
        let end = start + chrono::Duration::minutes(self.duration_min as i64);

        time >= start && time < end
    }

    pub fn start_at(&self) -> DateTime<Local> {
        self.start_at.with_timezone(&Local)
    }

    pub fn start_at_utc(&self) -> DateTime<Utc> {
        self.start_at
    }

    pub fn end_at(&self) -> DateTime<Local> {
        self.start_at.with_timezone(&Local) + chrono::Duration::minutes(self.duration_min as i64)
    }

    pub fn room(&self) -> ObjectId {
        self.room
    }

    pub fn duration_min(&self) -> u32 {
        self.duration_min
    }

    pub fn has_conflict(&self, other: &Slot) -> bool {
        if self.room != other.room {
            return false;
        }

        let this_start = self.start_at + chrono::Duration::milliseconds(1);
        let this_end = self.start_at + chrono::Duration::minutes(self.duration_min as i64)
            - chrono::Duration::milliseconds(1);

        let (start, end) = (
            other.start_at,
            other.start_at + chrono::Duration::minutes(other.duration_min as i64),
        );
        if start >= this_start && start < this_end {
            return true;
        }

        if end > this_start && end <= this_end {
            return true;
        }

        if start <= this_start && end >= this_end {
            return true;
        }

        false
    }

    pub fn day_id(&self) -> DayId {
        DayId::from(self.start_at())
    }

    pub fn with_day(&self, day_id: DayId) -> Slot {
        let day_local = day_id.local();
        let start_at = self.start_at();

        let start_at = day_local
            .with_hour(start_at.hour())
            .unwrap()
            .with_minute(start_at.minute())
            .unwrap()
            .with_second(start_at.second())
            .unwrap();
        Slot::new(start_at.with_timezone(&Utc), self.duration_min, self.room)
    }

    pub fn with_room(mut self, room: ObjectId) -> Slot {
        self.room = room;
        self
    }
}

impl Debug for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start_at = self.start_at.with_timezone(&Local);
        let fmt = "%H:%M";
        write!(
            f,
            "[({}):{}<->{}]",
            start_at.format("%d.%m"),
            start_at.format(fmt),
            (start_at + chrono::Duration::minutes(self.duration_min as i64)).format(fmt)
        )
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone as _;

    use super::*;

    #[test]
    fn test_conflict_different_days_no_overlap() {
        let room = ObjectId::new();
        let slot1 = Slot::new(
            Utc.with_ymd_and_hms(2023, 9, 14, 11, 15, 0)
                .single()
                .unwrap(),
            30,
            room,
        );
        let slot2 = Slot::new(
            Utc.with_ymd_and_hms(2023, 9, 13, 11, 0, 0)
                .single()
                .unwrap(),
            30,
            room,
        );

        assert!(!slot1.has_conflict(&slot2));
    }

    #[test]
    fn test_conflict_different_days_overlap() {
        let room = ObjectId::new();
        let slot1 = Slot::new(
            Utc.with_ymd_and_hms(2023, 9, 14, 11, 15, 0)
                .single()
                .unwrap(),
            30,
            room,
        );
        let slot2 = Slot::new(
            Utc.with_ymd_and_hms(2023, 9, 14, 11, 0, 0)
                .single()
                .unwrap(),
            30,
            room,
        );

        assert!(slot1.has_conflict(&slot2));
    }

    #[test]
    fn test_slot_creation() {
        let start_at = Utc
            .with_ymd_and_hms(2023, 10, 1, 12, 0, 0)
            .single()
            .unwrap();
        let duration_min = 60;
        let slot = Slot::new(start_at, duration_min, ObjectId::new());

        assert_eq!(slot.start_at, start_at);
        assert_eq!(slot.duration_min, duration_min);
    }

    #[test]
    fn test_no_conflict() {
        let room = ObjectId::new();
        let slot1 = Slot::new(
            Utc.with_ymd_and_hms(2023, 10, 1, 12, 0, 0)
                .single()
                .unwrap(),
            60,
            room,
        );
        let slot2 = Slot::new(
            Utc.with_ymd_and_hms(2023, 10, 1, 14, 0, 0)
                .single()
                .unwrap(),
            60,
            room,
        );

        assert!(!slot1.has_conflict(&slot2));
    }

    #[test]
    fn test_conflict_start_overlap() {
        let room = ObjectId::new();
        let slot1 = Slot::new(
            Utc.with_ymd_and_hms(2023, 10, 1, 12, 0, 0)
                .single()
                .unwrap(),
            60,
            room,
        );
        let slot2 = Slot::new(
            Utc.with_ymd_and_hms(2023, 10, 1, 12, 30, 0)
                .single()
                .unwrap(),
            60,
            room,
        );

        assert!(slot1.has_conflict(&slot2));
    }

    #[test]
    fn test_conflict_end_overlap() {
        let room = ObjectId::new();
        let slot1 = Slot::new(
            Utc.with_ymd_and_hms(2023, 10, 1, 12, 0, 0)
                .single()
                .unwrap(),
            60,
            room,
        );
        let slot2 = Slot::new(
            Utc.with_ymd_and_hms(2023, 10, 1, 11, 30, 0)
                .single()
                .unwrap(),
            60,
            room,
        );

        assert!(slot1.has_conflict(&slot2));
    }

    #[test]
    fn test_conflict_full_overlap() {
        let room = ObjectId::new();
        let slot1 = Slot::new(
            Utc.with_ymd_and_hms(2023, 10, 1, 12, 0, 0)
                .single()
                .unwrap(),
            60,
            room,
        );
        let slot2 = Slot::new(
            Utc.with_ymd_and_hms(2023, 10, 1, 12, 0, 0)
                .single()
                .unwrap(),
            30,
            room,
        );

        assert!(slot1.has_conflict(&slot2));
    }

    #[test]
    fn test_conflict_contained_within() {
        let room = ObjectId::new();
        let slot1 = Slot::new(
            Utc.with_ymd_and_hms(2023, 10, 1, 12, 0, 0)
                .single()
                .unwrap(),
            120,
            room,
        );
        let slot2 = Slot::new(
            Utc.with_ymd_and_hms(2023, 10, 1, 12, 30, 0)
                .single()
                .unwrap(),
            30,
            room,
        );

        assert!(slot1.has_conflict(&slot2));
    }

    #[test]
    fn test_conflict_exact_match() {
        let room = ObjectId::new();

        let slot1 = Slot::new(
            Utc.with_ymd_and_hms(2023, 10, 1, 12, 0, 0)
                .single()
                .unwrap(),
            60,
            room,
        );
        let slot2 = Slot::new(
            Utc.with_ymd_and_hms(2023, 10, 1, 12, 0, 0)
                .single()
                .unwrap(),
            60,
            room,
        );

        assert!(slot1.has_conflict(&slot2));
    }

    #[test]
    fn test_no_conflict_adjacent_slots() {
        let room = ObjectId::new();

        let slot1 = Slot::new(
            Utc.with_ymd_and_hms(2023, 10, 1, 12, 0, 0)
                .single()
                .unwrap(),
            60,
            room,
        );
        let slot2 = Slot::new(
            Utc.with_ymd_and_hms(2023, 10, 1, 13, 0, 0)
                .single()
                .unwrap(),
            60,
            room,
        );

        assert!(!slot1.has_conflict(&slot2));
    }

    #[test]
    fn test_conflict_partial_overlap() {
        let room = ObjectId::new();

        let slot1 = Slot::new(
            Utc.with_ymd_and_hms(2023, 10, 1, 12, 0, 0)
                .single()
                .unwrap(),
            90,
            room,
        );
        let slot2 = Slot::new(
            Utc.with_ymd_and_hms(2023, 10, 1, 13, 0, 0)
                .single()
                .unwrap(),
            60,
            room,
        );

        assert!(slot1.has_conflict(&slot2));
    }

    #[test]
    fn test_no_conflict_different_days() {
        let room = ObjectId::new();

        let slot1 = Slot::new(
            Utc.with_ymd_and_hms(2023, 10, 1, 12, 0, 0)
                .single()
                .unwrap(),
            60,
            room,
        );
        let slot2 = Slot::new(
            Utc.with_ymd_and_hms(2023, 10, 2, 12, 0, 0)
                .single()
                .unwrap(),
            60,
            room,
        );

        assert!(!slot1.has_conflict(&slot2));
    }

    #[test]
    fn test_conflict_overlap_midnight() {
        let room = ObjectId::new();
        let slot1 = Slot::new(
            Utc.with_ymd_and_hms(2023, 10, 1, 23, 30, 0)
                .single()
                .unwrap(),
            60,
            room,
        );
        let slot2 = Slot::new(
            Utc.with_ymd_and_hms(2023, 10, 2, 0, 0, 0).single().unwrap(),
            60,
            room,
        );

        assert!(slot1.has_conflict(&slot2));
    }

    #[test]
    fn test_in_slot_exact_start() {
        let slot = Slot::new(
            Local
                .with_ymd_and_hms(2023, 10, 1, 12, 0, 0)
                .single()
                .unwrap()
                .with_timezone(&Utc),
            60,
            ObjectId::new(),
        );
        let time = Local
            .with_ymd_and_hms(2023, 10, 1, 12, 0, 0)
            .single()
            .unwrap();

        assert!(slot.in_slot(time));
    }

    #[test]
    fn test_in_slot_within_duration() {
        let slot = Slot::new(
            Local
                .with_ymd_and_hms(2023, 10, 1, 12, 0, 0)
                .single()
                .unwrap()
                .with_timezone(&Utc),
            60,
            ObjectId::new(),
        );
        let time = Local
            .with_ymd_and_hms(2023, 10, 1, 12, 30, 0)
            .single()
            .unwrap();

        assert!(slot.in_slot(time));
    }

    #[test]
    fn test_in_slot_exact_end() {
        let slot = Slot::new(
            Local
                .with_ymd_and_hms(2023, 10, 1, 12, 0, 0)
                .single()
                .unwrap()
                .with_timezone(&Utc),
            60,
            ObjectId::new(),
        );
        let time = Local
            .with_ymd_and_hms(2023, 10, 1, 13, 0, 0)
            .single()
            .unwrap();

        assert!(!slot.in_slot(time));
    }

    #[test]
    fn test_in_slot_before_start() {
        let slot = Slot::new(
            Local
                .with_ymd_and_hms(2023, 10, 1, 12, 0, 0)
                .single()
                .unwrap()
                .with_timezone(&Utc),
            60,
            ObjectId::new(),
        );
        let time = Local
            .with_ymd_and_hms(2023, 10, 1, 11, 59, 59)
            .single()
            .unwrap();

        assert!(!slot.in_slot(time));
    }

    #[test]
    fn test_in_slot_after_end() {
        let slot = Slot::new(
            Local
                .with_ymd_and_hms(2023, 10, 1, 12, 0, 0)
                .single()
                .unwrap()
                .with_timezone(&Utc),
            60,
            ObjectId::new(),
        );
        let time = Local
            .with_ymd_and_hms(2023, 10, 1, 13, 0, 1)
            .single()
            .unwrap();

        assert!(!slot.in_slot(time));
    }
}
