use crate::{BusinessError, Format};
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
            TrainingError::TrainingNotOpenToSignUp(training, _) => match format {
                Format::Text => format!(
                    "Тренировка {} в {} закрыта для записи",
                    training.name,
                    training.id.start_at().format("%d.%m.%Y %H:%M")
                ),
                Format::Markdown => format!(
                    "Тренировка *{}* в _{}_ закрыта для записи",
                    escape(&training.name),
                    training.id.start_at().format("%d\\.%m\\.%Y %H:%M")
                ),
            },
            TrainingError::ClientAlreadySignedUp(name, training) => match format {
                Format::Text => format!(
                    "Клиент {} уже записан на тренировку {} в {}",
                    name.name,
                    training.name,
                    training.id.start_at().format("%d.%m.%Y %H:%M")
                ),
                Format::Markdown => format!(
                    "Клиент *{}* уже записан на тренировку *{}* в _{}_",
                    escape(&name.name),
                    escape(&training.name),
                    training.id.start_at().format("%d\\.%m\\.%Y %H:%M")
                ),
            },
            TrainingError::TrainingIsFull(training) => match format {
                Format::Text => format!(
                    "На тренировке {} в {} нет свободных мест",
                    training.name,
                    training.id.start_at().format("%d.%m.%Y %H:%M")
                ),
                Format::Markdown => format!(
                    "На тренировке *{}* в _{}_ нет свободных мест",
                    escape(&training.name),
                    training.id.start_at().format("%d\\.%m\\.%Y %H:%M")
                ),
            },
            TrainingError::NotEnoughBalance => "Нет подходящего абонемента".to_string(),

            TrainingError::TrainingNotFound(training) => match format {
                Format::Text => format!(
                    "Тренировка {} не найдена",
                    training.start_at().format("%d.%m.%Y %H:%M")
                ),
                Format::Markdown => format!(
                    "Тренировка *{}* не найдена",
                    training.start_at().format("%d\\.%m\\.%Y %H:%M")
                ),
            },
            TrainingError::TrainingNotOpenToSignOut(training) => match format {
                Format::Text => format!(
                    "Невозможно отменить запись на тренировку {} в {}",
                    training.name,
                    training.id.start_at().format("%d.%m.%Y %H:%M")
                ),
                Format::Markdown => format!(
                    "Невозможно отменить запись на тренировку *{}* в _{}_",
                    escape(&training.name),
                    training.id.start_at().format("%d\\.%m\\.%Y %H:%M")
                ),
            },
            TrainingError::ClientNotSignedUp(user, training) => match format {
                Format::Text => format!(
                    "Клиент {} не записан на тренировку {} в {}",
                    user.name,
                    training.name,
                    training.id.start_at().format("%d.%m.%Y %H:%M")
                ),
                Format::Markdown => format!(
                    "Клиент *{}* не записан на тренировку *{}* в _{}_",
                    escape(&user.name),
                    escape(&training.name),
                    training.id.start_at().format("%d\\.%m\\.%Y %H:%M")
                ),
            },
            TrainingError::NotEnoughReservedBalance => {
                "У пользователя недостаточно зарезервированных занятий".to_string()
            }
            TrainingError::TrainingHasClients(training) => match format {
                Format::Text => format!(
                    "В тренировке {} в {} есть записанные клиенты",
                    training.name,
                    training.id.start_at().format("%d.%m.%Y %H:%M")
                ),
                Format::Markdown => format!(
                    "В тренировке *{}* в _{}_ есть записанные клиенты",
                    escape(&training.name),
                    training.id.start_at().format("%d\\.%m\\.%Y %H:%M")
                ),
            },
        }
    }

    fn is_fatal(&self) -> bool {
        match self {
            TrainingError::Eyre(_)
            | TrainingError::MongoError(_)
            | TrainingError::TrainingNotFound(_) => true,
            TrainingError::TrainingHasClients(_) => false,
            TrainingError::TrainingIsProcessed(_) => false,
            TrainingError::TrainingNotOpenToSignUp(_, _) => false,
            TrainingError::ClientAlreadySignedUp(_, _) => false,
            TrainingError::TrainingIsFull(_) => false,
            TrainingError::NotEnoughBalance => false,
            TrainingError::TrainingNotOpenToSignOut(_) => false,
            TrainingError::ClientNotSignedUp(_, _) => false,
            TrainingError::NotEnoughReservedBalance => false,
        }
    }
}
