use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use mongodb::bson::oid::ObjectId;
use teloxide::types::{InlineKeyboardMarkup, Message};
use users::model::{rate::EmployeeRole, sanitize_phone};

pub struct MakeEmployee {}

impl Default for MakeEmployee {
    fn default() -> Self {
        Self::new()
    }
}

impl MakeEmployee {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl View for MakeEmployee {
    async fn show(&mut self, ctx: &mut Context) -> Result<(), eyre::Error> {
        let msg = "Введите номер телефона нового сотрудника:";
        let keymap = InlineKeyboardMarkup::default();
        ctx.edit_origin(msg, keymap).await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, message: &Message) -> ViewResult {
        ctx.delete_msg(message.id).await?;
        let phone = if let Some(phone) = message.text() {
            sanitize_phone(phone)
        } else {
            ctx.send_notification("Номер телефона не найден").await;
            return Ok(Jmp::Stay);
        };

        let user = ctx
            .services
            .users
            .find_by_phone(&mut ctx.session, &phone)
            .await?;

        Ok(if let Some(user) = user {
            if user.employee.is_some() {
                ctx.send_notification("Пользователь уже является сотрудником")
                    .await;
                Jmp::Stay
            } else {
                Jmp::Next(EmployeeDescription { user_id: user.id }.into())
            }
        } else {
            ctx.send_notification("Пользователь не найден").await;
            Jmp::Stay
        })
    }
}

pub struct EmployeeDescription {
    user_id: ObjectId,
}

#[async_trait]
impl View for EmployeeDescription {
    async fn show(&mut self, ctx: &mut Context) -> Result<(), eyre::Error> {
        let msg = "Введите описание нового сотрудника:";
        let keymap = InlineKeyboardMarkup::default();
        ctx.edit_origin(msg, keymap).await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, message: &Message) -> ViewResult {
        ctx.delete_msg(message.id).await?;
        Ok(if let Some(description) = message.text() {
            Jmp::Next(
                EmployeeRoleView {
                    user_id: self.user_id,
                    description: description.to_string(),
                }
                .into(),
            )
        } else {
            ctx.send_notification("Описание не найдено").await;
            Jmp::Stay
        })
    }
}

pub struct EmployeeRoleView {
    user_id: ObjectId,
    description: String,
}

#[async_trait]
impl View for EmployeeRoleView {
    async fn show(&mut self, ctx: &mut Context) -> Result<(), eyre::Error> {
        let msg = "Выберите роль нового сотрудника:";
        let mut keymap = InlineKeyboardMarkup::default();
        keymap = keymap.append_row(EmployeeRole::Manager.btn_row("Менеджер"));
        keymap = keymap.append_row(EmployeeRole::Couch.btn_row("Тренер"));
        keymap = keymap.append_row(EmployeeRole::Admin.btn_row("Администратор"));
        ctx.edit_origin(msg, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        let role: EmployeeRole = calldata!(data);
        ctx.services
            .employee
            .make_user_employee(
                &mut ctx.session,
                self.user_id,
                self.description.clone(),
                vec![],
                role,
            )
            .await?;
        ctx.send_notification("Сотрудник добавлен").await;
        Ok(Jmp::Home)
    }
}
