use std::sync::Arc;

use crate::{SfServices, Task};
use async_trait::async_trait;
use booking::payer::{AvailableBalance as _, FindFor, SubscriptionResolver as _};
use bot_core::{CommonLocation, bot::TgBot};
use bot_viewer::{fmt_phone, user::link_to_user};
use employee::reward::EmployeeReward as _;
use eyre::{Error, Result, bail, eyre};
use mongodb::bson::oid::ObjectId;
use program::model::TrainingType;
use rewards::model::user::UserRewardContribution;
use rights::Rule;
use store::session::Session;
use subscription::model::UserSubscription;
use teloxide::{
    types::{ChatId, InlineKeyboardMarkup},
    utils::markdown::escape,
};
use tracing::{error, info};
use trainings::model::{Training, statistics::Statistics, status::TrainingStatus};
use tx_macro::tx;
use users::model::User;

#[derive(Clone)]
pub struct TriningBg {
    ledger: Arc<SfServices>,
    bot: Arc<TgBot>,
}

const SYSTEM_ID: ObjectId = ObjectId::from_bytes(*b"SF-SYSTEM-13");

#[async_trait]
impl Task for TriningBg {
    const NAME: &'static str = "training";
    const CRON: &'static str = "every 30 minutes";

    async fn process(&mut self) -> Result<(), Error> {
        let mut session = self.ledger.db.start_anonymous_session().await?;
        session.set_actor(SYSTEM_ID);

        let mut cursor = self.ledger.calendar.days_to_process(&mut session).await?;
        let now = chrono::Local::now();
        while let Some(day) = cursor.next(&mut session).await {
            let day = day?;
            for training in day.training {
                if training.is_processed {
                    continue;
                }
                let slot = training.get_slot();

                let result = match training.status(now) {
                    TrainingStatus::OpenToSignup { .. }
                    | TrainingStatus::ClosedToSignup
                    | TrainingStatus::InProgress => continue,
                    TrainingStatus::Finished => match training.tp {
                        TrainingType::Group { .. } | TrainingType::Personal { .. } => {
                            let notifications = self
                                .process_finished_training(&mut session, training)
                                .await?;
                            for notification in notifications {
                                self.send_notification(&mut session, notification).await?;
                            }
                            Ok(())
                        }
                        TrainingType::SubRent { .. } => {
                            self.process_finished_sub_rent(&mut session, training).await
                        }
                    },
                    TrainingStatus::Cancelled => {
                        if training.get_slot().start_at() < now {
                            self.process_canceled(&mut session, training).await
                        } else {
                            continue;
                        }
                    }
                };
                if let Err(err) = result {
                    error!("Failed to finalize: training:{:?}. {:?}", slot, err);
                }
            }
        }
        Ok(())
    }
}

impl TriningBg {
    pub fn new(ledger: Arc<SfServices>, bot: Arc<TgBot>) -> TriningBg {
        TriningBg { ledger, bot }
    }

    #[tx]
    async fn process_finished_sub_rent(
        &self,
        session: &mut Session,
        training: Training,
    ) -> Result<()> {
        info!("Finalize sub rent:{:?}", training);
        let mut statistic = Statistics::default();
        let (is_free, price) = match training.tp {
            TrainingType::SubRent { is_free, price } => (is_free, price),
            _ => bail!("Invalid training type"),
        };

        self.ledger
            .calendar
            .finalized(session, training.id(), &statistic)
            .await?;
        self.ledger
            .history
            .process_finished(session, &training)
            .await?;

        if !is_free {
            statistic.earned += price;
            self.ledger
                .treasury
                .sub_rent_txless(session, price, training.description)
                .await?;
        }

        Ok(())
    }

