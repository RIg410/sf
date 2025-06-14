use crate::model::role::{
    admin::AdminRole, client::ClientRole, instructor::InstructorRole, manager::ManagerRole,
};

pub mod admin;
pub mod client;
pub mod instructor;
pub mod manager;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum UserRole {
    Client(ClientRole),
    Instructor(InstructorRole),
    SeniorInstructor(InstructorRole),
    Manager(ManagerRole),
    Admin(AdminRole),
}

impl UserRole {
    pub fn is_admin(&self) -> bool {
        matches!(self, UserRole::Admin(_))
    }

    pub fn is_manager(&self) -> bool {
        matches!(self, UserRole::Manager(_))
    }

    pub fn is_instructor(&self) -> bool {
        matches!(
            self,
            UserRole::Instructor(_) | UserRole::SeniorInstructor(_)
        )
    }

    pub fn is_senior_instructor(&self) -> bool {
        matches!(self, UserRole::SeniorInstructor(_))
    }

    pub fn is_client(&self) -> bool {
        matches!(self, UserRole::Client(_))
    }

    pub fn get_role_type(&self) -> RoleType {
        match self {
            UserRole::Client(_) => RoleType::Client,
            UserRole::Instructor(_) => RoleType::Instructor,
            UserRole::SeniorInstructor(_) => RoleType::SeniorInstructor,
            UserRole::Manager(_) => RoleType::Manager,
            UserRole::Admin(_) => RoleType::Admin,
        }
    }

    pub fn has_role(&self, role_types: RoleType) -> bool {
        self.get_role_type() == role_types
    }

    pub fn has_any_role(&self, role_types: &[RoleType]) -> bool {
        let current_role = self.get_role_type();
        role_types.contains(&current_role)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RoleType {
    Client,
    Instructor,
    SeniorInstructor,
    Manager,
    Admin,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Role {
    Client {},
    Instructor {},
    Manager {},
    Admin {},
}
