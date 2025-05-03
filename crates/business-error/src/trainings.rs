use crate::{BusinessError, Format};
use eyre::Error;
use ident::training::TrainingId;
use services::SfServices;
use store::session::Session;
use teloxide::utils::markdown::escape;
use trainings::error::TrainingError;

impl BusinessError for TrainingError {
    fn message(&self, format: Format) -> String {
        match self {
            TrainingError::Eyre(_) | TrainingError::MongoError(_) => "Системная ошибка".to_string(),
            TrainingError::TrainingIsProcessed(t) => match format {
                Format::Text => format!(
                    "Тренировка {} в {} уже завершена",
                    t.name,
                    t.id.start_at().format("%d.%m.%Y %H:%M")
                ),
                Format::Markdown => format!(
                    "Тренировка *{}* в _{}_ уже завершена",
                    escape(&t.name),
                    t.id.start_at().format("%d\\.%m\\.%Y %H:%M")
                ),
            },
            TrainingError::TrainingNotOpenToSignUp(t, _) => match format {
                Format::Text => format!(
                    "Тренировка {} в {} закрыта для записи",
                    t.name,
                    t.id.start_at().format("%d.%m.%Y %H:%M")
                ),
                Format::Markdown => format!(
                    "Тренировка *{}* в _{}_ закрыта для записи",
                    escape(&t.name),
                    t.id.start_at().format("%d\\.%m\\.%Y %H:%M")
                ),
            },
            TrainingError::ClientAlreadySignedUp(object_id, training_id) => {
                match format {
                    Format::Text => format!(
                        "Клиент {} уже записан на тренировку {}",
                        object_id,
                        training_id.start_at().format("%d.%m.%Y %H:%M")
                    ),
                    Format::Markdown => format!(
                        "Клиент *{}* уже записан на тренировку _{}_",
                        escape(object_id),
                        training_id.start_at().format("%d\\.%m\\.%Y %H:%M")
                    ),
                }

                // format!(
                //     "Ошибка:*Клиент {} уже записан на тренировку {}*",
                //     user_name(ctx, *object_id).await?,
                //     training_name(ctx, training_id).await?
                // )
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
        }
    }

    fn is_fatal(&self) -> bool {
        match self {
            TrainingError::Eyre(_) | TrainingError::MongoError(_) => true,
            TrainingError::TrainingHasClients(_) => false,
            TrainingError::TrainingIsProcessed(_) => false,
            TrainingError::TrainingNotOpenToSignUp(_, _) => false,
            TrainingError::ClientAlreadySignedUp(_, _) => false,
            TrainingError::TrainingIsFull(_) => false,
            TrainingError::NotEnoughBalance(_) => false,
            TrainingError::TrainingNotFound(_) => false,
            TrainingError::TrainingNotOpenToSignOut(_) => false,
            TrainingError::ClientNotSignedUp(_, _) => false,
            TrainingError::NotEnoughReservedBalance(_) => false,
        }
    }
}

async fn training_name(
    srv: &mut SfServices,
    session: &mut Session,
    training_id: &TrainingId,
) -> Result<String, Error> {
    let training = srv
        .calendar
        .get_training_by_id(session, *training_id)
        .await?;
    Ok(training
        .map(|t| t.name)
        .unwrap_or_else(|| "Тренировка не найдена".to_string()))
}
