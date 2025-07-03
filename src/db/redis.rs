use redis::{aio::MultiplexedConnection, AsyncCommands, Client};
use std::sync::Arc;

pub type RedisPool = Arc<Client>;

pub async fn connect_redis() -> RedisPool {
    let client = Client::open("redis://redis/").expect("Invalid Redis URL");
    Arc::new(client)
}

pub async fn get_cache(conn: &mut MultiplexedConnection, key: &str) -> Option<String> {
    conn.get(&key).await.ok()
}

pub async fn set_cache(conn: &mut MultiplexedConnection, key: &str, value: &str, ttl_secs: usize) {
    let _: () = redis::pipe()
        .set(key, value)
        .expire(key, ttl_secs.try_into().unwrap())
        .query_async(conn)
        .await
        .expect("Failed to set cache");
}
