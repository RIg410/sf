use ident::source::Source;
use subscription::model::UserSubscription;

use crate::model::{Freeze, family::Family};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClientRole {
    pub freeze_days: u32,
    pub freeze: Option<Freeze>,
    pub subscriptions: Vec<UserSubscription>,
    pub family: Family,
    pub come_from: Source,
}

impl Default for ClientRole {
    fn default() -> Self {
        ClientRole {
            freeze_days: 0,
            freeze: None,
            subscriptions: Vec::new(),
            family: Family::default(),
            come_from: Source::default(),
        }
    }
}
