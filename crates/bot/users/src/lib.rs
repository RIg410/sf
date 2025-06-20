use ::rights::Rule;
use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use bot_viewer::user::fmt_user_type;
use eyre::Error;
use mongodb::{SessionCursor, bson::oid::ObjectId};
use profile::UserProfile;
use serde::{Deserialize, Serialize};
use teloxide::{
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message},
    utils::markdown::escape,
};
use users::model::{User, sanitize_phone};

pub mod come_from;
pub mod comments;
pub mod family;
pub mod freeze;
pub mod history;
pub mod notification;
pub mod profile;
pub mod rewards;
pub mod rights;
pub mod set_ai_prompt;
pub mod set_birthday;
pub mod set_fio;
pub mod set_phone;
pub mod subscriptions;

pub const LIMIT: u64 = 7;

pub struct UsersView {
    query: Query,
}

impl UsersView {
    pub fn new(query: Query) -> UsersView {
        UsersView { query }
    }
}

#[async_trait]
impl View for UsersView {
    async fn show(&mut self, ctx: &mut Context) -> Result<(), eyre::Error> {
        let count = ctx
            .services
            .users
            .count(&mut ctx.session, self.query.only_with_subscriptions)
            .await?;
        let users = ctx
            .services
            .users
            .find(
                &mut ctx.session,
                &self.query.query,
                self.query.offset,
                LIMIT,
                Some(false),
                self.query.only_with_subscriptions,
            )
            .await?;

        let (txt, markup) = render_message(ctx, count, &self.query, users).await?;
        ctx.edit_origin(&txt, markup).await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, msg: &Message) -> ViewResult {
        ctx.delete_msg(msg.id).await?;
        ctx.ensure(Rule::ViewUsers)?;

        let mut query = msg.text().to_owned().unwrap_or_default().to_string();
        if query.len() == 1 && !query.chars().next().unwrap().is_alphanumeric() {
            query = "".to_string();
        }

        let phone = sanitize_phone(&query);
        let query = if !phone.is_empty() {
            phone
        } else {
            remove_non_alphanumeric(&query)
        };

        self.query = Query {
            query,
            offset: 0,
            only_with_subscriptions: false,
        };
        Ok(Jmp::Stay)
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        ctx.ensure(Rule::ViewUsers)?;

        match calldata!(data) {
            Callback::Next => {
                self.query.offset += LIMIT;
            }
            Callback::Prev => {
                self.query.offset = self.query.offset.saturating_sub(LIMIT);
            }
            Callback::Select(user_id) => {
                return Ok(UserProfile::new(ObjectId::from_bytes(user_id)).into());
            }
            Callback::OnlyWithSubscriptions => {
                self.query.only_with_subscriptions = !self.query.only_with_subscriptions;
            }
        }

        Ok(Jmp::Stay)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Query {
    pub query: String,
    pub offset: u64,
    pub only_with_subscriptions: bool,
}

impl Default for Query {
    fn default() -> Self {
        Query {
            query: "".to_string(),
            offset: 0,
            only_with_subscriptions: false,
        }
    }
}

async fn render_message(
    ctx: &mut Context,
    total_count: u64,
    query: &Query,
    mut users: SessionCursor<User>,
) -> Result<(String, InlineKeyboardMarkup), Error> {
    let msg = format!(
        "
    🟣 Всего пользователей: _{}_
    ➖➖➖➖➖➖➖➖➖➖
    🔵 \\- Инструктор
    🟢 \\- Клиент
    🔴 \\- Администратор 

    Что бы найти пользователя, воспользуйтесь поиском\\. Введите имя, фамилию или телефон пользователя\\.\n
    Запрос: _'{}'_
    ",
        total_count,
        escape(&query.query)
    );

    let mut keymap = InlineKeyboardMarkup::default();
    if query.only_with_subscriptions {
        keymap = keymap.append_row(vec![InlineKeyboardButton::callback(
            "Только с абонементами",
            Callback::OnlyWithSubscriptions.to_data(),
        )]);
    } else {
        keymap = keymap.append_row(vec![InlineKeyboardButton::callback(
            "Все пользователи",
            Callback::OnlyWithSubscriptions.to_data(),
        )]);
    }

    let mut users_count = 0;
    let mut ids = vec![];
    while let Some(user) = users.next(&mut ctx.session).await {
        let user = user?;
        if ids.contains(&user.id) {
            continue;
        }
        ids.push(user.id);

        keymap = keymap.append_row(vec![make_button(&user)]);
        users_count += 1;

        for child in user.family.children_ids.iter() {
            let child = ctx
                .services
                .users
                .get_user(&mut ctx.session, *child)
                .await?;
            if child.phone.is_none() {
                if ids.contains(&child.id) {
                    continue;
                }
                ids.push(child.id);
                keymap = keymap.append_row(vec![make_child_button(&user, &child)]);
            }
        }
    }

    let mut raw = vec![];

    if query.offset > 0 {
        raw.push(InlineKeyboardButton::callback(
            "⬅️",
            Callback::Prev.to_data(),
        ));
    }

    if users_count == LIMIT as usize {
        raw.push(InlineKeyboardButton::callback(
            "➡️",
            Callback::Next.to_data(),
        ));
    }

    if !raw.is_empty() {
        keymap = keymap.append_row(raw);
    }
    Ok((msg, keymap))
}

fn make_button(user: &User) -> InlineKeyboardButton {
    Callback::Select(user.id.bytes()).button(format!(
        "{}{} {}",
        fmt_user_type(user),
        user.name.first_name,
        user.name.last_name.as_ref().unwrap_or(&"".to_string())
    ))
}

fn make_child_button(parent: &User, child: &User) -> InlineKeyboardButton {
    InlineKeyboardButton::callback(
        format!(
            "{}{} {} ({})",
            fmt_user_type(child),
            parent.name.first_name,
            parent.name.last_name.as_ref().unwrap_or(&"".to_string()),
            child.name.first_name
        ),
        Callback::Select(child.id.bytes()).to_data(),
    )
}
#[derive(Debug, Serialize, Deserialize)]
enum Callback {
    Next,
    Prev,
    Select([u8; 12]),
    OnlyWithSubscriptions,
}

fn remove_non_alphanumeric(input: &str) -> String {
    input.chars().filter(|c| c.is_alphanumeric()).collect()
}
