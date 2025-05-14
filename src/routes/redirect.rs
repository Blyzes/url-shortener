use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

use crate::{handlers::redirect::redirect_handler, AppState};

pub async fn redirect(Path(key): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
    redirect_handler(key, state).await
}
