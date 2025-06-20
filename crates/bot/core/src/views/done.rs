use std::borrow::Cow;

use crate::{
    callback_data::Calldata as _,
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use async_trait::async_trait;
use eyre::Error;
use serde::{Deserialize, Serialize};
use teloxide::types::InlineKeyboardMarkup;

pub struct DoneView {
    pub text: Cow<'static, str>,
    pub err: bool,
    pub back: bool,
}

impl DoneView {
    pub fn ok<T: Into<Cow<'static, str>>>(text: T) -> Self {
        Self {
            text: text.into(),
            err: false,
            back: false,
        }
    }

    pub fn err<T: Into<Cow<'static, str>>>(text: T) -> Self {
        Self {
            text: text.into(),
            err: true,
            back: false,
        }
    }

    pub fn go_back(self) -> Self {
        Self { back: true, ..self }
    }
}

#[async_trait]
impl View for DoneView {

    async fn show(&mut self, ctx: &mut Context) -> Result<(), Error> {
        ctx.set_system_go_back(false);
        ctx.set_system_go_home(false);
        let mut keymap = InlineKeyboardMarkup::default();
        if self.err {
            keymap = keymap.append_row(vec![OkCallback::Ok.button("😢 Окай")]);
        } else {
            keymap = keymap.append_row(vec![OkCallback::Ok.button("✅ ОК")]);
        }

        ctx.edit_origin(&self.text, keymap).await?;
        Ok(())
    }

    async fn handle_callback(&mut self, ctx: &mut Context, data: &str) -> ViewResult {
        let cb: OkCallback = if let Some(cb) = OkCallback::from_data(data) {
            cb
        } else {
            return Ok(Jmp::Stay);
        };
        ctx.set_system_go_back(false);
        ctx.set_system_go_home(false);

        let msg = if self.err {
            format!("❌ {}", self.text)
        } else {
            format!("✅ {}", self.text)
        };

        ctx.edit_origin(&msg, InlineKeyboardMarkup::default())
            .await?;

        ctx.reset_origin();
        match cb {
            OkCallback::Ok => {
                if self.back {
                    Ok(Jmp::Back(1))
                } else {
                    Ok(Jmp::ToSafePoint)
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum OkCallback {
    Ok,
}
