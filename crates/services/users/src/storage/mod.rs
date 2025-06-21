mod employee;

use crate::error::UserError;
use crate::model::extension::UserExtension;
use crate::model::{Freeze, User, UserName};
use bson::to_document;
use chrono::{DateTime, Local, Utc};
use dashmap::DashMap;
use decimal::Decimal;
use eyre::{Error, Result, bail, eyre};
use futures_util::stream::TryStreamExt;
use ident::source::Source;
use mongodb::options::UpdateOptions;
use mongodb::{
    Collection, Database,
    bson::{doc, oid::ObjectId},
};
use mongodb::{IndexModel, SessionCursor};
use rights::Rule;
use store::session::Session;
use subscription::model::{Subscription, SubscriptionStatus, UserSubscription};
use tracing::info;

const COLLECTION: &str = "users";

pub struct UserStore {
    pub(crate) users: Collection<User>,
    pub(crate) extensions: Collection<UserExtension>,
    pub(crate) user_cache: DashMap<ObjectId, User>,
    pub(crate) tg_id_cache: DashMap<i64, ObjectId>,
}

impl UserStore {
    pub async fn new(db: &Database) -> Result<Self> {
        let users = db.collection(COLLECTION);
        users
            .create_index(IndexModel::builder().keys(doc! { "tg_id": 1 }).build())
            .await?;
        users
            .create_index(IndexModel::builder().keys(doc! { "phone": 1 }).build())
            .await?;
        Ok(UserStore {
            users,
            extensions: db.collection("users_extension"),
            user_cache: DashMap::new(),
            tg_id_cache: DashMap::new(),
        })
    }

    pub async fn migrate_users(&self, _: &mut Session) -> Result<()> {
        // info!("Migrating users...");
        // let mut cursor = self.users.find(doc! {}).session(&mut *session).await?;
        // while let Some(user) = cursor.next(&mut *session).await {
        //     let mut user = user?;
        //     let come_from = user.come_from;
        //     let client =if let Ok(client) = user.as_client_mut() {
        //         client
        //     } else {
        //         continue; // Skip users that are not clients
        //     };

        //     let source = match come_from {
        //         ident::source::Source::Unknown {} => SourceV2::Unknown,
        //         ident::source::Source::Website {} => SourceV2::Website,
        //         ident::source::Source::Instagram {} => SourceV2::Instagram,
        //         ident::source::Source::VK {} => SourceV2::VK,
        //         ident::source::Source::YandexMap {} => SourceV2::YandexMap,
        //         ident::source::Source::YandexDirect {} => SourceV2::YandexDirect,
        //         ident::source::Source::DirectAdds {} => SourceV2::DirectAdds,
        //         ident::source::Source::VkAdds {} => SourceV2::VkAdds,
        //         ident::source::Source::DoubleGIS {} => SourceV2::DoubleGIS,
        //         ident::source::Source::Avito {} => SourceV2::Avito,
        //         ident::source::Source::Recommendation {} => SourceV2::Recommendation,
        //         ident::source::Source::Other {} => SourceV2::Other,
        //         ident::source::Source::WebSearch {} => SourceV2::WebSearch,
        //         ident::source::Source::OldBase {} => SourceV2::OldBase,
        //     };
        //     client.come_from = source;

        //     self.update(session, &mut user).await?;
        // }
        // info!("User migration completed");
        Ok(())
    }

    pub async fn find_users_for_personal_training(
        &self,
        session: &mut Session,
        instructor_id: ObjectId,
    ) -> Result<Vec<User>> {
        let filter = doc! {
            "subscriptions": {
                "$elemMatch": {
                    "tp.Personal.couch_filter": instructor_id
                }
            }
        };

        let mut cursor = self.users.find(filter).session(&mut *session).await?;

        let mut users = vec![];
        while let Some(user) = cursor.next(&mut *session).await {
            let mut user = user?;
            self.resolve_family(&mut *session, &mut user).await?;
            if !user.family.children.is_empty() {
                users.extend(user.family.children.clone());
            }
            users.push(user);
        }
        Ok(users)
    }

    pub async fn find_users_with_right(
        &self,
        session: &mut Session,
        role: Rule,
    ) -> Result<Vec<User>> {
        let filter = doc! { "rights.rights": { "$elemMatch": { "$eq": format!("{:?}", role) } } };
        let mut cursor = self.users.find(filter).session(&mut *session).await?;
        Ok(cursor.stream(&mut *session).try_collect().await?)
    }

