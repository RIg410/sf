use super::User;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Family {
    #[serde(default)]
    pub payer_id: Option<ObjectId>,
    #[serde(default)]
    pub is_individual: bool,
    #[serde(skip)]
    pub payer: Option<Box<User>>,
    #[serde(default)]
    pub children_ids: Vec<ObjectId>,
    #[serde(skip)]
    pub children: Vec<User>,
    #[serde(default)]
    pub members: Vec<ObjectId>,
}

impl Family {
    pub fn exists(&self) -> bool {
        self.payer_id.is_some() || !self.children_ids.is_empty()
    }
}
