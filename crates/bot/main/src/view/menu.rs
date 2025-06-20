use async_trait::async_trait;
use bot_calendar::CalendarView;
use bot_core::{
    CommonLocation,
    callback_data::Calldata,
    context::Context,
    widget::{Jmp, View, ViewResult},
};
use bot_couch::list::CouchingList;
use bot_finance::FinanceView;
use bot_locations::LocationsView;
use bot_marketing::Marketing;
use bot_subscription::SubscriptionView;
use bot_trainings::program::list::ProgramList;
use bot_users::{Query, UsersView, profile::UserProfile};
use eyre::{Result, bail};
use rights::Rule;
use strum::EnumIter;
use teloxide::{
    types::{BotCommand, InlineKeyboardButton, InlineKeyboardMarkup, Message, WebAppInfo},
    utils::markdown::escape,
};

use crate::system::SystemView;

use super::{common_location::handle_common_location, signup::SignUpView};

#[derive(Default)]
pub struct MainMenuView;

impl MainMenuView {
    pub async fn send_self(&self, ctx: &mut Context) -> Result<(), eyre::Error> {
        let mut keymap = InlineKeyboardMarkup::default();
        keymap = keymap.append_row(vec![MainMenuItem::Profile.into()]);
        keymap = keymap.append_row(vec![MainMenuItem::Schedule.into()]);
        keymap = keymap.append_row(vec![MainMenuItem::Subscription.into()]);
        keymap = keymap.append_row(vec![MainMenuItem::Coach.into()]);
        keymap = keymap.append_row(vec![MainMenuItem::Programs.into()]);

        if ctx.has_right(Rule::ViewUsers) {
            keymap = keymap.append_row(vec![MainMenuItem::Users.into()]);
        }
        if ctx.has_right(Rule::ViewFinance) {
            keymap = keymap.append_row(vec![MainMenuItem::FinanceView.into()]);
        }

        if ctx.has_right(Rule::ViewMarketingInfo) {
            keymap = keymap.append_row(vec![MainMenuItem::Marketing.into()]);
        }

        if ctx.has_right(Rule::System) {
            keymap = keymap.append_row(vec![MainMenuItem::System.into()]);
            keymap = keymap.append_row(vec![MainMenuItem::Locations.into()]);
        }

        if ctx.has_right(Rule::MiniApp) {
            let env = ctx.bot.env();
            keymap = keymap.append_row(vec![InlineKeyboardButton::web_app(
                "App",
                WebAppInfo {
                    url: env.app_url().parse()?,
                },
            )]);
        }

        let group_balance = ctx.me.payer()?.group_balance();
        let personal_balance = ctx.me.payer()?.personal_balance();

        let mut txt = "🏠 *Меню* 🤸🏼\n".to_string();

        if ctx.me.employee.is_some() {
            txt.push_str(&format!(
                "Накопленное вознаграждение: *{}*",
                escape(&ctx.me.employee.as_ref().unwrap().reward.to_string())
            ));
        } else {
            if group_balance.unlimited {
                txt.push_str("👥 Групповые занятия: *безлимит*\n");
            } else if group_balance.is_empty() {
                txt.push_str("👥 Групповые занятия: 🅾️\n");
            } else {
                let lock = if group_balance.locked_balance == 0 {
                    ""
                } else {
                    &format!("\\(*{}* резерв\\)", group_balance.locked_balance)
                };

                txt.push_str(&format!(
                    "\n👥 Групповые занятия: *{}*{}\n",
                    group_balance.balance, lock
                ));
            }

            if personal_balance.unlimited {
                txt.push_str("🧑 Индивидуальные занятия: *безлимит*\n");
            } else if !personal_balance.is_empty() {
                let lock: &str = if personal_balance.locked_balance == 0 {
                    ""
                } else {
                    &format!("\\(*{}* резерв\\)", personal_balance.locked_balance)
                };

                txt.push_str(&format!(
                    "\n🧑 Индивидуальные занятия: *{}*{}",
                    personal_balance.balance, lock
                ));
            }
        }

        ctx.edit_origin(&txt, keymap).await?;
        Ok(())
    }
}

