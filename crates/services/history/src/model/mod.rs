use bson::oid::ObjectId;
use chrono::{DateTime, Local, Utc};
use decimal::Decimal;
use ident::rooms::Room;
use serde::{Deserialize, Serialize};
use subscription::model::{Subscription, UserSubscription};
use users::model::UserName;

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryRow {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub actor: ObjectId,
    pub sub_actors: Vec<ObjectId>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub date_time: DateTime<Utc>,
    pub action: Action,
}

impl HistoryRow {
    pub fn new(actor: ObjectId, action: Action) -> Self {
        HistoryRow {
            id: ObjectId::new(),
            actor,
            sub_actors: vec![],
            date_time: Local::now().with_timezone(&Utc),
            action,
        }
    }

    pub fn with_sub_actors(actor: ObjectId, sub_actors: Vec<ObjectId>, action: Action) -> Self {
        HistoryRow {
            id: ObjectId::new(),
            actor,
            sub_actors,
            date_time: Local::now().with_timezone(&Utc),
            action,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    BlockUser {
        is_active: bool,
    },
    SignUp {
        start_at: DateTime<Local>,
        name: String,
        #[serde(default)]
        room_id: Room,
    },
    SignOut {
        start_at: DateTime<Local>,
        name: String,
        #[serde(default)]
        room_id: Room,
    },
    SellSub {
        subscription: Subscription,
        #[serde(default)]
        discount: Option<Decimal>,
    },
    PreSellSub {
        subscription: Subscription,
        phone: String,
    },
    FinalizedCanceledTraining {
        name: String,
        start_at: DateTime<Utc>,
        #[serde(default)]
        room_id: Room,
    },
    FinalizedTraining {
        name: String,
        start_at: DateTime<Utc>,
        #[serde(default)]
        room_id: Room,
    },
    Payment {
        amount: Decimal,
        description: String,
        date_time: DateTime<Utc>,
    },
    Deposit {
        amount: Decimal,
        description: String,
        date_time: DateTime<Utc>,
    },
    CreateUser {
        name: UserName,
        phone: String,
    },
    Freeze {
        days: u32,
    },
    Unfreeze {},
    ChangeBalance {
        amount: i32,
    },
    ChangeReservedBalance {
        amount: i32,
    },
    ChangeSubscriptionDays {
        delta: i32,
    },
    PayReward {
        amount: Decimal,
    },
    ExpireSubscription {
        subscription: UserSubscription,
    },
    RemoveFamilyMember {},
    AddFamilyMember {},
}

pub enum ActionType {
    SignUp,
    SignOut,
    SellSub,
    FinalizedTraining,
    FinalizedCanceledTraining,
    Payment,
    Deposit,
    CreateUser,
    Freeze,
    Unfreeze,
    ChangeBalance,
    ChangeReservedBalance,
    ChangeSubscriptionDays,
    PayReward,
    ExpireSubscription,
    RemoveFamilyMember,
    AddFamilyMember,
}

impl ActionType {
    pub fn name(&self) -> &'static str {
        match self {
            ActionType::SignUp => "SignUp",
            ActionType::SignOut => "SignOut",
            ActionType::SellSub => "SellSub",
            ActionType::FinalizedTraining => "FinalizedTraining",
            ActionType::FinalizedCanceledTraining => "FinalizedCanceledTraining",
            ActionType::Payment => "Payment",
            ActionType::Deposit => "Deposit",
            ActionType::CreateUser => "CreateUser",
            ActionType::Freeze => "Freeze",
            ActionType::Unfreeze => "Unfreeze",
            ActionType::ChangeBalance => "ChangeBalance",
            ActionType::ChangeReservedBalance => "ChangeReservedBalance",
            ActionType::ChangeSubscriptionDays => "ChangeSubscriptionDays",
            ActionType::PayReward => "PayReward",
            ActionType::ExpireSubscription => "ExpireSubscription",
            ActionType::RemoveFamilyMember => "RemoveFamilyMember",
            ActionType::AddFamilyMember => "AddFamilyMember",
        }
    }
}
