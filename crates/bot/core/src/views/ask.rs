use crate::{
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use async_trait::async_trait;
use std::str::FromStr;
use teloxide::types::{InlineKeyboardMarkup, Message};

#[async_trait]
pub trait AskView<T: FromStr + Send + Sync + 'static> {
    const ERROR_MESSAGE: &'static str;

    async fn message(&self, ctx: &mut Context) -> eyre::Result<String>;
    async fn on_answer(&self, ctx: &mut Context, value: T) -> ViewResult;
}

impl<T, V> From<V> for AskViewWidget<T, V>
where
    T: FromStr + Send + Sync + 'static,
    V: AskView<T> + Send + Sync + 'static,
{
    fn from(view: V) -> Self {
        AskViewWidget {
            view,
            _marker: std::marker::PhantomData,
        }
    }
}

pub struct AskViewWidget<T, V>
where
    T: FromStr + Send + Sync + 'static,
    V: AskView<T> + Send + Sync + 'static,
{
    view: V,
    _marker: std::marker::PhantomData<T>,
}

impl<T, V> AskViewWidget<T, V>
where
    T: FromStr + Send + Sync + 'static,
    V: AskView<T> + Send + Sync + 'static,
{
    pub fn new(view: V) -> Self {
        AskViewWidget {
            view,
            _marker: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<V, T> View for AskViewWidget<T, V>
where
    T: FromStr + Send + Sync + 'static,
    V: AskView<T> + Send + Sync + 'static,
{
    fn name(&self) -> &'static str {
        "Ask"
    }

    async fn show(&mut self, ctx: &mut Context) -> eyre::Result<()> {
        let keymap = InlineKeyboardMarkup::default();
        let msg = self.view.message(ctx).await?;
        ctx.edit_origin(&msg, keymap).await?;
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, msg: &Message) -> ViewResult {
        ctx.delete_msg(msg.id).await?;
        let value = match msg.text() {
            Some(text) => T::from_str(text).map_err(|_| ()),
            None => {
                ctx.send_notification("Не удалось получить текст сообщения")
                    .await;
                return Ok(Jmp::Stay);
            }
        };

        let value = match value {
            Ok(val) => val,
            Err(_) => {
                ctx.send_notification(V::ERROR_MESSAGE).await;
                return Ok(Jmp::Stay);
            }
        };

        self.view.on_answer(ctx, value).await
    }
}
