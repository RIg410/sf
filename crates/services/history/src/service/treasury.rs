use treasury::service::log::TreasuryLog;

use super::History;

impl TreasuryLog for History {
    fn payment(
        &self,
        session: &mut store::session::Session,
        amount: decimal::Decimal,
        description: String,
        date_time: &chrono::DateTime<chrono::Local>,
    ) -> impl Future<Output = eyre::Result<()>> + Send {
        self.payment(session, amount, description, date_time)
    }

    fn deposit(
        &self,
        session: &mut store::session::Session,
        amount: decimal::Decimal,
        description: String,
        date_time: &chrono::DateTime<chrono::Local>,
    ) -> impl Future<Output = eyre::Result<()>> + Send {
        self.deposit(session, amount, description, date_time)
    }
}
