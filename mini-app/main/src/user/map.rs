use crate::{
    adapters::ToView,
    pb::{
        subscription::{subscription_type_view, Group, Personal, SubscriptionTypeView},
        user::{
            rate_view, status_view, ActiveStatusView, EmployeeRoleView, EmployeeView, FamilyView,
            FixRateView, FreezeView, GroupTrainingRateView, IntervalView, NotActive,
            PersonalTrainingRateView, RateView, RightsView, RuleView, SourceView, StatusView,
            UserNameView, UserSubscriptionView, UserView,
        },
    },
};
use bot_viewer::fmt_phone_escape_less;
use model::{
    rights::{HasRule, Rights, Rule},
    statistics::source::Source,
    subscription::{SubscriptionStatus, SubscriptionType, UserSubscription},
    user::{
        employee::Employee,
        family::Family,
        rate::{EmployeeRole, Interval, Rate},
        Freeze, User,
    },
};

impl ToView<UserView> for User {
    fn to_view<R: HasRule>(self, rights: &R) -> UserView {
        let come_from = if rights.has_rule(Rule::ViewMarketingInfo) {
            Some(self.come_from.to_view(&()) as i32)
        } else {
            None
        };

        UserView {
            id: Some(self.id.to_view(rights)),
            tg_id: self.tg_id,
            name: Some(self.name.to_view(rights)),
            rights: Some(self.rights.to_view(rights)),
            phone: self.phone.as_deref().map(fmt_phone_escape_less),
            is_active: self.is_active,
            freeze: self.freeze.map(|freeze| freeze.to_view(rights)),
            freeze_days: self.freeze_days,
            come_from,
            family: Some(Box::new(self.family.to_view(rights))),
            subscriptions: self
                .subscriptions
                .into_iter()
                .map(|s| s.to_view(rights))
                .collect(),
            employee: self.employee.map(|e| e.to_view(rights)),
        }
    }
}

impl ToView<UserNameView> for model::user::UserName {
    fn to_view<R: HasRule>(self, _: &R) -> UserNameView {
        UserNameView {
            tg_user_name: self.tg_user_name,
            first_name: self.first_name,
            last_name: self.last_name,
        }
    }
}

impl ToView<RightsView> for Rights {
    fn to_view<R: HasRule>(self, _: &R) -> RightsView {
        RightsView {
            full: self.is_full(),
            rights: self.iter().map(|r| r.to_view(&()) as i32).collect(),
        }
    }
}

