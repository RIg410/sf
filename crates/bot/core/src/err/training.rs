use eyre::Error;
use ident::training::TrainingId;
use teloxide::utils::markdown::escape;
use trainings::error::TrainingError;

use crate::{context::Context, err::user_name};

pub async fn training_error(
    ctx: &mut Context,
    err: &TrainingError,
) -> Result<Option<String>, Error> {
    Ok(Some(match err {
        TrainingError::WrongTrainingClients { .. } => return Ok(None),
        TrainingError::Eyre(_) => return Ok(None),
        TrainingError::MongoError(_) => return Ok(None),
        TrainingError::TrainingIsProcessed(training_id) => {
            format!(
                "Ошибка:*Тренировка {} в {} уже обработана*",
                training_name(ctx, training_id).await?,
                training_id.start_at().format("%d\\.%m\\.%Y %H:%M")
            )
        }
        TrainingError::TrainingNotOpenToSignUp(training_id, _) => {
            format!(
                "Ошибка:*Тренировка {} в {} закрыта для записи*",
                training_name(ctx, training_id).await?,
                training_id.start_at().format("%d\\.%m\\.%Y %H:%M")
            )
        }
        TrainingError::ClientAlreadySignedUp(object_id, training_id) => {
            format!(
                "Ошибка:*Клиент {} уже записан на тренировку {}*",
                user_name(ctx, *object_id).await?,
                training_name(ctx, training_id).await?
            )
        }
        TrainingError::TrainingIsFull(training_id) => {
            format!(
                "Ошибка:*На тренировке {} в {} нет свободных мест*",
                training_name(ctx, training_id).await?,
                training_id.start_at().format("%d\\.%m\\.%Y %H:%M")
            )
        }
        TrainingError::NotEnoughBalance(_) => "Ошибка:*Нет подходящего абонемента*".to_string(),
        TrainingError::TrainingNotFound(training_id) => {
            format!(
                "Ошибка:*Тренировка {} не найдена*",
                training_id.start_at().format("%d\\.%m\\.%Y %H:%M")
            )
        }
        TrainingError::TrainingNotOpenToSignOut(training_id) => {
            format!(
                "Ошибка:*Тренировка {} в {} закрыта для отмены записи*",
                training_name(ctx, training_id).await?,
                training_id.start_at().format("%d\\.%m\\.%Y %H:%M")
            )
        }
        TrainingError::ClientNotSignedUp(object_id, training_id) => {
            format!(
                "Ошибка:*Клиент {} не записан на тренировку {}*",
                user_name(ctx, *object_id).await?,
                training_name(ctx, training_id).await?
            )
        }
        TrainingError::NotEnoughReservedBalance(object_id) => {
            format!(
                "Ошибка:*У пользователя {} недостаточно зарезервированных средств*",
                user_name(ctx, *object_id).await?
            )
        }
        TrainingError::TrainingHasClients(training_id) => {
            format!(
                "Ошибка:*Тренировка {} в {} имеет записанных клиентов*",
                training_name(ctx, training_id).await?,
                training_id.start_at().format("%d\\.%m\\.%Y %H:%M")
            )
        }
    }))
}

async fn training_name(ctx: &mut Context, training_id: &TrainingId) -> Result<String, Error> {
    let training = ctx
        .services
        .calendar
        .get_training_by_id(&mut ctx.session, *training_id)
        .await?;
    Ok(training
        .map(|t| escape(&t.name))
        .unwrap_or_else(|| "Тренировка не найдена".to_string()))
}
