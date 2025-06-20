use super::View;
use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata,
    calldata,
    context::Context,
    widget::{Jmp, ViewResult},
};
use bot_viewer::user::fmt_user_type;
use mongodb::bson::oid::ObjectId;
use rights::Rule;
use serde::{Deserialize, Serialize};
use teloxide::types::InlineKeyboardMarkup;
use users::model::User;

#[derive(Default)]
pub struct UserRightsView {
    id: ObjectId,
}

impl UserRightsView {
    pub fn new(id: ObjectId) -> UserRightsView {
        UserRightsView { id }
    }
}

#[async_trait]
impl View for UserRightsView {

    async fn show(&mut self, ctx: &mut Context) -> Result<(), eyre::Error> {
        let user = ctx
            .services
            .users
            .get(&mut ctx.session, self.id)
            .await?
            .ok_or_else(|| eyre::eyre!("Failed to load user"))?;
        let (text, markup) = render_user_rights(&user);
        ctx.edit_origin(&text, markup).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        let cb = calldata!(data);
        match cb {
            Callback::EditRule(rule_id, is_active) => {
                ctx.ensure(Rule::EditUserRights)?;

                let rule = Rule::try_from(rule_id)?;
                ctx.services
                    .users
                    .edit_user_rule(&mut ctx.session, self.id, rule, is_active)
                    .await?;
                ctx.reload_user().await?;
                Ok(Jmp::Stay)
            }
        }
    }
}

fn render_user_rights(user: &User) -> (String, InlineKeyboardMarkup) {
    let mut msg = format!("{} 🔒Права:", fmt_user_type(user));
    let mut keymap = InlineKeyboardMarkup::default();

    if !user.rights.is_full() {
        for (rule, is_active) in user.rights.get_all_rules().iter() {
            keymap = keymap.append_row(Callback::EditRule(rule.id(), !is_active).btn_row(format!(
                "{} {}",
                rule.name(),
                if *is_active { "✅" } else { "❌" }
            )));
        }
    } else {
        msg.push_str("\n\nПользователь имеет права администратора");
    }

    (msg, keymap)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Callback {
    EditRule(u8, bool),
}
