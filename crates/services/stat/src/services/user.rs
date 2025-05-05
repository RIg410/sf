use crate::models::user::{Statistics, SubscriptionStat};
use bson::oid::ObjectId;
use chrono::{Duration, Local};
use decimal::Decimal;
use history::{model::Action, service::History};
use store::session::Session;
use trainings::model::CLOSE_SING_UP;

pub struct UserStat {
    logs: History,
}

impl UserStat {
    pub fn new(logs: History) -> Self {
        Self { logs }
    }

    pub async fn collect_statistics(
        &self,
        session: &mut Session,
        user: &ObjectId,
    ) -> Result<Statistics, eyre::Error> {
        let mut statistics = Statistics::default();
        let mut history = self
            .logs
            .get_actor_logs(session, *user, None, 0, vec![])
            .await?;

        while let Some(row) = history.next(session).await {
            let row = row?;
            match row.action {
                Action::RemoveFamilyMember {}
                | Action::AddFamilyMember {}
                | Action::PayReward { .. }
                | Action::Unfreeze {}
                | Action::Deposit { .. }
                | Action::CreateUser { .. }
                | Action::Payment { .. }
                | Action::FinalizedCanceledTraining { .. }
                | Action::PreSellSub { .. }
                | Action::SignUp { .. }
                | Action::BlockUser { .. } => {
                    //no-op
                }
                Action::SignOut { start_at, name, .. } => {
                    if row.date_time.with_timezone(&Local) + Duration::minutes(CLOSE_SING_UP as i64)
                        > start_at
                    {
                        statistics
                            .training
                            .entry(name)
                            .or_default()
                            .cancellations_count += 1;
                    }
                }
                Action::SellSub {
                    subscription,
                    discount,
                } => {
                    let stat = statistics
                        .subscriptions
                        .entry(subscription.id)
                        .or_insert_with(|| SubscriptionStat::new(subscription.name.clone()));
                    stat.soult_count += 1;

                    if let Some(discount) = discount {
                        let discount_sum = subscription.price * discount;
                        stat.discount += discount_sum;
                        stat.spent += subscription.price - discount_sum;
                    } else {
                        stat.spent += subscription.price;
                    }
                }
                Action::FinalizedTraining { name, .. } => {
                    let training = statistics.training.entry(name).or_default();
                    training.count += 1;
                }
                Action::Freeze { days } => {
                    statistics.total_freeze += days;
                }
                Action::ChangeBalance { amount } => {
                    statistics.changed_subscription_balance += amount as i64;
                }
                Action::ChangeReservedBalance { amount } => {
                    statistics.changed_subscription_balance += amount as i64;
                }
                Action::ChangeSubscriptionDays { delta } => {
                    statistics.changed_subscription_days += delta as i64;
                }
                Action::ExpireSubscription { subscription } => {
                    let stat = statistics
                        .subscriptions
                        .entry(subscription.subscription_id)
                        .or_insert_with(|| SubscriptionStat::new(subscription.name.clone()));

                    stat.expired_sum +=
                        subscription.item_price() * Decimal::int(subscription.balance as i64);
                    stat.expired_trainings += subscription.balance as u64;
                }
            }
        }

        Ok(statistics)
    }
}
