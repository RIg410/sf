use bson::oid::ObjectId;
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TrainingId {
    pub start_at: DateTime<Utc>,
    pub room: ObjectId,
}

impl TrainingId {
    pub fn start_at(&self) -> DateTime<Local> {
        self.start_at.with_timezone(&Local)
    }
}

