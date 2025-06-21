use crate::model::role::{
    admin::AdminRole, client::ClientRole, instructor::InstructorRole, manager::ManagerRole,
};

pub mod admin;
pub mod client;
pub mod instructor;
pub mod manager;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum Role {
    Client(ClientRole),
    Instructor(InstructorRole),
    Manager(ManagerRole),
    Admin(AdminRole),
}

impl Role {
    pub fn is_admin(&self) -> bool {
        matches!(self, Role::Admin(_))
    }

    pub fn is_manager(&self) -> bool {
        matches!(self, Role::Manager(_))
    }

    pub fn is_instructor(&self) -> bool {
        matches!(self, Role::Instructor(_))
    }

    pub fn is_client(&self) -> bool {
        matches!(self, Role::Client(_))
    }

    pub fn get_role_type(&self) -> RoleType {
        match self {
            Role::Client(_) => RoleType::Client,
            Role::Instructor(_) => RoleType::Instructor,
            Role::Manager(_) => RoleType::Manager,
            Role::Admin(_) => RoleType::Admin,
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

impl Default for Role {
    fn default() -> Self {
        Role::Client(ClientRole::default())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RoleType {
    Client,
    Instructor,
    Manager,
    Admin,
}

impl RoleType {
    pub fn make_role(&self) -> Role {
        match self {
            RoleType::Client => Role::Client(ClientRole::default()),
            RoleType::Instructor => Role::Instructor(InstructorRole::default()),
            RoleType::Manager => Role::Manager(ManagerRole::default()),
            RoleType::Admin => Role::Admin(AdminRole::default()),
        }
    }
}
