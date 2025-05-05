use chrono::{DateTime, Local};
use decimal::Decimal;
use eyre::Result;
use store::session::Session;

pub trait TreasuryLog {
    fn payment(
        &self,
        session: &mut Session,
        amount: Decimal,
        description: String,
        date_time: &DateTime<Local>,
    ) -> impl Future<Output = Result<()>> + Send;

    fn deposit(
        &self,
        session: &mut Session,
        amount: Decimal,
        description: String,
        date_time: &DateTime<Local>,
    ) -> impl Future<Output = Result<()>> + Send;
}
