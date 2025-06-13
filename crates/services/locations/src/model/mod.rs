use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub address: String,
    pub working_hours: WorkingHours,
    pub halls: Vec<Hall>,
    #[serde(default)]
    pub version: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkingHours {
    pub monday: Option<DayHours>,
    pub tuesday: Option<DayHours>,
    pub wednesday: Option<DayHours>,
    pub thursday: Option<DayHours>,
    pub friday: Option<DayHours>,
    pub saturday: Option<DayHours>,
    pub sunday: Option<DayHours>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DayHours {
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub open: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub close: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hall {
    pub id: ObjectId,
    pub name: String,
}

impl Default for Location {
    fn default() -> Self {
        Location {
            id: ObjectId::new(),
            name: String::new(),
            address: String::new(),
            working_hours: WorkingHours::default(),
            halls: Vec::new(),
            version: 0,
        }
    }
}

impl Default for WorkingHours {
    fn default() -> Self {
        WorkingHours {
            monday: None,
            tuesday: None,
            wednesday: None,
            thursday: None,
            friday: None,
            saturday: None,
            sunday: None,
        }
    }
}

impl Hall {
    pub fn new(name: String) -> Self {
        Hall {
            id: ObjectId::new(),
            name,
        }
    }
}