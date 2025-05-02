use bson::oid::ObjectId;
use mongodb::ClientSession;
use std::ops::{Deref, DerefMut};

pub struct Session {
    client_session: ClientSession,
    actor: ObjectId,
    in_transaction: bool,
}

impl Session {
    pub fn new(client_session: ClientSession, actor: ObjectId) -> Self {
        Session {
            client_session,
            actor,
            in_transaction: false,
        }
    }

    pub fn in_transaction(&self) -> bool {
        self.in_transaction
    }

    pub async fn start_transaction(&mut self) -> Result<(), mongodb::error::Error> {
        if self.in_transaction {
            return Ok(());
        }

        let result = self.client_session.start_transaction().await;
        if result.is_ok() {
            self.in_transaction = true;
        }
        result
    }

    pub async fn commit_transaction(&mut self) -> Result<(), mongodb::error::Error> {
        if !self.in_transaction {
            return Ok(());
        }

        self.in_transaction = false;
        self.client_session.commit_transaction().await
    }

    pub async fn abort_transaction(&mut self) -> Result<(), mongodb::error::Error> {
        if !self.in_transaction {
            return Ok(());
        }
        self.in_transaction = false;
        self.client_session.abort_transaction().await
    }

    pub fn actor(&self) -> ObjectId {
        self.actor
    }

    pub fn set_actor(&mut self, actor: ObjectId) {
        self.actor = actor;
    }
}

impl Deref for Session {
    type Target = ClientSession;

    fn deref(&self) -> &Self::Target {
        &self.client_session
    }
}

impl DerefMut for Session {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.client_session
    }
}

impl<'a> From<&'a mut Session> for &'a mut ClientSession {
    fn from(session: &'a mut Session) -> &'a mut ClientSession {
        &mut session.client_session
    }
}
