use bot_viewer::day::fmt_dt;
use teloxide::utils::markdown::escape;
use trainings::model::Training;

pub mod group;
pub mod personal;
pub mod rent;

pub fn render_time_slot_collision(training: &Training) -> String {
    format!(
        "Выбранное время пересекается с тренировкой *{}* в *{}*",
        escape(&training.name),
        fmt_dt(&training.get_slot().start_at())
    )
}
