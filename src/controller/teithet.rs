use crate::app::{ApiResult, AppState};
use crate::model::api;
use crate::model::api::teithet::CreateTeithet;
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(root))
        .route("/:id", get(get_single_by_id))
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

async fn root(
    State(app_state): State<Arc<AppState>>,
) -> ApiResult<Json<Vec<api::teithet::Teithet>>> {
    todo!()
}

async fn get_single_by_id(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> ApiResult<Json<api::teithet::Teithet>> {
    let result = app_state.read_service.get_teithet(id).await?;

    Ok(Json(result))
}
