use crate::model::employee::EmployeeV2;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InstructorRole {
    employees: Vec<EmployeeV2>,
}
