use crate::model::employee::EmployeeV2;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AdminRole {
    employees: EmployeeV2,
}

impl Default for AdminRole {
    fn default() -> Self {
        AdminRole {
            employees: EmployeeV2::default(),
        }
    }
}
