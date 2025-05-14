use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use sqlx::query;

use crate::{
    db::redis::{get_cache, set_cache},
    AppState,
};

pub async fn redirect_handler(key: String, mut state: AppState) -> impl IntoResponse {
    if let Some(cached_url) = get_cache(&mut state.redis, &key).await {
        return Redirect::temporary(&cached_url).into_response();
    }

    let result = query!("SELECT url FROM links WHERE `key` = ?", key)
        .fetch_optional(&*state.db)
        .await;

    match result {
        Ok(Some(record)) => {
            let _ = query!("UPDATE links SET clicks = clicks + 1 WHERE `key` = ?", key)
                .execute(&*state.db)
                .await;
            // 存入 Redis 缓存，设置过期时间 1 小时
            set_cache(&mut state.redis, &key, &record.url, 3600).await;
            Redirect::temporary(&record.url).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, "URL not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response(),
    }
}
