use std::sync::Arc;

use crate::{SfServices, Task};
use async_trait::async_trait;
use chrono::Local;
use employee::reward::EmployeeReward as _;
use eyre::Error;
use store::session::Session;
use tracing::info;
use tx_macro::tx;

#[derive(Clone)]
pub struct RewardsBg {
    ledger: Arc<SfServices>,
}

#[async_trait]
impl Task for RewardsBg {
    const NAME: &'static str = "rewards";
    const CRON: &'static str = "every 5 hour";

    async fn process(&mut self) -> Result<(), Error> {
        let mut session = self.ledger.db.start_session().await?;
        self.process_rewards(&mut session).await?;
        Ok(())
    }
}

impl RewardsBg {
    pub fn new(ledger: Arc<SfServices>) -> RewardsBg {
        RewardsBg { ledger }
    }

    #[tx]
    async fn process_rewards(&self, session: &mut Session) -> Result<(), Error> {
        info!("Processing rewards");
        let mut users = self
            .ledger
            .users
            .employees_with_ready_fix_reward(&mut *session)
            .await?;
        info!("Found {} users", users.len());
        let now = Local::now();
        for user in users.iter_mut() {
            if let Some(employee) = &mut user.employee {
                if let Some(reward) = employee.collect_fix_rewards(user.id, now)? {
                    info!("Added reward: {:?}", reward);

                    self.ledger
                        .rewards
                        .add_reward(&mut *session, reward)
                        .await?;
                    self.ledger
                        .users
                        .update_employee_reward_and_rates(
                            &mut *session,
                            user.id,
                            employee.reward,
                            Some(employee.rates.clone()),
                        )
                        .await?;
                }
            }
        }
        Ok(())
    }
}