    pub async fn get(&self, session: &mut Session, id: ObjectId) -> Result<Option<User>> {
        if !session.in_transaction() {
            if let Some(user) = self.user_cache.get(&id) {
                let user = user.clone();

                return Ok(Some(user));
            }
        }

        let mut user = self.get_row(session, id).await?;
        if let Some(ref mut user) = user {
            self.resolve_family(session, user).await?;
        }
        self.cache_user(session, &mut user).await?;

        Ok(user)
    }

    async fn get_row(&self, session: &mut Session, id: ObjectId) -> Result<Option<User>> {
        let user = self
            .users
            .find_one(doc! { "_id": id })
            .session(&mut *session)
            .await?;
        Ok(user)
    }

    async fn cache_user(&self, session: &mut Session, user: &mut Option<User>) -> Result<()> {
        if let Some(user) = user {
            self.resolve_family(session, user).await?;
            self.user_cache.insert(user.id, user.clone());
            if user.tg_id != -1 {
                self.tg_id_cache.insert(user.tg_id, user.id);
            }
        }
        Ok(())
    }

    pub async fn resolve_family(&self, session: &mut Session, user: &mut User) -> Result<()> {
        let family = &mut user.family;
        if family.payer.is_none() {
            if let Some(payer) = family.payer_id {
                if let Some(payer) = self.get_row(session, payer).await? {
                    family.payer = Some(Box::new(payer));
                }
            }
        }

        if family.children_ids.len() != family.children.len() {
            family.children.clear();
            for child in &family.children_ids {
                if let Some(child) = self.get_row(session, *child).await? {
                    family.children.push(child);
                }
            }
        }

        Ok(())
    }

    pub async fn get_by_tg_id(&self, session: &mut Session, tg_id: i64) -> Result<Option<User>> {
        if !session.in_transaction() {
            if let Some(id) = self.tg_id_cache.get(&tg_id) {
                if let Some(user) = self.user_cache.get(&id) {
                    let user = user.clone();
                    return Ok(Some(user));
                }
            }
        }

        let mut user = self
            .users
            .find_one(doc! { "tg_id": tg_id })
            .session(&mut *session)
            .await?;
        self.cache_user(session, &mut user).await?;

        Ok(user)
    }

    pub async fn find_by_phone(&self, session: &mut Session, phone: &str) -> Result<Option<User>> {
        Ok(self
            .users
            .find_one(doc! { "phone": phone })
            .session(&mut *session)
            .await?)
    }

    pub async fn insert(&self, session: &mut Session, user: User) -> Result<()> {
        info!("Inserting user: {:?}", user);
        self.user_cache.remove(&user.id);

        let result = self
            .users
            .update_one(
                doc! { "_id": user.id },
                doc! { "$setOnInsert": to_document(&user)? },
            )
            .session(&mut *session)
            .with_options(UpdateOptions::builder().upsert(true).build())
            .await?;
        if result.upserted_id.is_none() {
            return Err(Error::msg("User already exists"));
        }
        Ok(())
    }

