use decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Statistics {
    pub earned: Decimal,
    pub couch_rewards: Decimal,
}
