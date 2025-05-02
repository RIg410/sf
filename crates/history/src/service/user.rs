use users::log::UserLog;

use super::History;

impl UserLog for History {
    fn create_user(
        &self,
        session: &mut store::session::Session,
        name: users::model::UserName,
        phone: String,
    ) -> impl Future<Output = eyre::Result<()>> + Send {
        self.create_user(session, name, phone)
    }

    fn freeze(
        &self,
        session: &mut store::session::Session,
        user: bson::oid::ObjectId,
        days: u32,
    ) -> impl Future<Output = eyre::Result<()>> + Send {
        self.freeze(session, user, days)
    }

    fn unfreeze(
        &self,
        session: &mut store::session::Session,
        user: bson::oid::ObjectId,
    ) -> impl Future<Output = eyre::Result<()>> + Send {
        self.unfreeze(session, user)
    }

    fn change_balance(
        &self,
        session: &mut store::session::Session,
        user: bson::oid::ObjectId,
        amount: i32,
    ) -> impl Future<Output = eyre::Result<()>> + Send {
        self.change_balance(session, user, amount)
    }

    fn change_subscription_days(
        &self,
        session: &mut store::session::Session,
        user: bson::oid::ObjectId,
        delta: i32,
    ) -> impl Future<Output = eyre::Result<()>> + Send {
        self.change_subscription_days(session, user, delta)
    }

    fn change_reserved_balance(
        &self,
        session: &mut store::session::Session,
        user: bson::oid::ObjectId,
        amount: i32,
    ) -> impl Future<Output = eyre::Result<()>> + Send {
        self.change_reserved_balance(session, user, amount)
    }

    fn expire_subscription(
        &self,
        session: &mut store::session::Session,
        id: bson::oid::ObjectId,
        subscription: subscription::model::UserSubscription,
    ) -> impl Future<Output = eyre::Result<()>> + Send {
        self.expire_subscription(session, id, subscription)
    }

    fn remove_family_member(
        &self,
        session: &mut store::session::Session,
        main_id: bson::oid::ObjectId,
        member_id: bson::oid::ObjectId,
    ) -> impl Future<Output = eyre::Result<()>> + Send {
        self.remove_family_member(session, main_id, member_id)
    }

    fn add_family_member(
        &self,
        session: &mut store::session::Session,
        main_id: bson::oid::ObjectId,
        member_id: bson::oid::ObjectId,
    ) -> impl Future<Output = eyre::Result<()>> + Send {
        self.add_family_member(session, main_id, member_id)
    }

    fn block_user(
        &self,
        session: &mut store::session::Session,
        user: bson::oid::ObjectId,
        is_active: bool,
    ) -> impl Future<Output = eyre::Result<()>> + Send {
        self.block_user(session, user, is_active)
    }
}
