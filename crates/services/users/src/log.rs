use bson::oid::ObjectId;
use eyre::Result;
use store::session::Session;
use subscription::model::UserSubscription;

use crate::model::UserName;

pub trait UserLog {
    fn create_user(
        &self,
        session: &mut Session,
        name: UserName,
        phone: String,
    ) -> impl Future<Output = Result<()>> + Send;

    fn freeze(
        &self,
        session: &mut Session,
        user: ObjectId,
        days: u32,
    ) -> impl Future<Output = Result<()>> + Send;

    fn unfreeze(
        &self,
        session: &mut Session,
        user: ObjectId,
    ) -> impl Future<Output = Result<()>> + Send;

    fn change_balance(
        &self,
        session: &mut Session,
        user: ObjectId,
        amount: i32,
    ) -> impl Future<Output = Result<()>> + Send;

    fn change_subscription_days(
        &self,
        session: &mut Session,
        user: ObjectId,
        delta: i32,
    ) -> impl Future<Output = Result<()>> + Send;

    fn change_reserved_balance(
        &self,
        session: &mut Session,
        user: ObjectId,
        amount: i32,
    ) -> impl Future<Output = Result<()>> + Send;

    fn expire_subscription(
        &self,
        session: &mut Session,
        id: ObjectId,
        subscription: UserSubscription,
    ) -> impl Future<Output = Result<()>> + Send;

    fn remove_family_member(
        &self,
        session: &mut Session,
        main_id: ObjectId,
        member_id: ObjectId,
    ) -> impl Future<Output = Result<()>> + Send;

    fn add_family_member(
        &self,
        session: &mut Session,
        main_id: ObjectId,
        member_id: ObjectId,
    ) -> impl Future<Output = Result<()>> + Send;

    fn block_user(
        &self,
        session: &mut Session,
        user: ObjectId,
        is_active: bool,
    ) -> impl Future<Output = Result<()>> + Send;
}
