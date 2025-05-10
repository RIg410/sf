use crate::models::advertising::{AdvertisingConversionStat, SourceStat};
use ai::{Ai, AiContext, AiModel};
use eyre::{Context, Error, Result};
use history::{
    model::{Action, ActionType},
    service::History,
};
use ident::source::Source;
use requests::service::Requests;
use std::collections::HashMap;
use store::session::Session;
use time::range::MonthRange;
use users::{log::UserLog, service::Users};

pub struct AdvertisingStatService<L> {
    requests: Requests<L>,
    users: Users<L>,
    history: History,
    ai: Ai,
}

impl<L: UserLog> AdvertisingStatService<L> {
    pub fn new(requests: Requests<L>, users: Users<L>, history: History, ai: Ai) -> Self {
        Self {
            requests,
            users,
            history,
            ai,
        }
    }

    pub async fn conversion(
        &self,
        session: &mut Session,
        range: MonthRange,
        ai: Option<AiModel>,
    ) -> Result<AdvertisingConversionStat, Error> {
        let (from, to) = range.range()?;

        let mut requests = self
            .requests
            .find_range(session, Some(from), Some(to))
            .await?;

        let mut stat = AdvertisingConversionStat::default();

        while let Some(request) = requests.next(session).await {
            let request = request?;
            let come_from = request.come_from;
            let phone = request.phone;

            let source_stat = stat.sources.entry(come_from).or_default();
            source_stat.processed_requests += 1;
            self.collect_users_stat(session, source_stat, phone).await?;
        }

        for source in stat.sources.values_mut() {
            source.calculate_conversions();
        }

        if let Some(model) = ai {
            stat.ai_comment = Some(self.ask_ai_stat(&mut stat.sources, model).await?);
        }

        Ok(stat)
    }

    async fn collect_users_stat(
        &self,
        session: &mut Session,
        stat: &mut SourceStat,
        phone: String,
    ) -> Result<(), Error> {
        let user = self.users.find_by_phone(session, &phone).await?;

        if let Some(user) = user {
            let mut history = self
                .history
                .actor_logs(session, user.id, None, 0, vec![ActionType::SellSub])
                .await?;

            let mut count = 0;
            while let Some(row) = history.next(session).await {
                let row = row?;

                match row.action {
                    Action::SellSub { .. } => {
                        count += 1;
                    }
                    _ => {
                        // no-op
                    }
                }
            }

            if count == 1 {
                stat.trial_visits += 1;
            } else if count > 1 {
                stat.trial_visits += 1;
                stat.bought_memberships += 1;
            }
        }
        Ok(())
    }

    async fn ask_ai_stat(
        &self,
        sources: &mut HashMap<Source, SourceStat>,
        model: AiModel,
    ) -> Result<String, Error> {
        let prompt = "Вот агригация конверсии клиентов по источникам в формате json. 
        Расскажи мне, что ты думаешь об этом. Ответ должен быть на русском языке, без форматирование, только переносы строк.
        Пришли только результат без размышлений.
        Не используй markdown разметку и символы, которые могут быть интерпретированы как разметка или запрещенные символы в markdown.
        \n\n";

        let sources: Vec<(&'static str, &SourceStat)> = sources
            .iter()
            .map(|(source, stat)| (source.name(), stat))
            .collect();

        let sources = serde_json::to_string(&sources).context("Failed to serialize sources")?;
        let prompt = format!("{prompt}\n{sources}");
        let result = self
            .ai
            .ask(model, prompt, &mut AiContext::default())
            .await?;
        Ok(result.response)
    }
}
