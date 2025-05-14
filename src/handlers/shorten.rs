use uuid::Uuid;

use crate::{
    db::redis::set_cache,
    models::{ShortenRequest, ShortenResponse},
    AppState,
};
use sqlx::query;

pub async fn shorten_handler(payload: ShortenRequest, mut state: AppState) -> ShortenResponse {
    let key = Uuid::new_v4().to_string()[..6].to_string();

    query!(
        "INSERT INTO links (`key`, url) VALUES (?, ?)",
        key,
        payload.url
    )
    .execute(&*state.db)
    .await
    .expect("Failed to insert link");

    set_cache(&mut state.redis, &key, &payload.url, 86400 * 30).await;

    ShortenResponse {
        short_url: format!("http://localhost:3000/{key}"),
    }
}
