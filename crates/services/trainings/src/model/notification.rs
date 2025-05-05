use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Notified {
    None {},
    Tomorrow {},
    ByHours(Vec<ObjectId>),
}

impl Notified {
    pub fn is_notified(&self) -> bool {
        !matches!(self, Notified::None {})
    }
}

impl Default for Notified {
    fn default() -> Self {
        Notified::None {}
    }
}

