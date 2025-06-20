use super::{new::MakeEmployee, profile::EmployeeProfile};
use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use eyre::Result;
use mongodb::bson::oid::ObjectId;
use rights::Rule;
use serde::{Deserialize, Serialize};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use users::model::User;

pub struct EmployeeList {}

impl Default for EmployeeList {
    fn default() -> Self {
        Self::new()
    }
}

impl EmployeeList {
    pub fn new() -> EmployeeList {
        EmployeeList {}
    }
}

#[async_trait]
impl View for EmployeeList {
    async fn show(&mut self, ctx: &mut Context) -> Result<()> {
        let msg = "Сотрудники ❤️";
        let mut keymap = InlineKeyboardMarkup::default();
        let employee = ctx.services.users.employees(&mut ctx.session).await?;

        for instruct in employee {
            keymap = keymap.append_row(vec![render_button(
                &instruct,
                ctx.has_right(Rule::ViewRewards),
            )]);
        }

        if ctx.has_right(Rule::ViewEmployees) {
            keymap = keymap.append_row(Callback::Make.btn_row("Новый сотрудник 🔥"));
        }

        ctx.edit_origin(msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, _: &mut Context, data: &str) -> ViewResult {
        match calldata!(data) {
            Callback::Select(id) => Ok(Jmp::Next(
                EmployeeProfile::new(ObjectId::from_bytes(id)).into(),
            )),
            Callback::Make => Ok(Jmp::Next(MakeEmployee::new().into())),
        }
    }
}

#[derive(Serialize, Deserialize)]
enum Callback {
    Select([u8; 12]),
    Make,
}

fn render_button(user: &User, view_rewards: bool) -> InlineKeyboardButton {
    if view_rewards {
        Callback::Select(user.id.bytes()).button(format!(
            "{} {} ({} p)",
            user.name.first_name,
            user.name.last_name.clone().unwrap_or_default(),
            user.employee.as_ref().map(|c| c.reward).unwrap_or_default()
        ))
    } else {
        Callback::Select(user.id.bytes()).button(format!(
            "{} {}",
            user.name.first_name,
            user.name.last_name.clone().unwrap_or_default(),
        ))
    }
}
