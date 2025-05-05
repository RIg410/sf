use serde::{Deserialize, Serialize};
use decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Statistics {
    pub earned: Decimal,
    pub couch_rewards: Decimal,
}
