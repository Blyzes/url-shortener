use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

use crate::handlers::stats::stats_handler;
use crate::AppState;

pub async fn stats(Path(key): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
    stats_handler(key, state).await
}
