//! Api: the HTTP front desk. (Given: you wrote this in 26-3.)

use crate::domain::{BoardError, NewQuest, Quest, Summary};
use crate::store::{AppError, Store};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct Filter {
    pub done: Option<bool>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::Board(BoardError::EmptyTitle | BoardError::BadReward(_)) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::Board(BoardError::NotFound(_)) => StatusCode::NOT_FOUND,
            AppError::Board(BoardError::AlreadyDone(_)) => StatusCode::CONFLICT,
            AppError::Db(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let message = match &self {
            AppError::Db(_) => "internal error".to_string(),
            other => other.to_string(),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}

async fn list(State(store): State<Store>, Query(f): Query<Filter>) -> Result<Json<Vec<Quest>>, AppError> {
    Ok(Json(store.list(f.done).await?))
}

async fn add(State(store): State<Store>, Json(new): Json<NewQuest>) -> Result<(StatusCode, Json<Quest>), AppError> {
    Ok((StatusCode::CREATED, Json(store.add(new).await?)))
}

async fn get_one(State(store): State<Store>, Path(id): Path<i64>) -> Result<Json<Quest>, AppError> {
    Ok(Json(store.get(id).await?))
}

async fn finish(State(store): State<Store>, Path(id): Path<i64>) -> Result<Json<Quest>, AppError> {
    Ok(Json(store.finish(id).await?))
}

async fn summary(State(store): State<Store>) -> Result<Json<Summary>, AppError> {
    Ok(Json(Summary::of(&store.list(None).await?)))
}

pub fn router(store: Store) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/quests", get(list).post(add))
        .route("/quests/{id}", get(get_one))
        .route("/quests/{id}/finish", post(finish))
        .route("/summary", get(summary))
        .with_state(store)
}
