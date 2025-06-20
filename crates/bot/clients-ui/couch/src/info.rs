use bot_client_trainings::list::TrainingList;
use bot_core::{
    callback_data::Calldata as _, calldata, context::Context, widget::{View, ViewResult}
};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use teloxide::{types::InlineKeyboardMarkup, utils::markdown::escape};

pub struct CouchInfo {
    id: ObjectId,
}

impl CouchInfo {
    pub fn new(id: ObjectId) -> Self {
        Self { id }
    }
}

#[async_trait::async_trait]
impl View for CouchInfo {
    fn name(&self) -> &'static str {
        "CouchInfo"
    }

    fn safe_point(&self) -> bool {
        true
    }

    async fn show(&mut self, ctx: &mut Context) -> eyre::Result<()> {
        let user = ctx
            .services
            .users
            .get_user(&mut ctx.session, self.id)
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
        let keymap = InlineKeyboardMarkup::default().append_row(
            Callback::FindTraining.btn_row("Найти тренировку"),
        );
        ctx.edit_origin(&msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, _: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Callback::FindTraining => Ok(TrainingList::couches(self.id).into()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
enum Callback {
    FindTraining,
}
