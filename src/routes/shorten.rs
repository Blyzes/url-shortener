use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    handlers::shorten::shorten_handler,
    models::{ShortenRequest, ShortenResponse},
    AppState,
};

pub async fn shorten(
    State(state): State<AppState>,
    Json(payload): Json<ShortenRequest>,
) -> impl IntoResponse {
    let res: ShortenResponse = shorten_handler(payload, state).await;
    Json(res)
}
