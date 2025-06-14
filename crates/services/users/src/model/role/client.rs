use ident::source::Source;
use subscription::model::UserSubscription;

use crate::model::{Freeze, family::Family};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClientRole {
    #[serde(default)]
    pub freeze_days: u32,
    #[serde(default)]
    pub freeze: Option<Freeze>,
    #[serde(default)]
    pub subscriptions: Vec<UserSubscription>,
    pub family: Family,
    #[serde(default)]
    pub come_from: Source,
}