#[async_trait]
impl View for MainMenuView {
    fn main_view(&self) -> bool {
        true
    }

    fn safe_point(&self) -> bool {
        true
    }

    async fn show(&mut self, ctx: &mut Context) -> Result<(), eyre::Error> {
        if !ctx.origin().is_valid() {
            ctx.reset_origin();
            self.send_self(ctx).await?;
        }
        Ok(())
    }

    async fn handle_message(&mut self, ctx: &mut Context, msg: &Message) -> ViewResult {
        if !ctx.is_real_user {
            return Ok(SignUpView.into());
        }
        let text = if let Some(text) = msg.text() {
            text
        } else {
            return Ok(Jmp::Stay);
        };

        let command = if let Ok(command) = MainMenuItem::try_from(text) {
            command
        } else {
            return Ok(Jmp::Stay);
        };

        ctx.delete_msg(msg.id).await?;
        self.send_self(ctx).await?;
        Ok(match command {
            MainMenuItem::Profile => UserProfile::new(ctx.me.id).into(),
            MainMenuItem::Schedule => CalendarView::default().into(),
            MainMenuItem::Users => UsersView::new(Query::default()).into(),
            MainMenuItem::Subscription => SubscriptionView.into(),
            MainMenuItem::FinanceView => FinanceView.into(),
            MainMenuItem::Coach => CouchingList::new().into(),
            MainMenuItem::Home => MainMenuView.into(),
            MainMenuItem::Programs => ProgramList::default().into(),
            MainMenuItem::Marketing => Marketing::default().into(),
            MainMenuItem::System => SystemView::default().into(),
            MainMenuItem::Locations => LocationsView::new().into(),
        })
    }

    async fn handle_callback(&mut self, ctx: &mut Context, msg: &str) -> ViewResult {
        if !ctx.is_real_user {
            return Ok(SignUpView.into());
        }

        if CommonLocation::is_cmd(msg) {
            if let Some(location) = CommonLocation::from_data(msg) {
                return handle_common_location(ctx, location).await;
            }
        }

        let command = if let Ok(command) = MainMenuItem::try_from(msg) {
            command
        } else {
            return Ok(Jmp::Stay);
        };
        self.send_self(ctx).await?;
        Ok(match command {
            MainMenuItem::Profile => UserProfile::new(ctx.me.id).into(),
            MainMenuItem::Schedule => CalendarView::default().into(),
            MainMenuItem::Users => UsersView::new(Query::default()).into(),
            MainMenuItem::Subscription => SubscriptionView.into(),
            MainMenuItem::FinanceView => FinanceView.into(),
            MainMenuItem::Coach => CouchingList::new().into(),
            MainMenuItem::Home => MainMenuView.into(),
            MainMenuItem::Programs => ProgramList::default().into(),
            MainMenuItem::Marketing => Marketing::default().into(),
            MainMenuItem::System => SystemView::default().into(),
            MainMenuItem::Locations => LocationsView::new().into(),
        })
    }
}

#[derive(EnumIter, Clone, Copy, Debug, PartialEq)]
pub enum MainMenuItem {
    Home,
    Profile,
    Schedule,
    Users,
    Subscription,
    FinanceView,
    Coach,
    Programs,
    Marketing,
    System,
    Locations,
}

const HOME_DESCRIPTION: &str = "🏠";
const HOME_NAME: &str = "/start";

const PROFILE_DESCRIPTION: &str = "Профиль 🧑";
const PROFILE_NAME: &str = "/profile";

const TRAININGS_DESCRIPTION: &str = "Расписание 📅";
const TRAININGS_NAME: &str = "/schedule";

const SUBSCRIPTION_DESCRIPTION: &str = "Абонементы 💳";
const SUBSCRIPTION_NAME: &str = "/subscription";

const COUCH_DESCRIPTION: &str = "Наши инструкторы ❤️";
const COUCH_NAME: &str = "/couch";

const PROGRAM_DESCRIPTION: &str = "Наши программы 💪🏼";
const PROGRAM_NAME: &str = "/program";

