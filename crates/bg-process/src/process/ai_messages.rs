use crate::Task;
use async_trait::async_trait;
use bot_core::bot::TgBot;
use eyre::Error;
use rights::Rule;
use services::SfServices;
use std::sync::Arc;
use teloxide::{types::ChatId, utils::markdown::escape};

#[derive(Clone)]
pub struct MotivationNotifier {
    pub ledger: Arc<SfServices>,
    pub bot: Arc<TgBot>,
}

impl MotivationNotifier {
    pub fn new(ledger: Arc<SfServices>, bot: Arc<TgBot>) -> MotivationNotifier {
        MotivationNotifier { ledger, bot }
    }
}

#[async_trait]
impl Task for MotivationNotifier {
    const NAME: &'static str = "motivation-notifier";
    const CRON: &'static str = "every 1 day at 9:12";

    async fn process(&mut self) -> Result<(), Error> {
        let mut session = self.ledger.db.start_anonymous_session().await?;

        let notification_listener = self
            .ledger
            .users
            .find_users_with_right(&mut session, Rule::ReceiveAiNotifications)
            .await?;

        for user in notification_listener {
            let extension = self
                .ledger
                .users
                .get_extension(&mut session, user.id)
                .await?;
            if let Some(prompt) = extension.ai_message_prompt {
                if let Ok(response) = self
                    .ledger
                    .ai
                    .ask(ai::AiModel::Gpt4oMini, prompt, &mut Default::default())
                    .await
                {
                    self.bot
                        .notify(ChatId(user.tg_id), &escape(&response.response), false)
                        .await;
                }
            }
        }

        Ok(())
    }
}
