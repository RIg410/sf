use super::Users;
use bson::oid::ObjectId;
use eyre::Error;
use model::user::comments::Comment;
use storage::session::Session;
use tx_macro::tx;

impl Users {
    #[tx]
    pub async fn add_comment(
        &self,
        session: &mut Session,
        user_id: ObjectId,
        text: &str,
        author: ObjectId,
    ) -> Result<(), Error> {
        let comment = Comment::new(text.to_string(), author);

        let mut extension = self.store.get_extension(session, user_id).await?;
        extension.comments.push(comment);
        self.store.update_extension(session, extension).await?;

        Ok(())
    }

    #[tx]
    pub async fn delete_comment(
        &self,
        session: &mut Session,
        user_id: ObjectId,
        id: ObjectId,
    ) -> Result<(), Error> {
        let mut extension = self.store.get_extension(session, user_id).await?;
        extension.comments.retain(|comment| comment.id != id);
        self.store.update_extension(session, extension).await?;
        Ok(())
    }
}
