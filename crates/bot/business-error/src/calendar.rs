use crate::{BusinessError, Format};
use calendar::error::CalendarError;
use teloxide::utils::markdown::escape;

impl BusinessError for CalendarError {
    fn message(&self, format: Format) -> String {
        match self {
            CalendarError::Eyre(_) | CalendarError::MongoError(_) => "Системная ошибка".to_string(),
            CalendarError::ProgramNotFound(_) => "Программа не найдена".to_string(),
            CalendarError::InstructorNotFound(_) => "Тренер не найден".to_string(),
            CalendarError::ClientNotFound(_) => "Клиент не найден".to_string(),
            CalendarError::InstructorHasNoRights(_) => "У тренера нет прав".to_string(),

            CalendarError::TimeSlotCollision(training) => match format {
                Format::Text => format!(
                    "Тренировка пересекается с тренировкой {} в {}",
                    training.name,
                    training.id.start_at().format("%d.%m.%Y %H:%M")
                ),
                Format::Markdown => format!(
                    "Тренировка пересекается с тренировкой *{}* в _{}_",
                    escape(&training.name),
                    training.id.start_at().format("%d\\.%m\\.%Y %H:%M")
                ),
            },
            CalendarError::DayIdMismatch { old, new } => match format {
                Format::Text => format!(
                    "День не совпадает: старый: {}, новый: {}",
                    old.local().format("%d.%m.%Y"),
                    new.local().format("%d.%m.%Y")
                ),
                Format::Markdown => {
                    format!(
                        "День не совпадает: старый: *{}*, новый: *{}*",
                        old.local().format("%d\\.%m\\.%Y"),
                        new.local().format("%d\\.%m\\.%Y")
                    )
                }
            },
            CalendarError::TrainingError(err) => err.message(format),
        }
    }

    fn is_fatal(&self) -> bool {
        match self {
            CalendarError::Eyre(_)
            | CalendarError::MongoError(_)
            | CalendarError::ProgramNotFound(_)
            | CalendarError::InstructorNotFound(_)
            | CalendarError::ClientNotFound(_)
            | CalendarError::InstructorHasNoRights(_) => true,
            CalendarError::TimeSlotCollision(_) => false,
            CalendarError::DayIdMismatch { .. } => false,
            CalendarError::TrainingError(err) => err.is_fatal(),
        }
    }
}
