use bson::oid::ObjectId;
use calendar::service::Calendar;
use chrono::Local;
use decimal::Decimal;
use eyre::Result;
use eyre::eyre;
use history::service::History;
use reward::EmployeeReward as _;
use rewards::service::Rewards;
use store::session::Session;
use trainings::model::Filter;
use treasury::model::subs::UserId;
use treasury::service::Treasury;
use treasury::service::log::TreasuryLog;
use tx_macro::tx;
use users::error::UserError;
use users::log::UserLog;
use users::model::employee::Employee;
use users::model::rate::EmployeeRole;
use users::model::rate::Rate;
use users::service::Users;

pub mod reward;

pub struct EmployeeService<L> {
    users: Users<L>,
    rewards: Rewards,
    history: History,
    treasury: Treasury<L>,
    calendar: Calendar<L>,
}

impl<L: UserLog + TreasuryLog> EmployeeService<L> {
    pub fn new(
        users: Users<L>,
        rewards: Rewards,
        history: History,
        treasury: Treasury<L>,
        calendar: Calendar<L>,
    ) -> Self {
        EmployeeService {
            users,
            rewards,
            history,
            treasury,
            calendar,
        }
    }

    #[tx]
    pub async fn delete_employee(
        &self,
        session: &mut Session,
        id: ObjectId,
    ) -> Result<(), UserError> {
        let has_trainings = !self
            .calendar
            .find_trainings(session, Filter::Instructor(id), 1, 0)
            .await?
            .is_empty();

        let user = self
            .users
            .get(session, id)
            .await?
            .ok_or_else(|| UserError::UserNotFound(id))?;

        if let Some(employee) = user.employee {
            if employee.reward != Decimal::zero() {
                return Err(UserError::EmployeeHasReward { user_id: id });
            }
        } else {
            return Err(UserError::UserNotEmployee { user_id: id });
        }

        if has_trainings {
            return Err(UserError::CouchHasTrainings(id));
        } else {
            self.users.delete_employee(session, id).await?;
            Ok(())
        }
    }

    #[tx]
    pub async fn add_recalculation_reward(
        &self,
        session: &mut Session,
        couch_id: ObjectId,
        amount: Decimal,
        comment: String,
    ) -> Result<()> {
        let mut user = self
            .users
            .get(session, couch_id)
            .await?
            .ok_or_else(|| UserError::UserNotFound(couch_id))?;

        let employee_info = user
            .employee
            .as_mut()
            .ok_or_else(|| eyre!("User is not couch"))?;
        let reward = employee_info.recalc_reward(user.id, amount, comment);
        self.rewards.add_reward(session, reward).await?;
        self.users
            .update_employee_reward_and_rates(session, user.id, employee_info.reward, None)
            .await?;
        Ok(())
    }

    #[tx]
    pub async fn pay_reward(
        &self,
        session: &mut Session,
        couch_id: ObjectId,
        amount: Decimal,
    ) -> Result<()> {
        let user = self
            .users
            .get(session, couch_id)
            .await?
            .ok_or_else(|| UserError::UserNotFound(couch_id))?;

        let mut employee_info = user.employee.ok_or_else(|| eyre!("User is not couch"))?;
        employee_info.get_reward(amount)?;
        self.history.pay_reward(session, couch_id, amount).await?;
        self.treasury
            .reward_employee(session, UserId::Id(couch_id), amount, &Local::now())
            .await?;
        self.users
            .update_employee_reward_and_rates(session, couch_id, employee_info.reward, None)
            .await?;
        Ok(())
    }

    #[tx]
    pub async fn update_employee_description(
        &self,
        session: &mut Session,
        id: ObjectId,
        description: String,
    ) -> Result<(), UserError> {
        let user = self
            .users
            .get(session, id)
            .await?
            .ok_or_else(|| UserError::UserNotFound(id))?;
        let employee = user
            .employee
            .ok_or_else(|| UserError::UserNotEmployee { user_id: id })?;
        let employee = Employee {
            description: description.clone(),
            reward: employee.reward,
            rates: employee.rates,
            role: employee.role,
        };

        self.users.set_employee(session, user.id, &employee).await?;
        Ok(())
    }

    #[tx]
    pub async fn make_user_employee(
        &self,
        session: &mut Session,
        id: ObjectId,
        description: String,
        rates: Vec<Rate>,
        role: EmployeeRole,
    ) -> Result<(), UserError> {
        let user = self
            .users
            .get(session, id)
            .await?
            .ok_or_else(|| UserError::UserNotFound(id))?;
        if user.employee.is_some() {
            return Err(UserError::UserAlreadyEmployee { user_id: id });
        }

        let employee = Employee {
            description,
            reward: Decimal::zero(),
            role,
            rates,
        };
        self.users.set_employee(session, id, &employee).await?;
        Ok(())
    }

    #[tx]
    pub async fn remove_rate(
        &self,
        session: &mut Session,
        id: ObjectId,
        rate: Rate,
    ) -> Result<(), UserError> {
        let user = self
            .users
            .get(session, id)
            .await?
            .ok_or_else(|| UserError::UserNotFound(id))?;
        let mut employee = user
            .employee
            .ok_or_else(|| UserError::UserNotEmployee { user_id: id })?;

        let rates = employee.rates.len();
        employee.rates.retain(|r| r != &rate);

        if rates == employee.rates.len() {
            return Err(UserError::RateNotFound { user_id: id, rate });
        }

        self.users.set_employee(session, user.id, &employee).await?;
        Ok(())
    }

    #[tx]
    pub fn update_rate(
        &self,
        session: &mut Session,
        id: ObjectId,
        old_date: Rate,
        new_rate: Rate,
    ) -> Result<(), UserError> {
        let user = self
            .users
            .get(session, id)
            .await?
            .ok_or_else(|| UserError::UserNotFound(id))?;
        let mut employee = user
            .employee
            .ok_or_else(|| UserError::UserNotEmployee { user_id: id })?;

        let rates = employee.rates.len();
        employee.rates.retain(|r| r != &old_date);

        if rates == employee.rates.len() {
            return Err(UserError::RateNotFound {
                user_id: id,
                rate: old_date,
            });
        }

        for rate in &mut employee.rates {
            if rate.as_u8() == old_date.as_u8() {
                return Err(UserError::RateTypeAlreadyExists {
                    user_id: id,
                    rate: new_rate,
                });
            }
        }

        employee.rates.push(new_rate);
        self.users.set_employee(session, user.id, &employee).await?;
        Ok(())
    }

    #[tx]
    pub async fn add_rate(
        &self,
        session: &mut Session,
        id: ObjectId,
        rate: Rate,
    ) -> Result<(), UserError> {
        let user = self
            .users
            .get(session, id)
            .await?
            .ok_or_else(|| UserError::UserNotFound(id))?;
        let mut employee = user
            .employee
            .ok_or_else(|| UserError::UserNotEmployee { user_id: id })?;

        if employee.rates.iter().any(|r| r.as_u8() == rate.as_u8()) {
            return Err(UserError::RateTypeAlreadyExists { user_id: id, rate });
        }

        employee.rates.push(rate);
        self.users.set_employee(session, user.id, &employee).await?;
        Ok(())
    }
}
