use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

use crate::{handlers::stats_page::stats_page_handler, AppState};

pub async fn stats_page(
    Path(key): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    stats_page_handler(key, state).await
}
