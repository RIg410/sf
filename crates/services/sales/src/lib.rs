use bson::oid::ObjectId;
use decimal::Decimal;
use error::SellSubscriptionError;
use eyre::{Result, eyre};
use history::service::History;
use ident::source::Source;
use program::service::Programs;
use store::session::Session;
use subscription::{model::SubscriptionType, service::Subscriptions};
use treasury::service::{Treasury, log::TreasuryLog};
use tx_macro::tx;
use users::{log::UserLog, model::sanitize_phone, service::Users};

pub mod error;

pub struct Sales<L> {
    users: Users<L>,
    subscriptions: Subscriptions,
    history: History,
    treasury: Treasury<L>,
    program: Programs,
}

impl<L: UserLog + TreasuryLog> Sales<L> {
    pub fn new(
        users: Users<L>,
        subscriptions: Subscriptions,
        history: History,
        treasury: Treasury<L>,
        program: Programs,
    ) -> Self {
        Self {
            users,
            subscriptions,
            history,
            treasury,
            program,
        }
    }

    #[tx]
    pub async fn sell_subscription(
        &self,
        session: &mut Session,
        subscription: ObjectId,
        buyer: ObjectId,
        discount: Option<Decimal>,
    ) -> Result<(), SellSubscriptionError> {
        let buyer = self
            .users
            .get(session, buyer)
            .await?
            .ok_or_else(|| SellSubscriptionError::UserNotFound)?;
        let subscription = self
            .subscriptions
            .get(session, subscription)
            .await?
            .ok_or_else(|| eyre!("User not found"))?;
        self.history
            .sell_subscription(session, subscription.clone(), buyer.id, discount)
            .await?;
        self.users
            .add_subscription(session, buyer.id, subscription.clone(), discount)
            .await?;
        self.treasury
            .sell(session, buyer.id, subscription, discount)
            .await?;
        Ok(())
    }

    #[tx]
    pub async fn presell_subscription(
        &self,
        session: &mut Session,
        sub_id: ObjectId,
        phone: String,
        first_name: String,
        last_name: Option<String>,
        come_from: Source,
        discount: Option<Decimal>,
    ) -> Result<()> {
        let phone = sanitize_phone(&phone);
        let buyer = if let Some(bayer) = self.users.get_by_phone(session, &phone).await? {
            bayer
        } else {
            self.users
                .create_uninit(session, phone, first_name, last_name, come_from)
                .await?
        };
        let subscription = self
            .subscriptions
            .get(session, sub_id)
            .await?
            .ok_or_else(|| eyre!("User not found"))?;
        self.history
            .sell_subscription(session, subscription.clone(), buyer.id, discount)
            .await?;
        self.users
            .add_subscription(session, buyer.id, subscription.clone(), discount)
            .await?;
        self.treasury
            .sell(session, buyer.id, subscription, discount)
            .await?;
        Ok(())
    }

    // TODO move this function to subscriptions
    #[tx]
    pub async fn edit_program_list(
        &self,
        session: &mut Session,
        sub: ObjectId,
        program_id: ObjectId,
        add: bool,
    ) -> Result<()> {
        let mut subscription = self
            .subscriptions
            .get(session, sub)
            .await?
            .ok_or_else(|| eyre!("Subscription not found"))?;
        let _ = self
            .program
            .get_by_id(session, program_id)
            .await?
            .ok_or_else(|| eyre!("Program not found"))?;
        if let SubscriptionType::Group { program_filter } = &mut subscription.subscription_type {
            if add {
                if program_filter.contains(&program_id) {
                    return Ok(());
                } else {
                    program_filter.push(program_id);
                }
            } else if program_filter.contains(&program_id) {
                program_filter.retain(|&x| x != program_id);
            } else {
                return Ok(());
            }
            self.subscriptions.update(session, &subscription).await?;
        } else {
            return Err(eyre!("Only group subscriptions can have programs"));
        }

        let users_with_subscription = self.users.find_with_subscription(session, sub).await?;
        for mut user in users_with_subscription {
            let subs = user.subscriptions_mut();
            for user_sub in subs.iter_mut() {
                if user_sub.subscription_id == sub {
                    user_sub.tp = subscription.subscription_type.clone();
                }
            }
            self.users.update(session, &mut user).await?;
        }
        Ok(())
    }
}
