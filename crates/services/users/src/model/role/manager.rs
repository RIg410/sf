use crate::model::employee::EmployeeV2;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct ManagerRole {
    employees: EmployeeV2,
}
