use bson::oid::ObjectId;
use calendar::service::Calendar;
use eyre::Result;
use program::service::Programs;
use store::session::Session;
use tx_macro::tx;
use users::log::UserLog;

pub struct TrainingAdjuster<L> {
    programs: Programs,
    calendar: Calendar<L>,
}

impl<L: UserLog> TrainingAdjuster<L> {
    pub fn new(programs: Programs, calendar: Calendar<L>) -> Self {
        Self { programs, calendar }
    }

    #[tx]
    pub async fn edit_program_capacity(
        &self,
        session: &mut Session,
        program_id: ObjectId,
        value: u32,
    ) -> Result<()> {
        self.programs
            .edit_capacity(session, program_id, value)
            .await?;
        self.calendar
            .edit_capacity(session, program_id, value)
            .await?;
        Ok(())
    }

    #[tx]
    pub async fn edit_program_duration(
        &self,
        session: &mut Session,
        program_id: ObjectId,
        value: u32,
    ) -> Result<()> {
        self.calendar
            .edit_duration(session, program_id, value)
            .await?;
        self.programs
            .edit_duration(session, program_id, value)
            .await?;
        Ok(())
    }

    #[tx]
    pub async fn edit_program_name(
        &self,
        session: &mut Session,
        id: ObjectId,
        value: String,
    ) -> Result<()> {
        self.programs.edit_name(session, id, value.clone()).await?;
        self.calendar.edit_program_name(session, id, value).await?;
        Ok(())
    }

    #[tx]
    pub async fn edit_program_description(
        &self,
        session: &mut Session,
        id: ObjectId,
        value: String,
    ) -> Result<()> {
        self.programs
            .edit_description(session, id, value.clone())
            .await?;
        self.calendar
            .edit_program_description(session, id, value)
            .await?;
        Ok(())
    }
}
