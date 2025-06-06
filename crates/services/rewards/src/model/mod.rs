use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use decimal::Decimal;
use ident::training::TrainingId;
use serde::{Deserialize, Serialize};
use user::UserRewardContribution;

pub mod user;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reward {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "couch")]
    pub employee: ObjectId,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    pub reward: Decimal,
    pub source: RewardSource,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RewardSource {
    Training {
        training_id: TrainingId,
        name: String,
        #[serde(default)]
        percent: Decimal,
        #[serde(default)]
        user_originals: Vec<UserRewardContribution>,
    },
    Fixed {},
    Recalc {
        comment: String,
    },
}