impl ToView<RuleView> for Rule {
    fn to_view<R: HasRule>(self, _: &R) -> RuleView {
        match self {
            Rule::ViewProfile => RuleView::ViewProfile,
            Rule::ViewUsers => RuleView::AiStatistic,
            Rule::EditUserRights => RuleView::EditUserRights,
            Rule::BlockUser => RuleView::BlockUser,
            Rule::EditUserInfo => RuleView::EditUserInfo,
            Rule::EditUserSubscription => RuleView::EditUserSubscription,
            Rule::FreezeUsers => RuleView::FreezeUsers,
            Rule::ChangeBalance => RuleView::ChangeBalance,
            Rule::EditMarketingInfo => RuleView::EditMarketingInfo,
            Rule::EditFamily => RuleView::EditFamily,
            Rule::ViewFamily => RuleView::ViewFamily,
            Rule::EditAiPrompt => RuleView::EditAiPrompt,
            Rule::ViewUserComments => RuleView::ViewUserComments,
            Rule::EditUserComments => RuleView::EditUserComments,
            Rule::DeleteUserComments => RuleView::DeleteUserComments,
            Rule::EditTraining => RuleView::EditTraining,
            Rule::CreateTraining => RuleView::CreateTraining,
            Rule::EditTrainingClientsList => RuleView::EditTrainingClientsList,
            Rule::SetKeepOpen => RuleView::SetKeepOpen,
            Rule::SetFree => RuleView::SetFree,
            Rule::EditSchedule => RuleView::EditSchedule,
            Rule::CancelTraining => RuleView::CancelTraining,
            Rule::RemoveTraining => RuleView::RemoveTraining,
            Rule::EditTrainingCouch => RuleView::EditTrainingCouch,
            Rule::ScheduleGroupTraining => RuleView::ScheduleGroupTraining,
            Rule::SchedulePersonalTraining => RuleView::SchedulePersonalTraining,
            Rule::ScheduleSubRent => RuleView::ScheduleSubRent,
            Rule::SelectPersonalInstructor => RuleView::SelectPersonalInstructor,
            Rule::ViewAllTrainings => RuleView::ViewAllTrainings,
            Rule::ChangeTrainingSlot => RuleView::ChangeTrainingSlot,
            Rule::CreateSubscription => RuleView::CreateSubscription,
            Rule::EditSubscription => RuleView::EditSubscription,
            Rule::SellSubscription => RuleView::SellSubscription,
            Rule::FreeSell => RuleView::FreeSell,
            Rule::SubRent => RuleView::SubRent,
            Rule::ViewFinance => RuleView::ViewFinance,
            Rule::MakePayment => RuleView::MakePayment,
            Rule::MakeDeposit => RuleView::MakeDeposit,
            Rule::FinanceHistoricalDate => RuleView::FinanceHistoricalDate,
            Rule::DeleteHistory => RuleView::DeleteHistory,
            Rule::ViewEmployees => RuleView::ViewEmployees,
            Rule::EditEmployee => RuleView::EditEmployee,
            Rule::EditEmployeeRates => RuleView::EditEmployeeRates,
            Rule::ViewLogs => RuleView::ViewLogs,
            Rule::CreateCouch => RuleView::CreateCouch,
            Rule::EditCouch => RuleView::EditCouch,
            Rule::ViewCouchRates => RuleView::ViewCouchRates,
            Rule::ViewStatistics => RuleView::ViewStatistics,
            Rule::System => RuleView::System,
            Rule::ViewRewards => RuleView::ViewRewards,
            Rule::RecalculateRewards => RuleView::RecalculateRewards,
            Rule::ViewMarketingInfo => RuleView::ViewMarketingInfo,
            Rule::CreateRequest => RuleView::CreateRequest,
            Rule::RequestsHistory => RuleView::RequestsHistory,
            Rule::ReceiveNotificationsAboutSubscriptions => {
                RuleView::ReceiveNotificationsAboutSubscriptions
            }
            Rule::ReceiveNotificationsAboutBirthdays => {
                RuleView::ReceiveNotificationsAboutBirthdays
            }
            Rule::ReceiveAiNotifications => RuleView::ReceiveAiNotifications,
            Rule::MiniApp => RuleView::MiniApp,
            Rule::BuySubscription => RuleView::BuySubscription,
            Rule::ViewHiddenPrograms => RuleView::ViewHiddenPrograms,
            Rule::HistoryViewer => RuleView::HistoryViewer,
            Rule::AIStatistic => RuleView::AiStatistic,
            Rule::AIUserInfo => RuleView::AiUserInfo,
            Rule::SelectModel => RuleView::SelectModel,
        }
    }
}

impl ToView<FreezeView> for Freeze {
    fn to_view<R: HasRule>(self, _: &R) -> FreezeView {
        FreezeView {
            freeze_start: self.freeze_start.timestamp(),
            freeze_end: self.freeze_end.timestamp(),
        }
    }
}

impl ToView<SourceView> for Source {
    fn to_view<R: HasRule>(self, _: &R) -> SourceView {
        match self {
            Source::Unknown {} => SourceView::Unknown,
            Source::Website {} => SourceView::Website,
            Source::Instagram {} => SourceView::Instagram,
            Source::VK {} => SourceView::Vk,
            Source::YandexMap {} => SourceView::YandexMap,
            Source::YandexDirect {} => SourceView::YandexDirect,
            Source::DirectAdds {} => SourceView::DirectAdds,
            Source::VkAdds {} => SourceView::VkAdds,
            Source::DoubleGIS {} => SourceView::DoubleGis,
            Source::Avito {} => SourceView::Avito,
            Source::Recommendation {} => SourceView::Recommendation,
            Source::Other {} => SourceView::Other,
            Source::WebSearch {} => SourceView::WebSearch,
            Source::OldBase {} => SourceView::OldBase,
        }
    }
}

