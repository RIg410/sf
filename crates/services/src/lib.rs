use ai::Ai;
use backup::Backup;
use calendar::service::Calendar;
use env::Env;
use eyre::Error as EyError;
use eyre::Result;
use history::service::History;
use locations::service::Locations;
use program::service::Programs;
use requests::service::Requests;
use rewards::service::Rewards;
use sales::Sales;
use stat::services::Statistics;
use std::sync::Arc;
use store::Db;
use subscription::service::Subscriptions;
use training_adjuster::TrainingAdjuster;
use treasury::service::Treasury;
use users::service::Users;

pub mod error;

pub struct SfServices {
    pub db: Arc<Db>,
    pub users: Users<History>,
    pub calendar: Calendar<History>,
    pub programs: Programs,
    pub locations: Locations,
    pub treasury: Treasury<History>,
    pub subscriptions: Subscriptions,
    pub history: History,
    pub rewards: Rewards,
    pub statistics: Statistics<History>,
    pub backup: backup::Backup,
    pub requests: Requests<History>,
    pub yookassa: yookassa::Yookassa,
    pub booking: booking::Booking<History>,
    pub sales: Sales<History>,
    pub training_adjuster: TrainingAdjuster<History>,
    pub employee: employee::EmployeeService<History>,
    pub ai: Ai,
}

impl SfServices {
    pub async fn new(storage: Arc<Db>, env: Env) -> Result<Self, EyError> {
        let backup = Backup::new(storage.clone());

        let history = History::new(&storage).await?;
        let programs = Programs::new(&storage);
        let locations = Locations::new(&storage);
        let treasury = Treasury::new(&storage, history.clone()).await?;

        let ai = Ai::new(env.ai_base_url().to_owned(), env.ai_api_key().to_owned());

        let users = Users::new(&storage, history.clone(), ai.clone()).await?;
        let calendar = Calendar::new(&storage, users.clone(), programs.clone()).await?;

        let subscriptions = Subscriptions::new(&storage);
        let rewards = Rewards::new(&storage).await?;
        let requests = Requests::new(&storage, users.clone()).await?;

        let statistics =
            Statistics::new(history.clone(), users.clone(), requests.clone(), ai.clone());

        let booking = booking::Booking::new(calendar.clone(), users.clone(), history.clone());

        let sales = Sales::new(
            users.clone(),
            subscriptions.clone(),
            history.clone(),
            treasury.clone(),
            programs.clone(),
        );
        let training_adjuster = TrainingAdjuster::new(programs.clone(), calendar.clone());

        let employee = employee::EmployeeService::new(
            users.clone(),
            rewards.clone(),
            history.clone(),
            treasury.clone(),
            calendar.clone(),
        );

        Ok(SfServices {
            programs,
            locations,
            db: storage,
            treasury,
            subscriptions,
            history,
            rewards,
            statistics,
            backup,
            requests,
            yookassa: yookassa::Yookassa::new(&env),
            ai,
            booking,
            users,
            calendar,
            sales,
            training_adjuster,
            employee,
        })
    }
}
