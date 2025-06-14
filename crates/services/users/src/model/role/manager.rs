use crate::model::employee::EmployeeV2;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ManagerRole {
    employees: Vec<EmployeeV2>,
}
