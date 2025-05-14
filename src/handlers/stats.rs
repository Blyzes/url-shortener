use axum::response::IntoResponse;
use serde::Serialize;
use sqlx::query_as;

use crate::AppState;

#[derive(Serialize)]
pub struct StatsResponse {
    url: String,
    clicks: i64,
}

pub async fn stats_handler(key: String, state: AppState) -> impl IntoResponse {
    let result = query_as!(
        StatsResponse,
        "SELECT url, clicks FROM links WHERE `key` = ?",
        key
    )
    .fetch_optional(&*state.db)
    .await;

    match result {
        Ok(Some(stats)) => axum::Json(stats).into_response(),
        Ok(None) => (axum::http::StatusCode::NOT_FOUND, "Key not found").into_response(),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response(),
    }
}