    pub async fn set_tg_id(&self, session: &mut Session, id: ObjectId, tg_id: i64) -> Result<()> {
        info!("Setting tg_id for user {}: {}", tg_id, id);
        self.user_cache.remove(&id);
        self.tg_id_cache.remove(&tg_id);

        let result = self
            .users
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "tg_id": tg_id }, "$inc": { "version": 1 } },
            )
            .session(&mut *session)
            .await?;
        if result.modified_count == 0 {
            return Err(Error::msg("User not found"));
        }
        Ok(())
    }

    pub async fn set_name(
        &self,
        session: &mut Session,
        id: ObjectId,
        name: UserName,
    ) -> Result<()> {
        info!("Setting name for user {}: {}", id, name);
        self.user_cache.remove(&id);

        let result = self
            .users
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "name": to_document(&name)? }, "$inc": { "version": 1 } },
            )
            .session(&mut *session)
            .await?;
        if result.modified_count == 0 {
            return Err(Error::msg("User not found"));
        }
        Ok(())
    }

    pub async fn count(&self, session: &mut Session, only_with_subscriptions: bool) -> Result<u64> {
        let mut query = doc! {};
        if only_with_subscriptions {
            query = doc! { "$and": [ query, { "subscriptions": { "$ne": [] } } ] };
        }

        Ok(self
            .users
            .count_documents(query)
            .session(&mut *session)
            .await?)
    }

    pub async fn find(
        &self,
        session: &mut Session,
        keywords: &[&str],
        offset: u64,
        limit: u64,
        employee: Option<bool>,
        only_with_subscriptions: bool,
    ) -> Result<SessionCursor<User>> {
        let mut query = doc! {};
        if !keywords.is_empty() {
            let mut keyword_query = vec![];
            for keyword in keywords {
                let regex = format!("^{keyword}");
                let regex_query = doc! {
                    "$or": [
                        { "name.first_name": { "$regex": &regex, "$options": "i" } },
                        { "name.last_name": { "$regex": &regex, "$options": "i" } },
                        { "name.tg_user_name": { "$regex": &regex, "$options": "i" } },
                        { "phone": { "$regex": &regex, "$options": "i" } },
                    ]
                };
                keyword_query.push(regex_query);
            }
            query = doc! { "$or": keyword_query };
        }

        if only_with_subscriptions {
            query = doc! { "$and": [ query, { "subscriptions": { "$ne": [] } } ] };
        }

        if let Some(is_employee) = employee {
            query = if is_employee {
                doc! { "$and": [ query, { "employee.role": { "$ne": null } } ] }
            } else {
                doc! { "$and": [ query, { "employee.role": null } ]}
            }
        }

        Ok(self
            .users
            .find(query)
            .skip(offset)
            .limit(limit as i64)
            .session(&mut *session)
            .await?)
    }

    pub async fn add_subscription(
        &self,
        session: &mut Session,
        id: ObjectId,
        sub: Subscription,
        discount: Option<Decimal>,
    ) -> Result<()> {
        self.user_cache.remove(&id);
        info!("Add subscription for user {}: {:?}", id, sub);
        let freeze_days = sub.freeze_days as i32;
        let amount = sub.items as i32;

        let mut sub = UserSubscription::from(sub);
        sub.discount = discount;

        let result = self
            .users
            .update_one(
                doc! { "_id": id },
                doc! {
                "$inc": {
                    "balance": amount,
                    "freeze_days": freeze_days,
                     "version": 1
                    },
                    "$push": {
                        "subscriptions": to_document(&sub)?
                    }
                },
            )
            .session(&mut *session)
            .await?;

        if result.modified_count != 1 {
            return Err(eyre!("Failed to modify balance"));
        }
        Ok(())
    }

    pub async fn find_users_to_unfreeze(&self, session: &mut Session) -> Result<Vec<User>, Error> {
        let filter = doc! {
            "freeze.freeze_end": { "$lte": Local::now().with_timezone(&Utc) }
        };
        let mut cursor = self.users.find(filter).session(&mut *session).await?;
        Ok(cursor.stream(&mut *session).try_collect().await?)
    }

    pub async fn unfreeze(&self, session: &mut Session, id: ObjectId) -> Result<()> {
        self.user_cache.remove(&id);
        info!("Unfreeze account:{}", id);
        let result = self
            .users
            .update_one(doc! { "_id": id }, doc! { "$unset": { "role.freeze": "" } })
            .session(&mut *session)
            .await?;

        if result.modified_count != 1 {
            return Err(eyre!("Failed to unfreeze account"));
        }
        Ok(())
    }

    pub async fn freeze(
        &self,
        session: &mut Session,
        id: ObjectId,
        days: u32,
        force: bool,
    ) -> Result<(), UserError> {
        self.user_cache.remove(&id);
        info!("Freeze account:{}", id);
        let mut user = self
            .get_row(session, id)
            .await?
            .ok_or_else(|| UserError::UserNotFound(id))?;
        self.resolve_family(session, &mut user).await?;

        if !user.payer()?.is_owner() {
            return Err(UserError::OnlyOwnerCanFreeze);
        }

        let client = user.as_client_mut()?;

        if !force && client.freeze_days < days {
            return Err(UserError::InsufficientFreezeDays);
        }

        client.freeze_days = client.freeze_days.saturating_sub(days);

        for sub in user.payer_mut()?.subscriptions_mut() {
            match sub.status {
                SubscriptionStatus::NotActive => {
                    //no-op
                }
                SubscriptionStatus::Active {
                    start_date,
                    end_date,
                } => {
                    sub.status = SubscriptionStatus::Active {
                        start_date,
                        end_date: end_date + chrono::Duration::days(days as i64),
                    }
                }
            }
        }

        let client = user.as_client_mut()?;
        client.freeze = Some(Freeze {
            freeze_start: Local::now().with_timezone(&Utc),
            freeze_end: Local::now().with_timezone(&Utc) + chrono::Duration::days(days as i64),
        });

        self.users
            .update_one(doc! { "_id": id }, doc! { "$set": to_document(&user)? })
            .session(&mut *session)
            .await?;
        Ok(())
    }

    pub async fn set_first_name(
        &self,
        session: &mut Session,
        id: ObjectId,
        first_name: &str,
    ) -> Result<bool> {
        self.user_cache.remove(&id);
        info!("Setting first_name for user {}: {}", id, first_name);
        let result = self
            .users
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "name.first_name": first_name }, "$inc": { "version": 1 } },
            )
            .session(&mut *session)
            .await?;
        Ok(result.modified_count > 0)
    }

    pub async fn set_last_name(
        &self,
        session: &mut Session,
        id: ObjectId,
        last_name: &str,
    ) -> Result<bool> {
        self.user_cache.remove(&id);
        info!("Setting last_name for user {}: {}", id, last_name);
        let result = self
            .users
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "name.last_name": last_name }, "$inc": { "version": 1 } },
            )
            .session(&mut *session)
            .await?;
        Ok(result.modified_count > 0)
    }

    pub async fn set_tg_user_name(
        &self,
        session: &mut Session,
        id: ObjectId,
        tg_user_name: &str,
    ) -> Result<bool> {
        self.user_cache.remove(&id);
        info!("Setting tg_user_name for user {}: {}", id, tg_user_name);
        let result = self
            .users
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "name.tg_user_name": tg_user_name }, "$inc": { "version": 1 } },
            )
            .session(&mut *session)
            .await?;
        Ok(result.modified_count > 0)
    }

    pub async fn instructors(&self, session: &mut Session) -> Result<Vec<User>, Error> {
        let filter = doc! { "employee.role": "Couch" };
        let mut cursor = self.users.find(filter).session(&mut *session).await?;
        Ok(cursor.stream(&mut *session).try_collect().await?)
    }

    pub async fn block_user(
        &self,
        session: &mut Session,
        id: ObjectId,
        is_active: bool,
    ) -> Result<bool> {
        self.user_cache.remove(&id);
        info!("Blocking user {}: {}", id, is_active);
        let result = self
            .users
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "is_active": is_active }, "$inc": { "version": 1 } },
            )
            .session(&mut *session)
            .await?;
        Ok(result.modified_count > 0)
    }

    pub async fn add_rule(
        &self,
        session: &mut Session,
        id: ObjectId,
        rule: &rights::Rule,
    ) -> Result<bool> {
        self.user_cache.remove(&id);
        info!("Adding rule {:?} to user {}", rule, id);
        let result = self.users
            .update_one(
                doc! { "_id": id },
                doc! { "$addToSet": { "rights.rights": format!("{:?}", rule) }, "$inc": { "version": 1 } },
            ).session(&mut *session)
            .await?;
        Ok(result.modified_count > 0)
    }

    pub async fn remove_rule(
        &self,
        session: &mut Session,
        id: ObjectId,
        rule: &rights::Rule,
    ) -> Result<bool> {
        self.user_cache.remove(&id);
        info!("Removing rule {:?} from user {}", rule, id);
        let result = self.users
            .update_one(
                doc! { "_id": id },
                doc! { "$pull": { "rights.rights": format!("{:?}", rule) }, "$inc": { "version": 1 } },
            )
            .session(&mut *session)
            .await?;
        Ok(result.modified_count > 0)
    }

    pub async fn find_users_with_active_subs(
        &self,
        session: &mut Session,
    ) -> Result<SessionCursor<User>, Error> {
        let filter = doc! {
            "subscriptions": { "$elemMatch": { "status": { "$ne": "NotActive" } } }
        };
        Ok(self.users.find(filter).session(&mut *session).await?)
    }

    pub async fn set_phone(&self, session: &mut Session, id: ObjectId, phone: &str) -> Result<()> {
        self.user_cache.remove(&id);
        info!("Setting phone for user {}: {}", id, phone);
        let result = self
            .users
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "phone": phone }, "$inc": { "version": 1 } },
            )
            .session(&mut *session)
            .await?;
        if result.modified_count == 0 {
            return Err(Error::msg("User not found"));
        }
        Ok(())
    }

    pub async fn get_by_phone(
        &self,
        session: &mut Session,
        phone: &str,
    ) -> std::result::Result<Option<User>, Error> {
        Ok(self
            .users
            .find_one(doc! { "phone": phone })
            .session(&mut *session)
            .await?)
    }

    pub async fn update_come_from(
        &self,
        session: &mut Session,
        id: ObjectId,
        come_from: Source,
    ) -> Result<(), Error> {
        self.user_cache.remove(&id);
        info!("Updating come_from: {:?}", come_from);
        self.users
            .update_one(
                doc! { "_id": id },
                doc! { "$set": { "role.come_from": to_document(&come_from)? } },
            )
            .session(&mut *session)
            .await?;

        Ok(())
    }

    pub async fn update(&self, session: &mut Session, user: &mut User) -> Result<()> {
        user.gc();
        self.user_cache.remove(&user.id);
        self.users
            .update_one(
                doc! { "_id": user.id },
                doc! { "$set": to_document(&user)? },
            )
            .session(&mut *session)
            .await?;
        Ok(())
    }

    pub async fn find_by_birthday(
        &self,
        session: &mut Session,
        day: u32,
        month: u32,
    ) -> Result<Vec<User>> {
        let filter = doc! {
            "birthday": { "$exists": true },
            "birthday.day": day,
            "birthday.month": month,
        };
        let mut cursor = self.extensions.find(filter).session(&mut *session).await?;
        let mut users = vec![];
        while let Some(ext) = cursor.next(&mut *session).await {
            let id = ext?.id;
            let user = self.get_row(session, id).await?;
            if let Some(user) = user {
                users.push(user);
            } else {
                bail!("User not found: {}", id);
            }
        }

        Ok(users)
    }

    pub async fn find_all(
        &self,
        session: &mut Session,
        from: Option<DateTime<Local>>,
        to: Option<DateTime<Local>>,
    ) -> Result<SessionCursor<User>, Error> {
        let filter = match (from, to) {
            (Some(from), Some(to)) => doc! {
                "created_at": { "$gte": from, "$lte": to }
            },
            (Some(from), None) => doc! {
                "created_at": { "$gte": from }
            },
            (None, Some(to)) => doc! {
                "created_at": { "$lte": to }
            },
            (None, None) => doc! {},
        };

        Ok(self.users.find(filter).session(&mut *session).await?)
    }

    pub async fn get_extension(
        &self,
        session: &mut Session,
        id: ObjectId,
    ) -> Result<UserExtension> {
        Ok(self
            .extensions
            .find_one(doc! { "_id": id })
            .session(&mut *session)
            .await?
            .unwrap_or_else(|| UserExtension {
                id,
                birthday: None,
                notification_mask: Default::default(),
                ai_message_prompt: None,
                comments: Default::default(),
            }))
    }

    pub async fn update_extension(
        &self,
        session: &mut Session,
        extension: UserExtension,
    ) -> Result<()> {
        self.extensions
            .update_one(
                doc! { "_id": extension.id },
                doc! { "$set": to_document(&extension)? },
            )
            .upsert(true)
            .session(&mut *session)
            .await?;
        Ok(())
    }

    pub async fn users_without_subscription(
        &self,
        session: &mut Session,
    ) -> Result<SessionCursor<User>, Error> {
        let filter = doc! {
            "subscriptions": { "$eq": [] }
        };
        Ok(self.users.find(filter).session(&mut *session).await?)
    }

    pub async fn find_with_subscription(
        &self,
        session: &mut Session,
        subscription: ObjectId,
    ) -> Result<Vec<User>> {
        let filter = doc! {
            "subscriptions.subscription_id": subscription
        };
        let mut cursor = self.users.find(filter).session(&mut *session).await?;
        Ok(cursor.stream(&mut *session).try_collect().await?)
    }
}
