use bson::oid::ObjectId;
use chrono::{DateTime, TimeZone as _, Utc};
use core::fmt;
use eyre::eyre;
use family::Family;
use ident::{source::Source, user::UserIdWithName};
use payer::Payer;
use rights::Rights;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use subscription::model::UserSubscription;

use crate::{
    error::UserError,
    model::role::{
        RoleType, UserRole, admin::AdminRole, client::ClientRole, instructor::InstructorRole,
        manager::ManagerRole,
    },
};

pub mod comments;
pub mod employee;
pub mod extension;
pub mod family;
pub mod payer;
pub mod rate;
pub mod role;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub tg_id: i64,
    pub name: UserName,
    pub phone: Option<String>,
    #[serde(default = "default_is_active")]
    pub is_active: bool,

    #[serde(default)]
    pub role: UserRole,

    pub rights: Rights,
    #[serde(default)]
    pub subscriptions: Vec<UserSubscription>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    #[serde(default = "default_created_at")]
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub employee: Option<employee::Employee>,
    #[serde(default)]
    pub settings: UserSettings,
    #[serde(default)]
    pub come_from: Source,
    #[serde(default)]
    pub family: Family,
}

fn default_created_at() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2024, 9, 13, 12, 20, 0)
        .single()
        .unwrap()
}

impl User {
    pub fn new(
        tg_id: i64,
        name: UserName,
        phone: Option<String>,
        come_from: Source,
        role: RoleType,
    ) -> User {
        User {
            id: ObjectId::new(),
            tg_id,
            name,
            rights: Rights::customer(),
            phone,
            is_active: true,
            subscriptions: vec![],
            created_at: Utc::now(),
            settings: UserSettings::default(),
            come_from,
            family: Family::default(),
            employee: Default::default(),
            role: role.make_role(),
        }
    }

    pub fn has_subscriptions(&self) -> bool {
        !self.subscriptions.is_empty()
    }

    pub fn subscriptions_mut(&mut self) -> &mut [UserSubscription] {
        &mut self.subscriptions
    }

    pub fn subscriptions(&self) -> &[UserSubscription] {
        &self.subscriptions
    }

    pub fn with_tg_id(tg_id: i64) -> User {
        User {
            id: ObjectId::new(),
            tg_id,
            name: UserName {
                tg_user_name: None,
                first_name: "".to_owned(),
                last_name: None,
            },
            role: Default::default(),
            rights: Rights::customer(),
            phone: None,
            is_active: true,
            subscriptions: vec![],
            created_at: Utc::now(),
            settings: UserSettings::default(),
            come_from: Source::default(),
            family: Family::default(),
            employee: Default::default(),
        }
    }

    pub fn payer_mut(&mut self) -> Result<Payer<&mut User>, eyre::Error> {
        if self.family.is_individual {
            return Ok(Payer::new(self, true));
        }

        if !self.subscriptions.is_empty() {
            return Ok(Payer::new(self, true));
        }

        if self.family.payer_id.is_none() {
            return Ok(Payer::new(self, true));
        }

        if let Some(payer) = self.family.payer.as_mut() {
            Ok(Payer::new(payer, false))
        } else {
            Err(eyre!("Payer not resolved"))
        }
    }

    pub fn payer(&self) -> Result<Payer<&User>, eyre::Error> {
        if self.family.is_individual {
            return Ok(Payer::new(self, true));
        }

        if !self.subscriptions.is_empty() {
            return Ok(Payer::new(self, true));
        }

        if self.family.payer_id.is_none() {
            return Ok(Payer::new(self, true));
        }

        if let Some(payer) = self.family.payer.as_ref() {
            Ok(Payer::new(payer, false))
        } else {
            Err(eyre!("Payer not resolved"))
        }
    }

    pub fn gc(&mut self) {
        self.subscriptions.retain(|s| !s.is_empty());
    }

    pub fn is_couch(&self) -> bool {
        self.employee.as_ref().is_some_and(|e| e.is_couch())
    }

    pub fn has_family(&self) -> bool {
        self.family.payer_id.is_some() || !self.family.children_ids.is_empty()
    }

    pub fn id_with_name(&self) -> UserIdWithName {
        UserIdWithName {
            id: self.id,
            name: self.name.first_name.clone(),
        }
    }

    pub fn as_client(&self) -> Result<&ClientRole, UserError> {
        if let UserRole::Client(client) = &self.role {
            Ok(client)
        } else {
            Err(UserError::UserIsNotClient)
        }
    }

    pub fn as_client_mut(&mut self) -> Result<&mut ClientRole, UserError> {
        if let UserRole::Client(client) = &mut self.role {
            Ok(client)
        } else {
            Err(UserError::UserIsNotClient)
        }
    }

