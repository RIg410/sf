use ident::source::Source;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default)]
pub struct AdvertisingConversionStat {
    pub sources: HashMap<Source, SourceStat>,
    pub ai_comment: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct SourceStat {
    pub processed_requests: u32,
    pub trial_visits: u32,
    pub bought_memberships: u32,
    pub conversion_to_trial: f64,
    pub conversion_to_membership: f64,
}

impl SourceStat {
    pub fn calculate_conversions(&mut self) {
        self.conversion_to_trial = if self.processed_requests == 0 {
            0.0
        } else {
            self.trial_visits as f64 / self.processed_requests as f64 * 100.0
        };
        self.conversion_to_membership = if self.trial_visits == 0 {
            0.0
        } else {
            self.bought_memberships as f64 / self.trial_visits as f64 * 100.0
        };
    }
}
