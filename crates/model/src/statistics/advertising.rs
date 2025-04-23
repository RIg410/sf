use std::collections::HashMap;

use decimal::Decimal;

use super::source::Source;

pub struct AdvertisingStat {
    pub sources: HashMap<Source, SourceStat>,
    pub ai_comment: String,
}

pub struct SourceStat {
    pub spent: Decimal,
    pub leads: u32,
    pub trial_subscription: u32,
    pub subscriptions: u32,
    pub lead_to_trial_conversion: f64,
    pub trial_to_subscription_conversion: f64,
}
