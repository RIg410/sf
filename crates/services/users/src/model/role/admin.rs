use crate::model::employee::EmployeeV2;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[derive(Default)]
pub struct AdminRole {
    employees: EmployeeV2,
}

