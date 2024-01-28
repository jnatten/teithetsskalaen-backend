use crate::config::Config;
use crate::repository::{Repository, Teithet};
use crate::Error;
use axum::extract::State;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let state = Arc::new(AppState {
        repository: Repository::new(db),
    });

    let app = Router::new().route("/", get(root)).with_state(state);

    let address = format!("0.0.0.0:{}", config.port);
    println!("Serving application on {}", address);
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

struct AppState {
    repository: Repository,
}

pub type ApiResult<T, E = Error> = ::std::result::Result<T, E>;

async fn root(State(app_state): State<Arc<AppState>>) -> ApiResult<Json<Teithet>> {
    let result = app_state
        .repository
        .insert_teithet("test".to_string(), "test description".to_string())
        .await?;

    Ok(Json(result))
}