    #[tx]
    async fn process_finished_training(
        &self,
        session: &mut Session,
        training: Training,
    ) -> Result<Vec<Notification>> {
        info!("Finalize training:{:?}", training);
        let slot = training.get_slot();

        let mut notifications = vec![];

        let mut statistic = Statistics::default();

        let mut users_info = Vec::with_capacity(training.clients.len());
        if training.tp.is_not_free() {
            for client in &training.clients {
                let mut user = self.ledger.users.get_user(session, *client).await?;
                let mut payer = user.payer_mut()?;
                let sub = if let Some(sub) = payer.find_subscription(FindFor::Charge, &training) {
                    if !sub.change_locked_balance(&slot) {
                        return Err(eyre!("Not enough balance:{}", user.id));
                    }
                    statistic.earned += sub.item_price();
                    users_info.push(UserRewardContribution {
                        user: *client,
                        lesson_price: sub.item_price(),
                        subscription_price: sub.subscription_price(),
                        lessons_count: sub.items(),
                    });
                    if sub.balance == 0 {
                        Some(sub.clone())
                    } else {
                        None
                    }
                } else {
                    return Err(eyre!("Subscription not found for user:{}", user.id));
                };

                self.ledger.users.update(session, &mut payer).await?;
                if let Some(sub) = sub {
                    if let Some(notification) = self.user_notification(&mut user, sub, &training)? {
                        notifications.push(notification);
                    }
                }
            }
            let mut couch = self
                .ledger
                .users
                .get_user(session, training.instructor)
                .await?;
            if let Some(couch_info) = couch.employee.as_mut() {
                if let Some(reward) = couch_info.collect_training_rewards(&training, users_info)? {
                    statistic.couch_rewards += reward.reward;
                    self.ledger.rewards.add_reward(session, reward).await?;
                    self.ledger
                        .users
                        .update_employee_reward_and_rates(
                            session,
                            training.instructor,
                            couch_info.reward,
                            None,
                        )
                        .await?;
                }
            } else {
                bail!("Failed to process training. Failed to find instructor");
            }
        }
        self.ledger
            .calendar
            .finalized(session, training.id(), &statistic)
            .await?;
        self.ledger
            .history
            .process_finished(session, &training)
            .await?;
        Ok(notifications)
    }

    #[tx]
    async fn process_canceled(&self, session: &mut Session, training: Training) -> Result<()> {
        info!("Finalize canceled training:{:?}", training);

        self.ledger
            .calendar
            .finalized(session, training.id(), &Statistics::default())
            .await?;
        self.ledger
            .history
            .process_canceled(session, &training)
            .await?;
        Ok(())
    }

    fn user_notification(
        &self,
        user: &mut User,
        sub: UserSubscription,
        training: &Training,
    ) -> Result<Option<Notification>> {
        let payer = user.payer()?;
        let balance = payer.available_balance_for_training(training);

        // Notify user and manager if balance is zero and user has more than one lesson in subscription.
        if balance == 0 && sub.locked_balance == 0 && sub.items > 1 {
            Ok(Some(Notification {
                to_user: (
                    "Ваш абонемент закончился🥺".to_string(),
                    ChatId(payer.as_ref().tg_id),
                ),
                to_manager: (
                    format!(
                        "У {} {} закончился абонемент {}\\.",
                        link_to_user(payer.as_ref()),
                        fmt_phone(payer.as_ref().phone.as_deref()),
                        escape(&sub.name)
                    ),
                    InlineKeyboardMarkup::default()
                        .append_row(vec![CommonLocation::Profile(payer.as_ref().id).button()]),
                ),
            }))
        } else {
            Ok(None)
        }
    }

    async fn send_notification(
        &self,
        session: &mut Session,
        notification: Notification,
    ) -> Result<()> {
        self.bot
            .notify(notification.to_user.1, &notification.to_user.0, false)
            .await;

        let users = self
            .ledger
            .users
            .find_users_with_right(session, Rule::ReceiveNotificationsAboutSubscriptions)
            .await?;

        for user in users {
            self.bot
                .notify_with_markup(
                    ChatId(user.tg_id),
                    &notification.to_manager.0,
                    notification.to_manager.1.clone(),
                )
                .await;
        }

        Ok(())
    }
}

struct Notification {
    to_user: (String, ChatId),
    to_manager: (String, InlineKeyboardMarkup),
}