    pub fn as_instructor(&self) -> Result<&InstructorRole, UserError> {
        if let UserRole::Instructor(instructor) = &self.role {
            Ok(instructor)
        } else {
            Err(UserError::UserIsNotInstructor)
        }
    }
    pub fn as_instructor_mut(&mut self) -> Result<&mut InstructorRole, UserError> {
        if let UserRole::Instructor(instructor) = &mut self.role {
            Ok(instructor)
        } else {
            Err(UserError::UserIsNotInstructor)
        }
    }
    pub fn as_manager(&self) -> Result<&ManagerRole, UserError> {
        if let UserRole::Manager(manager) = &self.role {
            Ok(manager)
        } else {
            Err(UserError::UserIsNotManager)
        }
    }

    pub fn as_manager_mut(&mut self) -> Result<&mut ManagerRole, UserError> {
        if let UserRole::Manager(manager) = &mut self.role {
            Ok(manager)
        } else {
            Err(UserError::UserIsNotManager)
        }
    }

    pub fn as_admin(&self) -> Result<&AdminRole, UserError> {
        if let UserRole::Admin(admin) = &self.role {
            Ok(admin)
        } else {
            Err(UserError::UserIsNotAdmin)
        }
    }

    pub fn as_admin_mut(&mut self) -> Result<&mut AdminRole, UserError> {
        if let UserRole::Admin(admin) = &mut self.role {
            Ok(admin)
        } else {
            Err(UserError::UserIsNotAdmin)
        }
    }
}

pub fn test_user() -> User {
    User {
        id: ObjectId::new(),
        tg_id: 0,
        name: UserName {
            tg_user_name: None,
            first_name: "Test".to_owned(),
            last_name: None,
        },
        rights: Rights::customer(),
        phone: None,
        role: Default::default(),
        is_active: true,
        subscriptions: vec![],
        created_at: Utc::now(),
        settings: UserSettings::default(),
        come_from: Source::default(),
        family: Family::default(),
        employee: Default::default(),
    }
}

fn default_is_active() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Freeze {
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub freeze_start: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub freeze_end: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserName {
    pub tg_user_name: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
}

impl Display for UserName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.first_name)?;
        if let Some(last_name) = &self.last_name {
            write!(f, " {last_name}")?;
        }
        if let Some(tg_user_name) = &self.tg_user_name {
            write!(f, " (@{tg_user_name})")?;
        }
        Ok(())
    }
}

pub fn sanitize_phone(phone: &str) -> String {
    if phone.starts_with("8") {
        ("7".to_string() + &phone[1..])
            .chars()
            .filter_map(|c| if c.is_ascii_digit() { Some(c) } else { None })
            .collect()
    } else {
        phone
            .chars()
            .filter_map(|c| if c.is_ascii_digit() { Some(c) } else { None })
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSettings {
    pub notification: Notification,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Notification {
    pub notify_by_day: bool,
    pub notify_by_n_hours: Option<u8>,
}

impl Default for UserSettings {
    fn default() -> Self {
        UserSettings {
            notification: Notification {
                notify_by_day: true,
                notify_by_n_hours: Some(1),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::sanitize_phone;

    #[test]
    fn test_sanitize_phone_with_special_characters() {
        let phone = "+1 (234) 567-8900";
        let sanitized = sanitize_phone(phone);
        assert_eq!(sanitized, "12345678900");
    }

    #[test]
    fn test_sanitize_phone_with_spaces() {
        let phone = "123 456 7890";
        let sanitized = sanitize_phone(phone);
        assert_eq!(sanitized, "1234567890");
    }

    #[test]
    fn test_sanitize_phone_with_dashes() {
        let phone = "123-456-7890";
        let sanitized = sanitize_phone(phone);
        assert_eq!(sanitized, "1234567890");
    }

    #[test]
    fn test_sanitize_phone_with_dots() {
        let phone = "123.456.7890";
        let sanitized = sanitize_phone(phone);
        assert_eq!(sanitized, "1234567890");
    }

    #[test]
    fn test_sanitize_phone_with_letters() {
        let phone = "123-abc-7890";
        let sanitized = sanitize_phone(phone);
        assert_eq!(sanitized, "1237890");
    }

    #[test]
    fn test_sanitize_phone_with_empty_string() {
        let phone = "";
        let sanitized = sanitize_phone(phone);
        assert_eq!(sanitized, "");
    }

    #[test]
    fn test_sanitize_phone_with_only_special_characters() {
        let phone = "+-()";
        let sanitized = sanitize_phone(phone);
        assert_eq!(sanitized, "");
    }
}
