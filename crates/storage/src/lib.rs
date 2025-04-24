pub mod calendar;
pub mod history;
pub mod notification;
pub mod payment;
pub mod requests;
pub mod subscription;
pub mod treasury;
pub mod user;

use eyre::Result;
use history::HistoryStore;
use notification::NotificationStore;
use requests::RequestStore;
use std::sync::Arc;
use store::session::Db;
use user::UserStore;

const DB_NAME: &str = "ledger_db";

#[derive(Clone)]
pub struct Storage {
    pub db: Arc<Db>,
    pub users: Arc<UserStore>,
    pub calendar: Arc<calendar::CalendarStore>,
    pub treasury: Arc<treasury::TreasuryStore>,
    pub subscriptions: Arc<subscription::SubscriptionsStore>,
    pub history: Arc<HistoryStore>,
    pub requests: Arc<RequestStore>,
    pub notification: Arc<NotificationStore>,
}

impl Storage {
    pub async fn new(uri: &str) -> Result<Self> {
        let db = Db::new(uri, DB_NAME).await?;
        let users = UserStore::new(&db).await?;
        let calendar = calendar::CalendarStore::new(&db).await?;
        let treasury = treasury::TreasuryStore::new(&db).await?;
        let subscriptions = subscription::SubscriptionsStore::new(&db);
        let history = history::HistoryStore::new(&db).await?;
        let requests = RequestStore::new(&db).await?;
        let notification = NotificationStore::new(&db).await?;

        Ok(Storage {
            db: Arc::new(db),
            users: Arc::new(users),
            calendar: Arc::new(calendar),
            treasury: Arc::new(treasury),
            subscriptions: Arc::new(subscriptions),
            history: Arc::new(history),
            requests: Arc::new(requests),
            notification: Arc::new(notification),
        })
    }
}
