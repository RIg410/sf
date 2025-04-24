mod training;

use crate::{context::Context, widget::Jmp};
use chrono::Local;
use error::SfError;
use eyre::{Error, Result};
use model::user::rate::Rate;
use mongodb::bson::oid::ObjectId;
use teloxide::utils::markdown::escape;
use training::training_error;

pub async fn handle_result(ctx: &mut Context, result: Result<Jmp, Error>) -> Result<Jmp, Error> {
    match result {
        Ok(jmp) => Ok(jmp),
        Err(err) => {
            let ledger_err = err.downcast::<SfError>()?;
            if let Some(notification) = bassness_error(ctx, &ledger_err).await? {
                ctx.send_notification(&notification).await;
                Ok(Jmp::Stay)
            } else {
                Err(Error::new(ledger_err))
            }
        }
    }
}

pub async fn bassness_error(ctx: &mut Context, err: &SfError) -> Result<Option<String>> {
    Ok(Some(match err {
        SfError::Eyre(_) => return Ok(None),
        SfError::UserNotFound(object_id) => {
            format!("Ошибка: *Пользователь {} не найден*", obj_id(object_id))
        }
        SfError::MemberNotFound { user_id, member_id } => {
            let user = user_name(ctx, *user_id).await?;
            let member = user_name(ctx, *member_id).await?;
            format!(
                "Ошибка: *Пользователь {} не найден в семье пользователя {}*",
                member, user
            )
        }
        SfError::WrongFamilyMember { user_id, member_id } => {
            let user = user_name(ctx, *user_id).await?;
            let member = user_name(ctx, *member_id).await?;
            format!(
                "Ошибка:*Пользователь {} не является членом семьи пользователя {}*",
                member, user
            )
        }
        SfError::MongoError(_) => return Ok(None),
        SfError::UserAlreadyInFamily { user_id, member_id } => {
            let user = user_name(ctx, *user_id).await?;
            let member = user_name(ctx, *member_id).await?;
            format!(
                "Ошибка:*Пользователь {} уже является членом семьи пользователя {}*",
                member, user
            )
        }
        SfError::UserAlreadyEmployee { user_id } => format!(
            "Ошибка:*Пользователь {} уже является сотрудником*",
            user_name(ctx, *user_id).await?
        ),
        SfError::UserNotEmployee { user_id } => format!(
            "Ошибка:*Пользователь {} не является сотрудником*",
            user_name(ctx, *user_id).await?
        ),
        SfError::EmployeeHasReward { user_id } => format!(
            "Ошибка:*У сотрудника {} есть не выданная награда*",
            user_name(ctx, *user_id).await?
        ),
        SfError::CouchHasTrainings(user_id) => format!(
            "Ошибка:*Тренер {} имеет незавершенные тренировки*",
            user_name(ctx, *user_id).await?
        ),
        SfError::RateNotFound { user_id, rate } => {
            let user = user_name(ctx, *user_id).await?;
            format!(
                "Ошибка:*{} тариф не найден у пользователя {}*",
                rate_name(rate),
                user
            )
        }
        SfError::RateTypeAlreadyExists { user_id, rate } => {
            let user = user_name(ctx, *user_id).await?;
            format!(
                "Ошибка:*{} тариф уже существует у пользователя {}*",
                rate_name(rate),
                user
            )
        }
        SfError::NoRatesFound { user_id } => {
            let user = user_name(ctx, *user_id).await?;
            format!("Ошибка:*У пользователя {} нет тарифов*", user)
        }
        SfError::RequestNotFound { id } => format!("Ошибка:*Заявка {} не найдена*", id),
        SfError::ProgramNotFound(object_id) => {
            format!("Ошибка:*Программа {} не найдена*", object_id)
        }
        SfError::InstructorNotFound(object_id) => {
            format!(
                "Ошибка:*Тренер {} не найден*",
                user_name(ctx, *object_id).await?
            )
        }
        SfError::ClientNotFound(object_id) => {
            format!(
                "Ошибка:*Клиент {} не найден*",
                user_name(ctx, *object_id).await?
            )
        }
        SfError::InstructorHasNoRights(object_id) => {
            format!(
                "Ошибка:*Пользователь {} не имеет прав на проведение тренировки*",
                user_name(ctx, *object_id).await?
            )
        }
        SfError::TooCloseToStart { start_at: _ } => {
            "Ошибка:*Тренировка должна начаться не ранее чем за 3 часа от начала*".to_string()
        }
        SfError::TimeSlotCollision(training) => {
            format!(
                "Ошибка:*Тренировка пересекается с тренировкой {} в {}*",
                escape(&training.name),
                training.get_slot().start_at().format("%d\\.%m\\.%Y %H:%M")
            )
        }
        SfError::TrainingsError(err) => match training_error(ctx, err).await? {
            Some(error) => error,
            None => return Ok(None),
        },
        SfError::DayIdMismatch { old, new } => {
            format!(
                "Ошибка:*День {} не совпадает с днем {}*",
                old.id().with_timezone(&Local).format("%d\\.%m\\.%Y"),
                new.id().with_timezone(&Local).format("%d\\.%m\\.%Y")
            )
        }
    }))
}

pub async fn user_name(ctx: &mut Context, user_id: ObjectId) -> Result<String> {
    let user = ctx.services.users.get(&mut ctx.session, user_id).await?;
    Ok(user
        .map(|u| escape(&u.name.first_name))
        .unwrap_or_else(|| obj_id(&user_id)))
}

fn rate_name(rate: &Rate) -> &'static str {
    match rate {
        Rate::Fix { .. } => "Фиксированный",
        Rate::GroupTraining { .. } => "Групповой",
        Rate::PersonalTraining { .. } => "Персональный",
    }
}

fn obj_id(object_id: &ObjectId) -> String {
    escape(&object_id.to_string())
}
