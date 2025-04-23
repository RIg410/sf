use super::Statistics;
use eyre::Error;
use model::statistics::{advertising::AdvertisingStat, range::Range};
use storage::session::Session;

impl Statistics {
    pub async fn advertising(
        &self,
        session: &Session,
        range: Range,
    ) -> Result<AdvertisingStat, Error> {
        todo!()


    }
}