impl ToView<FamilyView> for Family {
    fn to_view<R: HasRule>(self, rights: &R) -> FamilyView {
        FamilyView {
            is_individual: self.is_individual,
            payer: self.payer.map(|u| Box::new(u.to_view(rights))),
            children: self
                .children
                .into_iter()
                .map(|u| u.to_view(rights))
                .collect(),
        }
    }
}

impl ToView<UserSubscriptionView> for UserSubscription {
    fn to_view<R: HasRule>(self, _: &R) -> UserSubscriptionView {
        UserSubscriptionView {
            id: Some(self.id.to_view(&())),
            subscription_id: Some(self.subscription_id.to_view(&())),
            name: self.name,
            items: self.items,
            days: self.days,
            status: Some(self.status.to_view(&())),
            price: self.price.inner(),
            tp: Some(self.tp.to_view(&())),
            balance: self.balance,
            locked_balance: self.locked_balance,
            unlimited: self.unlimited,
            discount: self.item_price.map(|p| p.inner()),
            item_price: self.item_price.map(|p| p.inner()),
        }
    }
}

impl ToView<StatusView> for SubscriptionStatus {
    fn to_view<R: HasRule>(self, _: &R) -> StatusView {
        match self {
            SubscriptionStatus::Active {
                start_date,
                end_date,
            } => StatusView {
                status_view: Some(status_view::StatusView::Active(ActiveStatusView {
                    start_date: start_date.timestamp(),
                    end_date: end_date.timestamp(),
                })),
            },
            SubscriptionStatus::NotActive => StatusView {
                status_view: Some(status_view::StatusView::NotActive(NotActive {})),
            },
        }
    }
}

impl ToView<SubscriptionTypeView> for SubscriptionType {
    fn to_view<R: HasRule>(self, _: &R) -> SubscriptionTypeView {
        match self {
            SubscriptionType::Group { program_filter } => SubscriptionTypeView {
                subscription_type: Some(subscription_type_view::SubscriptionType::Group(Group {
                    program_filter: program_filter.into_iter().map(|p| p.to_view(&())).collect(),
                })),
            },
            SubscriptionType::Personal { couch_filter } => SubscriptionTypeView {
                subscription_type: Some(subscription_type_view::SubscriptionType::Personal(
                    Personal {
                        couch_filter: Some(couch_filter.to_view(&())),
                    },
                )),
            },
        }
    }
}

impl ToView<EmployeeView> for Employee {
    fn to_view<R: HasRule>(self, _: &R) -> EmployeeView {
        let role = match self.role {
            EmployeeRole::Couch => EmployeeRoleView::Couch,
            EmployeeRole::Manager => EmployeeRoleView::Manager,
            EmployeeRole::Admin => EmployeeRoleView::Admin,
        } as i32;

        EmployeeView {
            role,
            description: self.description,
            reward: self.reward.inner(),
            rates: self.rates.into_iter().map(|r| r.to_view(&())).collect(),
        }
    }
}

impl ToView<RateView> for Rate {
    fn to_view<R: HasRule>(self, _: &R) -> RateView {
        match self {
            Rate::Fix {
                amount,
                next_payment_date,
                reward_interval,
            } => RateView {
                rate_type: Some(rate_view::RateType::Fix(FixRateView {
                    amount: amount.inner(),
                    next_payment_date: next_payment_date.timestamp(),
                    reward_interval: Some(reward_interval.to_view(&())),
                })),
            },
            Rate::GroupTraining {
                percent,
                min_reward,
            } => RateView {
                rate_type: Some(rate_view::RateType::GroupTraining(GroupTrainingRateView {
                    percent: percent.inner(),
                    min_reward: min_reward.inner(),
                })),
            },
            Rate::PersonalTraining { percent } => RateView {
                rate_type: Some(rate_view::RateType::PersonalTraining(
                    PersonalTrainingRateView {
                        percent: percent.inner(),
                    },
                )),
            },
        }
    }
}

impl ToView<IntervalView> for Interval {
    fn to_view<R: HasRule>(self, _: &R) -> IntervalView {
        match self {
            Interval::Month { num } => IntervalView { month_num: num },
        }
    }
}
