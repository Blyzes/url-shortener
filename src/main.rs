use std::net::SocketAddr;

use db::{
    redis::connect_redis,
    sqlite::{init_db, Db},
};
use dotenv::dotenv;
use redis::aio::MultiplexedConnection;
use routes::create_router;
use tokio::net::TcpListener;
use tracing_subscriber;

mod db;
mod handlers;
mod models;
mod routes;
mod templates;
mod utils;

#[tokio::main]
async fn main() {
    // Init the Logger
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db: Db = init_db(&database_url).await;

    println!("Database URL: {}", database_url);

    let redis_client = connect_redis().await;
    let conn = redis_client
        .get_multiplexed_async_connection()
        .await
        .expect("Redis connection failed");

    let state = AppState { db, redis: conn };

    // Build the application with a single route
    let app = create_router(state);

    // Define the address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!("Server is running at http://{addr}");

    // Start the server
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub redis: MultiplexedConnection,
}
