use crate::{BusinessError, Format};
use teloxide::utils::markdown::escape;
use users::error::UserError;

impl BusinessError for UserError {
    fn message(&self, format: Format) -> String {
        match self {
            UserError::Eyre(_) | UserError::MongoError(_) => "Системная ошибка".to_string(),
            UserError::UserNotFound(_) => "Пользователь не найден".to_string(),
            UserError::MemberNotFound { .. } => "Член семьи не найден".to_string(),
            UserError::WrongFamilyMember { .. } => {
                "Член семьи не совпадает с пользователем".to_string()
            }
            UserError::UserAlreadyInFamily { member, .. } => match format {
                Format::Text => format!("Пользователь {} уже состоит в семье", member.name),
                Format::Markdown => format!(
                    "Пользователь *{}* уже состоит в семье",
                    escape(&member.name)
                ),
            },
            UserError::UserAlreadyEmployee { .. } => {
                "Пользователь уже является сотрудником".to_string()
            }
            UserError::UserNotEmployee { .. } => "Пользователь не является сотрудником".to_string(),
            UserError::EmployeeHasReward { .. } => {
                "У сотрудника есть не выданная зарплата".to_string()
            }
            UserError::CouchHasTrainings(_) => {
                "У тренера есть незавершенные тренировки".to_string()
            }
            UserError::NoRatesFound { .. } => "У пользователя нет тарифов".to_string(),
            UserError::RateNotFound { rate, .. } => match rate {
                users::model::rate::Rate::Fix { .. } => "Фиксированный тариф не найден".to_string(),
                users::model::rate::Rate::GroupTraining { .. } => {
                    "Групповой тариф не найден".to_string()
                }
                users::model::rate::Rate::PersonalTraining { .. } => {
                    "Персональный тариф не найден".to_string()
                }
            },
            UserError::RateTypeAlreadyExists { rate, .. } => match rate {
                users::model::rate::Rate::Fix { .. } => {
                    "Фиксированный тариф уже существует".to_string()
                }
                users::model::rate::Rate::GroupTraining { .. } => {
                    "Групповой тариф уже существует".to_string()
                }
                users::model::rate::Rate::PersonalTraining { .. } => {
                    "Персональный тариф уже существует".to_string()
                }
            },
            UserError::Bson(_) => "Ошибка формата данных".to_string(),
            UserError::OnlyOwnerCanFreeze => "Только владелец может заморозить аккаунт".to_string(),
            UserError::InsufficientFreezeDays => "Недостаточно дней для заморозки".to_string(),
            UserError::UserIsNotClient => "Пользователь не является клиентом".to_string(),
            UserError::UserIsNotInstructor => "Пользователь не является инструктором".to_string(),
            UserError::UserIsNotManager => "Пользователь не является менеджером".to_string(),
            UserError::UserIsNotAdmin => "Пользователь не является администратором".to_string(),
            UserError::UserAlreadyFrozen => "Аккаунт уже заморожен".to_string(),
        }
    }

    fn is_fatal(&self) -> bool {
        match self {
            UserError::Eyre(_)
            | UserError::MongoError(_)
            | UserError::UserNotFound(_)
            | UserError::Bson(_) => true,

            UserError::MemberNotFound { .. }
            | UserError::WrongFamilyMember { .. }
            | UserError::UserAlreadyInFamily { .. }
            | UserError::UserAlreadyEmployee { .. }
            | UserError::UserNotEmployee { .. }
            | UserError::EmployeeHasReward { .. }
            | UserError::CouchHasTrainings(..)
            | UserError::NoRatesFound { .. }
            | UserError::RateNotFound { .. }
            | UserError::RateTypeAlreadyExists { .. }
            | UserError::OnlyOwnerCanFreeze
            | UserError::InsufficientFreezeDays
            | UserError::UserIsNotClient
            | UserError::UserIsNotInstructor
            | UserError::UserIsNotManager
            | UserError::UserIsNotAdmin
            | UserError::UserAlreadyFrozen => false,
        }
    }
}