const USERS_DESCRIPTION: &str = "Пользователи 👥";
const USERS_NAME: &str = "/users";

const FINANCE_DESCRIPTION: &str = "Финансы 💰";
const FINANCE_NAME: &str = "/finance";

const STATISTICS_DESCRIPTION: &str = "Маркетинг 📊";
const STATISTICS_NAME: &str = "/marketing";

const SYSTEM_DESCRIPTION: &str = "Система ⚙️";
const SYSTEM_NAME: &str = "/system";

const LOCATIONS_DESCRIPTION: &str = "Локации 🏢";
const LOCATIONS_NAME: &str = "/locations";

impl MainMenuItem {
    pub fn description(&self) -> &'static str {
        match self {
            MainMenuItem::Profile => PROFILE_DESCRIPTION,
            MainMenuItem::Schedule => TRAININGS_DESCRIPTION,
            MainMenuItem::Users => USERS_DESCRIPTION,
            MainMenuItem::Subscription => SUBSCRIPTION_DESCRIPTION,
            MainMenuItem::Home => HOME_DESCRIPTION,
            MainMenuItem::FinanceView => FINANCE_DESCRIPTION,
            MainMenuItem::Coach => COUCH_DESCRIPTION,
            MainMenuItem::Programs => PROGRAM_DESCRIPTION,
            MainMenuItem::Marketing => STATISTICS_DESCRIPTION,
            MainMenuItem::System => SYSTEM_DESCRIPTION,
            MainMenuItem::Locations => LOCATIONS_DESCRIPTION,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            MainMenuItem::Profile => PROFILE_NAME,
            MainMenuItem::Schedule => TRAININGS_NAME,
            MainMenuItem::Users => USERS_NAME,
            MainMenuItem::Subscription => SUBSCRIPTION_NAME,
            MainMenuItem::Home => HOME_NAME,
            MainMenuItem::FinanceView => FINANCE_NAME,
            MainMenuItem::Coach => COUCH_NAME,
            MainMenuItem::Programs => PROGRAM_NAME,
            MainMenuItem::Marketing => STATISTICS_NAME,
            MainMenuItem::System => SYSTEM_NAME,
            MainMenuItem::Locations => LOCATIONS_NAME,
        }
    }
}

impl From<MainMenuItem> for InlineKeyboardButton {
    fn from(value: MainMenuItem) -> Self {
        InlineKeyboardButton::callback(value.description(), value.name())
    }
}

impl From<MainMenuItem> for BotCommand {
    fn from(value: MainMenuItem) -> Self {
        BotCommand {
            command: value.name().to_string(),
            description: value.description().to_string(),
        }
    }
}

impl TryFrom<&str> for MainMenuItem {
    type Error = eyre::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            PROFILE_NAME | PROFILE_DESCRIPTION => Ok(MainMenuItem::Profile),
            TRAININGS_NAME | TRAININGS_DESCRIPTION => Ok(MainMenuItem::Schedule),
            USERS_NAME | USERS_DESCRIPTION => Ok(MainMenuItem::Users),
            SUBSCRIPTION_NAME | SUBSCRIPTION_DESCRIPTION => Ok(MainMenuItem::Subscription),
            HOME_NAME | HOME_DESCRIPTION | "/home" => Ok(MainMenuItem::Home),
            FINANCE_NAME | FINANCE_DESCRIPTION => Ok(MainMenuItem::FinanceView),
            COUCH_NAME | COUCH_DESCRIPTION => Ok(MainMenuItem::Coach),
            PROGRAM_NAME | PROGRAM_DESCRIPTION => Ok(MainMenuItem::Programs),
            STATISTICS_NAME | STATISTICS_DESCRIPTION => Ok(MainMenuItem::Marketing),
            SYSTEM_NAME | SYSTEM_DESCRIPTION => Ok(MainMenuItem::System),
            LOCATIONS_NAME | LOCATIONS_DESCRIPTION => Ok(MainMenuItem::Locations),
            _ => bail!("Unknown command"),
        }
    }
}
