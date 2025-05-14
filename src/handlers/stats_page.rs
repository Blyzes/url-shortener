use crate::{templates::stats::StatsTemplate, AppState};
use axum::response::IntoResponse;
use sqlx::query_as;

#[derive(Debug)]
struct StatsRow {
    url: String,
    clicks: i64,
}

pub async fn stats_page_handler(key: String, state: AppState) -> impl IntoResponse {
    let result = query_as!(
        StatsRow,
        "SELECT url, clicks FROM links WHERE `key` = ?",
        key
    )
    .fetch_optional(&*state.db)
    .await;

    match result {
        Ok(Some(data)) => {
            let template = StatsTemplate {
                key,
                url: data.url,
                clicks: data.clicks,
            };
            template.into_response()
        }
        Ok(None) => (axum::http::StatusCode::NOT_FOUND, "Key not found").into_response(),
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response(),
    }
}
