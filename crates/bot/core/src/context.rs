use crate::bot::TgBot;
use mongodb::bson::oid::ObjectId;
use rights::Rule;
use services::SfServices;
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};
use store::session::Session;
use users::model::User;

pub struct AnonymousContext {
    pub session: Session,
    pub services: Arc<SfServices>,
}

impl AnonymousContext {
    pub fn new(session: Session, services: Arc<SfServices>) -> Self {
        Self { session, services }
    }
}

pub struct Context {
    pub bot: TgBot,
    pub me: User,
    pub services: Arc<SfServices>,
    pub session: Session,
    pub is_real_user: bool,
}

impl Context {
    pub fn new(
        bot: TgBot,
        me: User,
        ledger: Arc<SfServices>,
        session: Session,
        is_real_user: bool,
    ) -> Context {
        Context {
            bot,
            me,
            services: ledger,
            session,
            is_real_user,
        }
    }

    pub fn is_me(&self, id: ObjectId) -> bool {
        self.me.id == id
    }

    pub fn is_employee(&self) -> bool {
        self.me.employee.is_some()
    }

    pub fn is_couch(&self) -> bool {
        match &self.me.employee {
            Some(employee) => employee.is_couch(),
            None => false,
        }
    }

    pub fn is_active(&self) -> bool {
        self.me.is_active
    }

    pub fn is_admin(&self) -> bool {
        self.me.rights.is_admin()
    }

    pub fn has_right(&self, rule: Rule) -> bool {
        self.me.rights.has_rule(rule)
    }

    pub fn ensure(&self, rule: Rule) -> Result<(), eyre::Error> {
        self.me.rights.ensure(rule)
    }

    pub async fn reload_user(&mut self) -> Result<(), eyre::Error> {
        let mut user = self
            .services
            .users
            .get(&mut self.session, self.me.id)
            .await?
            .ok_or_else(|| eyre::eyre!("Failed to load existing user:{}", self.me.id))?;
        self.services
            .users
            .resolve_family(&mut self.session, &mut user)
            .await?;
        self.me = user;
        Ok(())
    }
}

impl Deref for Context {
    type Target = TgBot;

    fn deref(&self) -> &Self::Target {
        &self.bot
    }
}

impl DerefMut for Context {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bot
    }
}
