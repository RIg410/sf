use async_trait::async_trait;
use bot_core::{context::Context, widget::View};
use eyre::Result;
use teloxide::{types::InlineKeyboardMarkup, utils::markdown::escape};

#[derive(Default)]
pub struct ShopView;

#[async_trait]
impl View for ShopView {
    async fn show(&mut self, ctx: &mut Context) -> Result<(), eyre::Error> {
        let mut msg = "💪 Тарифы:\n\n".to_string();

        let keymap = InlineKeyboardMarkup::default();
        let subscriptions = ctx.services.subscriptions.get_all(&mut ctx.session).await?;

        let delimiter = escape("-------------------------\n");
        msg.push_str(&delimiter);
        msg.push_str("_Групповые абонементы:_\n");

        for subscription in subscriptions
            .iter()
            .filter(|s| !s.subscription_type.is_personal())
        {
            if !subscription.can_user_buy() {
                continue;
            }
            msg.push_str(&format!(
                "*{}* \\- _{}_р\n",
                escape(&subscription.name),
                subscription.price.to_string().replace(".", ",")
            ));
        }
        msg.push_str(&delimiter);
        msg.push_str("_Индивидуальные абонементы:_\n");

        for subscription in subscriptions
            .iter()
            .filter(|s| s.subscription_type.is_personal())
        {
            if !subscription.can_user_buy() {
                continue;
            }

            msg.push_str(&format!(
                "*{}* \\- _{}_р\n",
                escape(&subscription.name),
                subscription.price.to_string().replace(".", ",")
            ));
        }
        ctx.edit_origin(&msg, keymap).await?;
        Ok(())
    }
}
