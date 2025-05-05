use std::fmt::Write as _;

use ai::AiModel;
use bot_core::context::Context;
use eyre::Error;
use rights::Rule;
use teloxide::utils::markdown::escape;
use time::range::MonthRange;

pub async fn send_conversion(
    ctx: &mut Context,
    range: MonthRange,
    ai: Option<AiModel>,
) -> Result<(), Error> {
    let ai = if ctx.has_right(Rule::AIStatistic) {
        ai
    } else {
        None
    };

    let stat = ctx
        .services
        .statistics
        .advertising
        .conversion(&mut ctx.session, range, ai)
        .await?;

    let mut msg = String::new();
    write!(&mut msg, "*Конверсия по рекламным каналам*")?;

    for (channel, stat) in stat.sources {
        write!(&mut msg, "\n\n*{}*", channel.name())?;

        write!(
            &mut msg,
            "\nЗапросов: {}\nПробных: {}\nАбонементов: {}\nКонверсия в пробное: {:.0}%\nКонверсия в абонемент: {:.0}%",
            stat.processed_requests,
            stat.trial_visits,
            stat.bought_memberships,
            stat.conversion_to_trial,
            stat.conversion_to_membership
        )?;
    }

    if let Some(ai_comment) = stat.ai_comment {
        write!(&mut msg, "\n\n*AI комментарий:*\n{}", escape(&ai_comment))?;
    }

    ctx.send_notification(&msg).await;
    Ok(())
}
