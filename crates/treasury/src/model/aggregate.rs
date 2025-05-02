use chrono::{DateTime, Local};
use decimal::Decimal;
use ident::source::Source;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct TreasuryAggregate {
    pub from: DateTime<Local>,
    pub to: DateTime<Local>,
    pub debit: Decimal,
    pub credit: Decimal,
    pub income: AggIncome,
    pub outcome: AggOutcome,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AggIncome {
    pub subscriptions: Agg,
    pub sub_rent: Agg,
    pub other: Agg,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AggOutcome {
    pub rewards: Agg,
    pub marketing: HashMap<Source, Agg>,
    pub rent: Agg,
    pub other: Agg,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Agg {
    pub sum: Decimal,
    pub count: u32,
}

impl Agg {
    pub fn add(&mut self, amount: Decimal) {
        self.sum += amount;
        self.count += 1;
    }
}
