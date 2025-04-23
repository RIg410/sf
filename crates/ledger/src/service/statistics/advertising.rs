use super::Statistics;
use eyre::Error;
use model::statistics::{advertising::AdvertisingStat, range::Range};
use storage::session::Session;

impl Statistics {
    pub async fn advertising(
        &self,
        session: &mut Session,
        range: Range,
    ) -> Result<AdvertisingStat, Error> {
        let (from, to) = range.range()?;

        let requests = self.requests.find_range(session, Some(from), Some(to)).await?;
        

        todo!()
    }
}
