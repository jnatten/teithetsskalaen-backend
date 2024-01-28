mod teithet;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::PrimitiveDateTime;

pub struct Repository {
    db: PgPool,
}

#[derive(Serialize, Deserialize)]
pub struct Teithet {
    id: i64,
    title: String,
    description: String,
    created_at: PrimitiveDateTime,
}

impl Repository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn insert_teithet(
        &self,
        title: String,
        description: String,
    ) -> anyhow::Result<Teithet> {
        let teithet = sqlx::query_as!(
            Teithet,
            r#"
                with inserted_teithet as (
                    insert into teithet(title, description)
                    values ($1, $2)
                    returning id, title, description, created_at
                )
                select id, title, description, created_at
                from inserted_teithet
            "#,
            title,
            description,
        )
        .fetch_one(&self.db)
        .await?;

        Ok(teithet)
    }
}
