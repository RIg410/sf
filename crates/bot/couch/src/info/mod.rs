use async_trait::async_trait;
use bot_core::{
    context::Context,
    script::{
        Dispatch, ScriptView, Stage,
        list::{ListId, ListItem, StageList},
    },
    widget::Widget,
};
use bot_trainings::view::TrainingView;
use bot_viewer::{day::fmt_weekday, training::fmt_training_status};
use chrono::{DateTime, Datelike, Local};
use eyre::{Error, Result};
use mongodb::bson::oid::ObjectId;
use rights::Rule;
use teloxide::utils::markdown::escape;
use trainings::model::{Filter, Training};

mod edit_description;

pub fn couch_view(id: ObjectId) -> Widget {
    ScriptView::new(State { id }, Stage::list(CouchInfo)).into()
}

struct State {
    id: ObjectId,
}

struct CouchInfo;

impl CouchInfo {
    pub async fn change_description(
        &self,
        ctx: &mut Context,
        _: &mut State,
    ) -> Result<Dispatch<State>> {
        ctx.ensure(Rule::EditCouch)?;
        Ok(Dispatch::Stage(Stage::text(
            edit_description::CouchDescription,
        )))
    }

    pub async fn delete_couch(
        &self,
        ctx: &mut Context,
        state: &mut State,
    ) -> Result<Dispatch<State>> {
        ctx.ensure(Rule::EditCouch)?;
        ctx.services
            .employee
            .delete_employee(&mut ctx.session, state.id)
            .await?;
        ctx.send_notification("🗑️ Удалено").await;
        Ok(Dispatch::None)
    }
}

#[async_trait]
impl StageList<State> for CouchInfo {
    async fn message(
        &self,
        ctx: &mut Context,
        state: &mut State,
        limit: usize,
        offset: usize,
    ) -> Result<(String, Vec<Vec<ListItem>>)> {
        let user = ctx
            .services
            .users
            .get_user(&mut ctx.session, state.id)
            .await?;
        let couch = if let Some(couch) = user.employee.as_ref() {
            couch
        } else {
            return Err(eyre::eyre!("User is not a couch"));
        };

        let msg = format!(
            "💪{}\n📝[Обо мне]({})\n",
            escape(&user.name.to_string()),
            escape(&couch.description)
        );
        let trainings = ctx
            .services
            .calendar
            .find_trainings(&mut ctx.session, Filter::Instructor(user.id), limit, offset)
            .await?;

        let now = Local::now();
        let mut row = trainings
            .into_iter()
            .map(|training| vec![make_item(training, ctx, now)])
            .collect::<Vec<Vec<ListItem>>>();

        if ctx.has_right(Rule::EditCouch) {
            row.push(vec![Action::ChangeDescription.button()]);
            row.push(vec![Action::DeleteCouch.button()]);
        }

        Ok((msg, row))
    }

    async fn select(
        &self,
        ctx: &mut Context,
        state: &mut State,
        id: ListId,
    ) -> Result<Dispatch<State>, Error> {
        match id {
            ListId::TrainingId(id) => Ok(Dispatch::Widget(TrainingView::new(id.into()).into())),
            ListId::I64(id) => {
                let action = Action::try_from(ListId::I64(id))?;
                match action {
                    Action::ChangeDescription => self.change_description(ctx, state).await,
                    Action::DeleteCouch => self.delete_couch(ctx, state).await,
                }
            }
            _ => Err(eyre::eyre!("Invalid id")),
        }
    }
}

fn make_item(training: Training, ctx: &mut Context, now: DateTime<Local>) -> ListItem {
    let start_at = training.get_slot().start_at();
    ListItem {
        id: ListId::TrainingId(training.id().into()),
        name: format!(
            "{} {} {} {}",
            fmt_training_status(
                training.status(now),
                training.is_processed,
                training.is_full(),
                training.clients.contains(&ctx.me.id)
            ),
            fmt_weekday(start_at.weekday()),
            start_at.format("%d.%m %H:%M"),
            training.name.as_str(),
        ),
    }
}

pub enum Action {
    ChangeDescription,
    DeleteCouch,
}

impl Action {
    fn button(&self) -> ListItem {
        match self {
            Self::ChangeDescription => ListItem {
                id: ListId::I64(0),
                name: "✏️ Изменить описание".to_string(),
            },
            Self::DeleteCouch => ListItem {
                id: ListId::I64(1),
                name: "🗑 Удалить профиль".to_string(),
            },
        }
    }
}

impl TryFrom<ListId> for Action {
    type Error = Error;

    fn try_from(value: ListId) -> Result<Self> {
        match value {
            ListId::I64(0) => Ok(Self::ChangeDescription),
            ListId::I64(1) => Ok(Self::DeleteCouch),
            _ => Err(eyre::eyre!("Invalid id")),
        }
    }
}
