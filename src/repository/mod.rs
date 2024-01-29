mod teithet;

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use time::PrimitiveDateTime;

pub struct Repository {
    db: PgPool,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Teithet {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub created_at: PrimitiveDateTime,
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
        let teithet = sqlx::query_as(
            r#"
                with inserted_teithet as (
                    insert into teithet(title, description)
                    values ($1, $2)
                    returning id, title, description, created_at
                )
                select id, title, description, created_at
                from inserted_teithet
            "#,
        )
        .bind(title)
        .bind(description)
        .fetch_one(&self.db)
        .await?;

        Ok(teithet)
    }

    pub async fn get_teithet(&self, id: i64) -> anyhow::Result<Teithet> {
        let teithet = sqlx::query_as(
            r#"
                select id, title, description, created_at
                from teithet
                where id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.db)
        .await?;

        Ok(teithet)
    }
}
