use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::context::Context;
use async_trait::async_trait;
use eyre::Result;
use services::error::SfError;
use teloxide::types::Message;

pub type ViewResult = Result<Jmp, SfError>;

#[async_trait]
pub trait View {
    // if view is safe point, Jmp::ToSafePoint will unroll the stack to this point
    fn safe_point(&self) -> bool {
        false
    }

    fn main_view(&self) -> bool {
        false
    }

    async fn show(&mut self, ctx: &mut Context) -> Result<(), eyre::Error>;

    async fn handle_message(&mut self, ctx: &mut Context, msg: &Message) -> ViewResult {
        ctx.delete_msg(msg.id).await?;
        Ok(Jmp::Stay)
    }

    async fn handle_callback(&mut self, _: &mut Context, _: &str) -> ViewResult {
        Ok(Jmp::Stay)
    }

    fn widget(self) -> Widget
    where
        Self: Sized + Send + Sync + 'static,
    {
        Widget {
            view: Box::new(self),
            back: None,
        }
    }
}

pub struct Widget {
    view: Box<dyn View + Send + Sync + 'static>,
    back: Option<Box<Widget>>,
}

impl Widget {
    pub fn set_back(&mut self, back: Widget) {
        self.back = Some(Box::new(back));
    }

    pub fn take_back(&mut self) -> Option<Widget> {
        self.back.take().map(|b| *b)
    }

    pub fn is_safe_point(&self) -> bool {
        self.view.safe_point()
    }
}

impl<T: View + Send + Sync + 'static> From<T> for Widget {
    fn from(value: T) -> Self {
        Widget {
            view: Box::new(value),
            back: None,
        }
    }
}

impl Deref for Widget {
    type Target = Box<dyn View + Send + Sync + 'static>;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

impl DerefMut for Widget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.view
    }
}

impl Debug for Widget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Widget")
    }
}

#[derive(Debug)]
pub enum Jmp {
    ToSafePoint,
    Next(Widget),
    Goto(Widget),
    Stay,
    Back(usize),
    Home,
}

impl<T: View + Send + Sync + 'static> From<T> for Jmp {
    fn from(value: T) -> Self {
        Jmp::Next(value.into())
    }
}

impl From<Widget> for Jmp {
    fn from(value: Widget) -> Self {
        Jmp::Next(value)
    }
}
