use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};
use redirect::redirect;
use shorten::shorten;
use stats::stats;
use stats_page::stats_page;

mod redirect;
mod shorten;
mod stats;
mod stats_page;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root_handler))
        .route("/shorten", post(shorten))
        .route("/:key", get(redirect))
        .route("/stats/:key", get(stats))
        .route("/stats/html/:key", get(stats_page))
        .with_state(state)
}

async fn root_handler() -> &'static str {
    "URL shortener is running"
}
