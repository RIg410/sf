use ident::source::Source;
use subscription::model::UserSubscription;

use crate::model::{Freeze, family::Family};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[derive(Default)]
pub struct ClientRole {
    pub freeze_days: u32,
    pub freeze: Option<Freeze>,
    pub subscriptions: Vec<UserSubscription>,
    pub family: Family,
    pub come_from: Source,
}

