use decimal::Decimal;
use eyre::Result;
use mongodb::bson::oid::ObjectId;
use store::session::Session;
use tx_macro::tx;

use crate::{error::UserError, log::UserLog, model::{employee::Employee, rate::{EmployeeRole, Rate}}};

use super::Users;

impl<L:UserLog> Users<L> {
    #[tx]
    pub async fn update_employee_description(
        &self,
        session: &mut Session,
        id: ObjectId,
        description: String,
    ) -> Result<(), UserError> {
        let user = self
            .store
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

        self.store.set_employee(session, user.id, &employee).await?;
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
            .store
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
        self.store.set_employee(session, id, &employee).await?;
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
            .store
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

        self.store.set_employee(session, user.id, &employee).await?;
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
            .store
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
        self.store.set_employee(session, user.id, &employee).await?;
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
            .store
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
        self.store.set_employee(session, user.id, &employee).await?;
        Ok(())
    }
}
