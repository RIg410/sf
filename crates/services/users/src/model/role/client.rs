use crate::model::{Freeze, family::Family};
use ident::source::Source;
use subscription::model::UserSubscription;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct ClientRole {
    pub freeze_days: u32,
    pub freeze: Option<Freeze>,
    pub come_from: Source,

    pub subscriptions: Vec<UserSubscription>,
    pub family: Family,
}
