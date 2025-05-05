use std::cmp::Ordering;

use subscription::model::{SubscriptionStatus, SubscriptionType, UserSubscription};
use trainings::model::Training;
use users::model::{User, payer::Payer};

pub trait SubscriptionResolver {
    fn find_subscription(
        &mut self,
        reason: FindFor,
        training: &Training,
    ) -> Option<&mut UserSubscription>;
}

pub trait AvailableBalance {
    fn available_balance_for_training(&self, training: &Training) -> u32;
}

impl SubscriptionResolver for Payer<&mut User> {
    fn find_subscription(
        &mut self,
        reason: FindFor,
        training: &Training,
    ) -> Option<&mut UserSubscription> {
        let start_at = training.get_slot().start_at();
        self.subscriptions
            .sort_by(|a, b| match (&a.status, &b.status) {
                (
                    SubscriptionStatus::Active {
                        start_date: _,
                        end_date: a_end_date,
                    },
                    SubscriptionStatus::Active {
                        start_date: _,
                        end_date: b_end_date,
                    },
                ) => a_end_date.cmp(b_end_date),
                (SubscriptionStatus::Active { .. }, SubscriptionStatus::NotActive) => {
                    Ordering::Less
                }
                (SubscriptionStatus::NotActive, SubscriptionStatus::Active { .. }) => {
                    Ordering::Greater
                }
                (SubscriptionStatus::NotActive, SubscriptionStatus::NotActive) => Ordering::Equal,
            });
        self.subscriptions
            .iter_mut()
            .filter(|s| match &s.tp {
                SubscriptionType::Group { program_filter } => {
                    !training.tp.is_personal() && program_filter.contains(&training.proto_id)
                }
                SubscriptionType::Personal { couch_filter } => {
                    if training.tp.is_personal() {
                        training.instructor == *couch_filter
                    } else {
                        false
                    }
                }
            })
            .find(|s| match reason {
                FindFor::Lock => {
                    if let SubscriptionStatus::Active {
                        start_date: _,
                        end_date,
                    } = s.status
                    {
                        end_date > start_at && (s.unlimited || s.balance > 0)
                    } else {
                        s.unlimited || s.balance > 0
                    }
                }
                FindFor::Charge => s.unlimited || s.locked_balance > 0,
                FindFor::Unlock => s.unlimited || s.locked_balance > 0,
            })
    }
}

impl AvailableBalance for Payer<&User> {
    fn available_balance_for_training(&self, training: &Training) -> u32 {
        self.subscriptions()
            .iter()
            .filter(|s| match &s.tp {
                SubscriptionType::Group { program_filter } => {
                    !training.tp.is_personal() && program_filter.contains(&training.proto_id)
                }
                SubscriptionType::Personal { couch_filter } => {
                    if training.tp.is_personal() {
                        training.instructor == *couch_filter
                    } else {
                        false
                    }
                }
            })
            .map(|s| s.balance)
            .sum()
    }
}

pub enum FindFor {
    Lock,
    Charge,
    Unlock,
}

#[cfg(test)]
mod tests {
    use bson::oid::ObjectId;
    use chrono::{DateTime, Utc};
    use decimal::Decimal;
    use program::model::TrainingType;
    use subscription::model::{SubscriptionStatus, SubscriptionType, UserSubscription};
    use trainings::model::Training;
    use users::model::test_user;

    use crate::payer::SubscriptionResolver as _;

    use super::User;

    fn user(subs: Vec<UserSubscription>) -> User {
        let mut user = test_user();
        user.subscriptions = subs;
        user
    }

    fn sub(
        items: u32,
        tp: SubscriptionType,
        days: u32,
        start_date: Option<&str>,
    ) -> UserSubscription {
        let status = if let Some(start_date) = start_date {
            let start_date: DateTime<Utc> = start_date.parse().unwrap();
            SubscriptionStatus::Active {
                start_date,
                end_date: start_date + chrono::Duration::days(i64::from(days)),
            }
        } else {
            SubscriptionStatus::NotActive
        };

        UserSubscription {
            id: ObjectId::new(),
            subscription_id: ObjectId::new(),
            name: "".to_owned(),
            items: 0,
            days,
            status,
            price: Decimal::zero(),
            tp,
            balance: items,
            locked_balance: 0,
            unlimited: false,
            discount: None,
            item_price: None,
        }
    }

    fn training(start_at: &str, group: bool) -> Training {
        Training::new(
            ObjectId::new(),
            "name".to_owned(),
            "desc".to_owned(),
            start_at.parse::<DateTime<Utc>>().unwrap(),
            1,
            ObjectId::new(),
            1,
            false,
            if group {
                TrainingType::Group { is_free: false }
            } else {
                TrainingType::Personal { is_free: false }
            },
            ObjectId::new(),
        )
    }

    #[test]
    fn test_users_find_subscription() {
        let mut alice = user(vec![]);
        let tr = training("2012-12-12T12:12:12Z", true);
        assert!(
            alice
                .payer_mut()
                .unwrap()
                .find_subscription(super::FindFor::Lock, &tr)
                .is_none()
        );

        let mut alice = user(vec![sub(
            0,
            SubscriptionType::Group {
                program_filter: vec![tr.proto_id],
            },
            1,
            None,
        )]);
        assert!(
            alice
                .payer_mut()
                .unwrap()
                .find_subscription(super::FindFor::Lock, &tr)
                .is_none()
        );

        let mut alice = user(vec![sub(
            1,
            SubscriptionType::Group {
                program_filter: vec![tr.proto_id],
            },
            1,
            None,
        )]);
        assert!(
            alice
                .payer_mut()
                .unwrap()
                .find_subscription(super::FindFor::Lock, &tr)
                .is_some()
        );

        let tr_1 = training("2014-12-12T12:12:12Z", true);
        let mut alice = user(vec![
            sub(
                1,
                SubscriptionType::Group {
                    program_filter: vec![tr.proto_id, tr_1.proto_id],
                },
                1,
                None,
            ),
            sub(
                1,
                SubscriptionType::Group {
                    program_filter: vec![tr.proto_id, tr_1.proto_id],
                },
                30,
                Some("2012-12-11T12:12:12Z"),
            ),
        ]);
        assert!(
            alice
                .payer_mut()
                .unwrap()
                .find_subscription(super::FindFor::Lock, &tr)
                .unwrap()
                .status
                .is_active()
        );
        assert!(
            !alice
                .payer_mut()
                .unwrap()
                .find_subscription(super::FindFor::Lock, &tr_1)
                .unwrap()
                .status
                .is_active()
        );
    }
}
