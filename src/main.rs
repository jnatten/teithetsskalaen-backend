use anyhow::Context;
use sqlx::Executor;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = dotenvy::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let db = PgPoolOptions::new()
        .max_connections(20)
        .after_connect(|conn, _meta| Box::pin(async move {
            conn.execute("SET application_name = 'teithetsskalaen'; SET search_path = 'teithetsskalaen';").await?;
            conn.execute("CREATE SCHEMA IF NOT EXISTS teithetsskalaen;").await?;
            Ok(())
        }))
        .connect(&database_url)
        .await
        .context("failed to connect to DATABASE_URL").expect("Failed to connect to database");

    sqlx::migrate!().run(&db).await.expect("Migration failed");

    println!("Hello my fancy database");

    Ok(())
}
