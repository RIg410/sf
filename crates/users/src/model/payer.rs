use super::User;
use chrono::{DateTime, Utc};
use std::{
    mem,
    ops::{Deref, DerefMut},
};
use subscription::model::UserSubscription;

pub type ExpiredSubscription = Vec<UserSubscription>;

pub struct Payer<U>(U, bool);

impl<U> Payer<U> {
    pub(super) fn new(user: U, owned: bool) -> Self {
        Payer(user, owned)
    }
}

impl Payer<&mut User> {
    pub fn subscriptions_mut(&mut self) -> &mut Vec<UserSubscription> {
        &mut self.0.subscriptions
    }

    pub fn expire(&mut self, now: DateTime<Utc>) -> ExpiredSubscription {
        let (expired, actual) = mem::take(&mut self.0.subscriptions).into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut expired, mut actual), sub| {
                if sub.is_expired(now) {
                    expired.push(sub);
                } else {
                    actual.push(sub);
                }
                (expired, actual)
            },
        );

        self.0.subscriptions = actual;
        expired
    }
}

impl<'u> Payer<&User> {
    pub fn is_owner(&self) -> bool {
        self.1
    }

    pub fn subscriptions(&self) -> &[UserSubscription] {
        self.0.subscriptions.as_slice()
    }

    pub fn has_subscription(&self) -> bool {
        !self.0.subscriptions.is_empty()
    }

    pub fn group_balance(&self) -> Balance {
        let mut balance = 0;
        let mut locked_balance = 0;
        let mut unlimited = false;

        for sub in &self.0.subscriptions {
            if !sub.tp.is_personal() {
                balance += sub.balance;
                locked_balance += sub.locked_balance;
                if unlimited {
                    continue;
                }
                unlimited |= sub.unlimited;
            }
        }

        Balance {
            balance,
            locked_balance,
            unlimited,
        }
    }

    pub fn personal_balance(&self) -> Balance {
        let mut balance = 0;
        let mut locked_balance = 0;
        let mut unlimited = false;

        for sub in &self.0.subscriptions {
            if sub.tp.is_personal() {
                balance += sub.balance;
                locked_balance += sub.locked_balance;
                if unlimited {
                    continue;
                }
                unlimited |= sub.unlimited;
            }
        }

        Balance {
            balance,
            locked_balance,
            unlimited,
        }
    }
}

impl Deref for Payer<&mut User> {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl AsRef<User> for Payer<&mut User> {
    fn as_ref(&self) -> &User {
        self.0
    }
}

impl AsRef<User> for Payer<&User> {
    fn as_ref(&self) -> &User {
        self.0
    }
}

impl DerefMut for Payer<&mut User> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

pub struct Balance {
    pub balance: u32,
    pub locked_balance: u32,
    pub unlimited: bool,
}

impl Balance {
    pub fn is_empty(&self) -> bool {
        self.balance == 0 && self.locked_balance == 0
    }
}
