use eyre::Result;
use std::sync::Arc;
use storage::{
    calendar::CalendarStore, history::HistoryStore, notification::NotificationStore,
    program::ProgramStore, requests::RequestStore, subscription::SubscriptionsStore,
    treasury::TreasuryStore, user::UserStore,
};
use store::session::Db;

const DB_NAME: &str = "ledger_db";

#[derive(Clone)]
pub struct Storage {
    pub db: Arc<Db>,
    pub users: Arc<UserStore>,
    pub calendar: Arc<CalendarStore>,
    pub programs: Arc<ProgramStore>,
    pub treasury: Arc<TreasuryStore>,
    pub subscriptions: Arc<SubscriptionsStore>,
    pub history: Arc<HistoryStore>,
    pub requests: Arc<RequestStore>,
    pub notification: Arc<NotificationStore>,
}

impl Storage {
    pub async fn new(uri: &str) -> Result<Self> {
        let db = Db::new(uri, DB_NAME).await?;
        let users = UserStore::new(&db).await?;
        let calendar = CalendarStore::new(&db).await?;
        let programs = ProgramStore::new(&db);
        let treasury = TreasuryStore::new(&db).await?;
        let subscriptions = SubscriptionsStore::new(&db);
        let history = HistoryStore::new(&db).await?;
        let requests = RequestStore::new(&db).await?;
        let notification = NotificationStore::new(&db).await?;

        Ok(Storage {
            db: Arc::new(db),
            users: Arc::new(users),
            calendar: Arc::new(calendar),
            programs: Arc::new(programs),
            treasury: Arc::new(treasury),
            subscriptions: Arc::new(subscriptions),
            history: Arc::new(history),
            requests: Arc::new(requests),
            notification: Arc::new(notification),
        })
    }
}
