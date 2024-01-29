use crate::model::api;
use crate::repository::Repository;
use std::sync::Arc;

pub struct ReadService {
    repository: Arc<Repository>,
}
impl ReadService {
    pub fn new(repository: Arc<Repository>) -> Self {
        Self { repository }
    }

    pub async fn get_teithet(&self, id: i64) -> anyhow::Result<api::teithet::Teithet> {
        let inserted = self.repository.get_teithet(id).await?;
        let converted = api::teithet::Teithet::from(inserted);
        Ok(converted)
    }
}
