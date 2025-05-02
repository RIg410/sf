use super::rate::{EmployeeRole, Rate};
use decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Employee {
    pub role: EmployeeRole,
    pub description: String,
    pub reward: Decimal,
    pub rates: Vec<Rate>,
}

impl Employee {
    pub fn is_couch(&self) -> bool {
        self.role == EmployeeRole::Couch
    }
}
