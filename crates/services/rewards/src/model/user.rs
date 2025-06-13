use bson::oid::ObjectId;
use decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRewardContribution {
    pub user: ObjectId,
    pub lesson_price: Decimal,
    pub subscription_price: Decimal,
    pub lessons_count: u32,
}
