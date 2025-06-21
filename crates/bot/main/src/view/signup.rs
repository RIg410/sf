use async_trait::async_trait;
use bot_core::{
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use eyre::Context as _;
use mongodb::bson::oid::ObjectId;
use services::SfServices;
use store::session::Session;
use teloxide::types::{
    ButtonRequest, Contact, KeyboardButton, KeyboardMarkup, KeyboardRemove, Message, ReplyMarkup,
};
use tracing::info;
use users::model::UserName;

const GREET_START: &str = "\nПожалуйста, оставьте ваш номер телефона\\. Для этого нажмите на кнопку ниже\\.\n\nОтправляя номер телефона, вы соглашаетесь на обработку ваших персональных данных\\.";

#[derive(Default)]
pub struct SignUpView;

#[async_trait]
impl View for SignUpView {
    async fn show(&mut self, ctx: &mut Context) -> Result<(), eyre::Error> {
        ctx.send_replay_markup(GREET_START, relay()).await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, msg: &Message) -> ViewResult {
        let from = if let Some(from) = &msg.from {
            from
        } else {
            ctx.bot.delete_msg(msg.id).await?;
            return Ok(Jmp::Stay);
        };

        if from.is_bot {
            ctx.send_msg("Бот работает только с людьми\\.").await?;
            return Ok(Jmp::Stay);
        }

        if let Some(contact) = msg.contact() {
            let id = create_user(
                &ctx.services,
                msg.chat.id.0,
                contact,
                from,
                &mut ctx.session,
            )
            .await
            .context("Failed to create user")?;
            ctx.send_replay_markup(
                "Добро пожаловать\\!",
                ReplyMarkup::KeyboardRemove(KeyboardRemove::new()),
            )
            .await?;
            ctx.me.id = id;
            ctx.reload_user().await?;
            return Ok(Jmp::Home);
        } else {
            Ok(Jmp::Stay)
        }
    }
}

pub async fn create_user(
    ledger: &SfServices,
    chat_id: i64,
    contact: &Contact,
    from: &teloxide::types::User,
    session: &mut Session,
) -> Result<ObjectId, eyre::Error> {
    info!("Creating user with chat_id: {}", chat_id);
    let user = ledger.users.get_by_tg_id(session, from.id.0 as i64).await?;
    if user.is_some() {
        return Err(eyre::eyre!("User {} already exists", chat_id));
    }

    let come_from = ledger
        .requests
        .come_from(session, &contact.phone_number)
        .await?;

    let id = ledger
        .users
        .create(
            session,
            chat_id,
            UserName {
                tg_user_name: from.username.clone(),
                first_name: from.first_name.clone(),
                last_name: from.last_name.clone(),
            },
            contact.phone_number.clone(),
            come_from,
        )
        .await
        .context("Failed to create user")?;
    Ok(id)
}

fn relay() -> ReplyMarkup {
    let keymap = KeyboardMarkup::new(vec![vec![
        KeyboardButton::new("📱 Отправить номер").request(ButtonRequest::Contact),
    ]]);
    ReplyMarkup::Keyboard(keymap.one_time_keyboard())
}
