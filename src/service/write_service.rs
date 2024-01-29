use crate::model::api;
use crate::repository::Repository;
use std::sync::Arc;

pub struct WriteService {
    repository: Arc<Repository>,
}

impl WriteService {
    pub fn new(repository: Arc<Repository>) -> Self {
        Self { repository }
    }

    pub async fn new_teithet(
        &self,
        title: String,
        description: String,
    ) -> anyhow::Result<api::teithet::Teithet> {
        let inserted = self.repository.insert_teithet(title, description).await?;
        let converted = api::teithet::Teithet::from(inserted);
        Ok(converted)
    }
}
