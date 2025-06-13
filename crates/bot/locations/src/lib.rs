use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use eyre::Error;
use locations::model::Location;
use mongodb::bson::oid::ObjectId;
use rights::Rule;
use serde::{Deserialize, Serialize};
use teloxide::{
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message},
    utils::markdown::escape,
};

pub mod create;
pub mod edit;
pub mod view;

pub const LIMIT: u64 = 10;

pub struct LocationsView {
    query: String,
    offset: u64,
}

impl LocationsView {
    pub fn new() -> Self {
        LocationsView {
            query: String::new(),
            offset: 0,
        }
    }
}

impl Default for LocationsView {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl View for LocationsView {
    fn name(&self) -> &'static str {
        "LocationsView"
    }

    async fn show(&mut self, ctx: &mut Context) -> Result<(), Error> {
        let locations = ctx.services.locations.get_all(&mut ctx.session).await?;

        let (txt, markup) = render_locations_list(ctx, &locations).await?;
        ctx.edit_origin(&txt, markup).await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, msg: &Message) -> ViewResult {
        ctx.delete_msg(msg.id).await?;
        ctx.ensure(Rule::System)?;

        if let Some(query) = msg.text() {
            self.query = query.to_string();
            self.offset = 0;
        }

        Ok(Jmp::Stay)
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        ctx.ensure(Rule::System)?;

        match calldata!(data) {
            Callback::Create => Ok(create::CreateLocationView.into()),
            Callback::Select(location_id) => {
                Ok(view::LocationDetailView::new(ObjectId::from_bytes(location_id)).into())
            }
            Callback::Edit(location_id) => {
                Ok(edit::EditLocationView::new(ObjectId::from_bytes(location_id)).into())
            }
        }
    }
}

async fn render_locations_list(
    _ctx: &mut Context,
    locations: &Vec<Location>,
) -> Result<(String, InlineKeyboardMarkup), Error> {
    let mut msg = "🏢 *Управление локациями*\n\n".to_string();

    if locations.is_empty() {
        msg.push_str("Локации не найдены\\.\n");
    } else {
        msg.push_str(&format!("Всего локаций: *{}*\n\n", locations.len()));

        for location in locations.iter().take(10) {
            msg.push_str(&format!(
                "📍 *{}*\n📮 _{}_\n🚪 Залов: *{}*\n\n",
                escape(&location.name),
                escape(&location.address),
                location.halls.len()
            ));
        }
    }

    let mut keymap = InlineKeyboardMarkup::default();

    // Add location buttons
    for location in locations.iter().take(10) {
        keymap = keymap.append_row(vec![InlineKeyboardButton::callback(
            format!("📍 {}", location.name),
            Callback::Select(location.id.bytes()).to_data(),
        )]);
    }

    // Add create button
    keymap = keymap.append_row(vec![InlineKeyboardButton::callback(
        "➕ Создать локацию",
        Callback::Create.to_data(),
    )]);

    Ok((msg, keymap))
}

#[derive(Debug, Serialize, Deserialize)]
enum Callback {
    Create,
    Select([u8; 12]),
    Edit([u8; 12]),
}
