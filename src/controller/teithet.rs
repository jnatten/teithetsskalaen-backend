use crate::app::{ApiResult, AppState};
use crate::model::api;
use crate::model::api::teithet::CreateTeithet;
use crate::repository::Teithet;
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(root))
        .route("/", post(create_new_teithet))
}

async fn create_new_teithet(
    State(app_state): State<Arc<AppState>>,
    Json(input): Json<CreateTeithet>,
) -> ApiResult<Json<api::teithet::Teithet>> {
    let result = app_state
        .write_service
        .new_teithet(input.title, input.description)
        .await?;
    Ok(Json(result))
}

async fn root(State(app_state): State<Arc<AppState>>) -> ApiResult<Json<Teithet>> {
    let result = app_state
        .repository
        .insert_teithet("test".to_string(), "test description".to_string())
        .await?;

    println!("Hallo kjære!");

    Ok(Json(result))
}
