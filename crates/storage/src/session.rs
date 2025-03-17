use bson::{doc, oid::ObjectId};
use eyre::{Context as _, Error};
use mongodb::ClientSession;
use mongodb::{Client, Database};
use std::ops::{Deref, DerefMut};

pub struct Db {
    client: Client,
    db: Database,
}

impl Db {
    pub(crate) async fn new(uri: &str, db_name: &str) -> Result<Self, Error> {
        let client = Client::with_uri_str(uri)
            .await
            .context("Failed to connect to MongoDB")?;
        let db = client.database(db_name);
        db.run_command(doc! { "ping": 1 })
            .await
            .context("Failed to ping MongoDB")?;
        Ok(Db { client, db })
    }

    pub async fn start_session(&self) -> Result<Session, Error> {
        let session = self.client.start_session().await?;
        Ok(Session::new(session, ObjectId::new()))
    }
}

impl Deref for Db {
    type Target = Database;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}

pub struct Session {
    client_session: ClientSession,
    actor: ObjectId,
}

impl Session {
    pub fn new(client_session: ClientSession, actor: ObjectId) -> Self {
        Session {
            client_session,
            actor,
        }
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
