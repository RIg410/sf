use advertising::AdvertisingStatService;
use ai::Ai;
use history::service::History;
use requests::service::Requests;
use user::UserStat;
use users::{log::UserLog, service::Users};

pub mod advertising;
pub mod user;

pub struct Statistics<L> {
    pub advertising: AdvertisingStatService<L>,
    pub user: UserStat,
}

impl<L: UserLog> Statistics<L> {
    pub fn new(history: History, users: Users<L>, requests: Requests<L>, ai: Ai) -> Self {
        Self {
            advertising: AdvertisingStatService::new(requests, users, history.clone(), ai),
            user: UserStat::new(history),
        }
    }
}
