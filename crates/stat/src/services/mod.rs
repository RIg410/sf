use advertising::AdvertisingStatService;
use ai::Ai;
use ledger::service::{history::History, requests::Requests, users::Users};

pub mod advertising;

pub struct Statistics {
    pub advertising: AdvertisingStatService,
}

impl Statistics {
    pub fn new(history: History, users: Users, requests: Requests, ai: Ai) -> Self {
        Self {
            advertising: AdvertisingStatService::new(requests, users, history, ai),
        }
    }
}
