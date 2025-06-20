use async_trait::async_trait;
use bot_core::{
    callback_data::Calldata as _,
    calldata,
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use ident::source::Source;
use mongodb::bson::oid::ObjectId;
use rights::Rule;
use teloxide::types::InlineKeyboardMarkup;
use users::model::sanitize_phone;

pub struct MarketingInfoView {
    id: ObjectId,
}

impl MarketingInfoView {
    pub fn new(id: ObjectId) -> Self {
        MarketingInfoView { id }
    }
}

#[async_trait]
impl View for MarketingInfoView {
    async fn show(&mut self, ctx: &mut Context) -> Result<(), eyre::Error> {
        ctx.ensure(Rule::EditMarketingInfo)?;
        let user = ctx
            .services
            .users
            .get_user(&mut ctx.session, self.id)
            .await?;

        let txt = format!("Источник : _{}_\n", user.as_client()?.come_from.name());
        let mut markup = InlineKeyboardMarkup::default();
        for come_from in Source::iter() {
            markup = markup.append_row(come_from.btn_row(come_from.name()));
        }
        ctx.edit_origin(&txt, markup).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        ctx.ensure(Rule::EditMarketingInfo)?;
        let come_from = calldata!(data);

        let user = ctx
            .services
            .users
            .get_user(&mut ctx.session, self.id)
            .await?;
        if let Some(phone) = user.phone {
            let request = ctx
                .services
                .requests
                .get_by_phone(&mut ctx.session, &sanitize_phone(&phone))
                .await?;
            if let Some(mut request) = request {
                request.source = come_from;
                ctx.services
                    .requests
                    .update(&mut ctx.session, &request)
                    .await?;
            }
        }

        ctx.services
            .users
            .update_come_from(&mut ctx.session, self.id, come_from)
            .await?;
        Ok(Jmp::Stay)
    }
}
