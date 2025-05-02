pub mod session;

use bson::{doc, oid::ObjectId};
use eyre::{Context as _, Error};
use mongodb::{Client, Database};
use session::Session;
use std::ops::Deref;

pub const SF_DB_NAME: &str = "ledger_db";

pub struct Db {
    client: Client,
    db: Database,
}

impl Db {
    pub async fn new(uri: &str, db_name: &str) -> Result<Self, Error> {
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
