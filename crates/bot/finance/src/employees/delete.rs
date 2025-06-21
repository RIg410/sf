use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use mongodb::bson::oid::ObjectId;
use rights::Rule;
use serde::{Deserialize, Serialize};
use teloxide::types::InlineKeyboardMarkup;

pub struct DeleteEmployeeConfirm {
    pub user_id: ObjectId,
}

impl DeleteEmployeeConfirm {
    pub fn new(user_id: ObjectId) -> Self {
        Self { user_id }
    }
}

#[async_trait]
impl View for DeleteEmployeeConfirm {
    async fn show(&mut self, ctx: &mut Context) -> Result<(), eyre::Error> {
        let msg = "Вы уверены, что хотите удалить сотрудника?";
        let mut keymap = InlineKeyboardMarkup::default();
        keymap = keymap.append_row(vec![
            CallbackQuery::Yes.button("✅ Да"),
            CallbackQuery::No.button("❌ Нет"),
        ]);
        ctx.edit_origin(msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, callback: &str) -> ViewResult {
        ctx.ensure(Rule::EditEmployee)?;
        match calldata!(callback) {
            CallbackQuery::Yes => {
                ctx.services
                    .employee
                    .delete_employee(&mut ctx.session, self.user_id)
                    .await?;
                ctx.send_notification("Сотрудник удален").await;
                Ok(Jmp::Back(1))
            }
            CallbackQuery::No => Ok(Jmp::Back(1)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CallbackQuery {
    Yes,
    No,
}
