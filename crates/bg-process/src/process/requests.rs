use crate::Task;
use async_trait::async_trait;
use bot_core::{CommonLocation, bot::TgBot};
use bot_viewer::request::fmt_request;
use chrono::Local;
use eyre::Error;
use requests::model::Request;
use services::SfServices;
use std::sync::Arc;
use store::session::Session;
use teloxide::types::{ChatId, InlineKeyboardMarkup};
use tx_macro::tx;

#[derive(Clone)]
pub struct RequestNotifier {
    pub ledger: Arc<SfServices>,
    pub bot: Arc<TgBot>,
}

#[async_trait]
impl Task for RequestNotifier {
    const NAME: &'static str = "request_notifier";
    const CRON: &'static str = "every 30 minutes";

    async fn process(&mut self) -> Result<(), Error> {
        let mut session = self.ledger.db.start_anonymous_session().await?;
        let mut requests = self.ledger.requests.to_notify(&mut session).await?;
        for req in requests.as_mut_slice() {
            if let Some(remind_later) = &req.remind_later {
                if remind_later.date_time.with_timezone(&Local) > Local::now() {
                    continue;
                }
                let user = self
                    .ledger
                    .users
                    .get_user(&mut session, remind_later.user_id)
                    .await?;
                self.notify(&self.ledger, &mut session, user.tg_id, req)
                    .await?;
            }
        }
        Ok(())
    }
}

impl RequestNotifier {
    pub fn new(ledger: Arc<SfServices>, bot: Arc<TgBot>) -> RequestNotifier {
        RequestNotifier { ledger, bot }
    }

    #[tx]
    async fn notify(
        &self,
        ledger: &SfServices,
        session: &mut Session,
        user: i64,
        request: &mut Request,
    ) -> Result<(), Error> {
        let msg = format!("Напоминание по заявке\n{}", fmt_request(request));
        let id = self
            .bot
            .notify_with_markup(
                ChatId(user),
                &msg,
                InlineKeyboardMarkup::default()
                    .append_row(vec![CommonLocation::Request(request.id).button()]),
            )
            .await;
        self.bot.pin_message(ChatId(user), id).await?;
        request.remind_later = None;
        ledger.requests.update(&mut *session, request).await?;
        Ok(())
    }
}
