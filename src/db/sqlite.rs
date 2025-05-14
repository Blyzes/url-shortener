use std::sync::Arc;

use sqlx::mysql::MySqlPool;

pub type Db = Arc<MySqlPool>;

pub async fn init_db(database_url: &str) -> Db {
    // max connections number
    let pool = MySqlPool::connect(database_url)
        .await
        .expect("Failed to connect to SQLite");
    Arc::new(pool)
}
