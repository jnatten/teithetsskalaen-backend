mod app;
mod config;
mod controller;
mod error;
mod model;
mod repository;
mod service;

pub use self::error::Error;
use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use std::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting teithetsskalaen-backend");
    let config = config::Config::from_env()?;

    println!("Connecting to database...");
    let db = PgPoolOptions::new()
        .max_connections(20)
        // TODO: This works but sqlx wont compile our queries with it
        //       We need to find a workaround for that, or just not use schemas...
        // .after_connect(|conn, _meta| {
        //     Box::pin(async move {
        //         conn.execute("SET application_name = 'teithetsskalaen'; SET search_path = 'teithetsskalaen';").await?;
        //         conn.execute("CREATE SCHEMA IF NOT EXISTS teithetsskalaen;").await?;
        //         Ok(())
        //     })
        // })
        .connect(&config.database_url)
        .await
        .context("failed to connect to DATABASE_URL")?;

    let before_migrations = Instant::now();
    println!("Running migrations...");
    sqlx::migrate!().run(&db).await?;
    println!(
        "Migrations completed successfully in {}ms",
        before_migrations.elapsed().as_millis()
    );

    app::serve(config, db).await
}
