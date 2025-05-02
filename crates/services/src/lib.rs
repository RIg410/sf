use ::store::session::{Db, Session};
use ai::Ai;
use backup::Backup;
use calendar::service::Calendar;
use env::Env;
use eyre::Error as EyError;
use eyre::{Context as _, Result, eyre};
use history::service::History;
use mongodb::bson::oid::ObjectId;
use program::service::Programs;
use requests::service::Requests;
use rewards::service::Rewards;
use sales::Sales;
use stat::services::Statistics;
use std::sync::Arc;
use subscription::service::Subscriptions;
use thiserror::Error;
use training_adjuster::TrainingAdjuster;
use trainings::model::status::TrainingStatus;
use treasury::service::Treasury;
use users::model::User;
use users::service::Users;

//pub mod store;

pub struct SfServices {
    pub db: Arc<Db>,
    pub users: Users<History>,
    pub calendar: Calendar<History>,
    pub programs: Programs,
    pub treasury: Treasury<History>,
    pub subscriptions: Subscriptions,
    pub history: History,
    pub rewards: Rewards,
    pub statistics: Statistics<History>,
    pub backup: backup::Backup,
    pub requests: Requests<History>,
    pub yookassa: yookassa::Yookassa,
    pub booking: booking::Booking<History>,
    pub sales: Sales<History>,
    pub training_adjuster: TrainingAdjuster<History>,
    pub ai: Ai,
}

impl SfServices {
    pub async fn new(storage: Arc<Db>, env: Env) -> Result<Self, EyError> {
        let backup = Backup::new(storage.clone());

        let history = History::new(&storage).await?;
        let programs = Programs::new(&storage);
        let treasury = Treasury::new(&storage, history.clone()).await?;

        let ai = Ai::new(env.ai_base_url().to_owned(), env.ai_api_key().to_owned());

        let users = Users::new(&storage, history.clone(), ai.clone()).await?;
        let calendar = Calendar::new(&storage, users.clone(), programs.clone()).await?;

        let subscriptions = Subscriptions::new(&storage);
        let rewards = Rewards::new(&storage).await?;
        let requests = Requests::new(&storage, users.clone()).await?;

        let statistics =
            Statistics::new(history.clone(), users.clone(), requests.clone(), ai.clone());

        let booking = booking::Booking::new(calendar.clone(), users.clone(), history.clone());

        let sales = Sales::new(
            users.clone(),
            subscriptions.clone(),
            history.clone(),
            treasury.clone(),
            programs.clone(),
        );
        let training_adjuster = TrainingAdjuster::new(programs.clone(), calendar.clone());

        Ok(SfServices {
            programs,
            db: storage,
            treasury,
            subscriptions,
            history,
            rewards,
            statistics,
            backup,
            requests,
            yookassa: yookassa::Yookassa::new(&env),
            ai,
            booking,
            users,
            calendar,
            sales,
            training_adjuster,
        })
    }

    pub async fn get_user(&self, session: &mut Session, id: ObjectId) -> Result<User> {
        let mut user = self
            .users
            .get(session, id)
            .await
            .context("get_user")?
            .ok_or_else(|| eyre!("User not found:{:?}", id))?;
        self.users.resolve_family(session, &mut user).await?;
        Ok(user)
    }

  
    // #[tx]
    // pub async fn delete_employee(
    //     &self,
    //     session: &mut Session,
    //     id: ObjectId,
    // ) -> Result<(), SfError> {
    //     let has_trainings = !self
    //         .calendar
    //         .find_trainings(session, Filter::Instructor(id), 1, 0)
    //         .await?
    //         .is_empty();

    //     let user = self
    //         .users
    //         .get(session, id)
    //         .await?
    //         .ok_or_else(|| UserError::UserNotFound(id))?;

    //     if let Some(employee) = user.employee {
    //         if employee.reward != Decimal::zero() {
    //             return Err(UserError::EmployeeHasReward { user_id: id }.into());
    //         }
    //     } else {
    //         return Err(UserError::UserNotEmployee { user_id: id }.into());
    //     }

    //     if has_trainings {
    //         return Err(UserError::CouchHasTrainings(id).into());
    //     } else {
    //         self.users.delete_employee(session, id).await?;
    //         Ok(())
    //     }
    // }

    // #[tx]
    // pub async fn add_recalculation_reward(
    //     &self,
    //     session: &mut Session,
    //     couch_id: ObjectId,
    //     amount: Decimal,
    //     comment: String,
    // ) -> Result<()> {
    //     let mut user = self.get_user(session, couch_id).await?;

    //     let employee_info = user
    //         .employee
    //         .as_mut()
    //         .ok_or_else(|| eyre!("User is not couch"))?;
    //     let reward = employee_info.recalc_reward(user.id, amount, comment);
    //     self.rewards.add_reward(session, reward).await?;
    //     self.users
    //         .update_employee_reward_and_rates(session, user.id, employee_info.reward, None)
    //         .await?;
    //     Ok(())
    // }

    // #[tx]
    // pub async fn pay_reward(
    //     &self,
    //     session: &mut Session,
    //     couch_id: ObjectId,
    //     amount: Decimal,
    // ) -> Result<()> {
    //     let user = self.get_user(session, couch_id).await?;
    //     let mut employee_info = user.employee.ok_or_else(|| eyre!("User is not couch"))?;
    //     employee_info.get_reward(amount)?;
    //     self.history.pay_reward(session, couch_id, amount).await?;
    //     self.treasury
    //         .reward_employee(session, UserId::Id(couch_id), amount, &Local::now())
    //         .await?;
    //     self.users
    //         .update_employee_reward_and_rates(session, couch_id, employee_info.reward, None)
    //         .await?;
    //     Ok(())
    // }
}

#[derive(Debug, Error)]
pub enum SignUpError {
    #[error("Training not found")]
    TrainingNotFound,
    #[error("Training is not open to sign up")]
    TrainingNotOpenToSignUp(TrainingStatus),
    #[error("Client already signed up")]
    ClientAlreadySignedUp,
    #[error("User not found")]
    UserNotFound,
    #[error("User is couch")]
    UserIsCouch,
    #[error("Common error:{0}")]
    Common(#[from] eyre::Error),
    #[error("Not enough balance")]
    NotEnoughBalance,
    #[error("Training is full")]
    TrainingIsFull,
}

impl From<mongodb::error::Error> for SignUpError {
    fn from(e: mongodb::error::Error) -> Self {
        SignUpError::Common(e.into())
    }
}
