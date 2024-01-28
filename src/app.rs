use crate::config::Config;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let app = Router::new().route("/", get(root));

    let address = format!("0.0.0.0:{}", config.port);
    println!("Serving application on {}", address);
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
