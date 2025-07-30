use crate::{
    db::redis::{get_cache, set_cache},
    models::{ShortenRequest, ShortenResponse},
    AppState,
};
use rand::distributions::Alphanumeric;
use rand::Rng;
use sqlx::query_scalar;

pub async fn shorten_handler(payload: ShortenRequest, mut state: AppState) -> ShortenResponse {
    let url_key = format!("{}", payload.url);

    // 1. 查 Redis 是否已有原始链接对应的短链 key
    if let Some(existing_key) = get_cache(&mut state.redis, &url_key).await {
        return ShortenResponse {
            short_url: format!("http://localhost:3000/{}", existing_key),
        };
    }

    // 2. 查数据库是否已有记录
    if let Ok(Some(existing_key)) = query_scalar::<_, String>(
        "SELECT `key` FROM links WHERE url = ?"
    )
    .bind(&payload.url)
    .fetch_optional(&*state.db)
    .await
    {
        // existing_key 是 String
        let _ = set_cache(&mut state.redis, &url_key, &existing_key, 86400 * 30).await;
        return ShortenResponse {
            short_url: format!("http://localhost:3000/{}", existing_key),
        };
    }
        

    // 3. 都没命中，生成新的短链接 key
    let key: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();

    // 插入数据库
    sqlx::query!(
        "INSERT INTO links (`key`, url) VALUES (?, ?)",
        key,
        payload.url
    )
    .execute(&*state.db)
    .await
    .expect("Failed to insert link");

    // 写入 Redis：双向缓存
    let _ = set_cache(&mut state.redis, &key, &payload.url, 86400 * 30).await;
    let _ = set_cache(&mut state.redis, &url_key, &key, 86400 * 30).await;

    ShortenResponse {
        short_url: format!("http://localhost:3000/{key}"),
    }
}
