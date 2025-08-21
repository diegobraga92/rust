use axum::{
    Router,
    routing::{get, post},
};
use diesel_migrations::{EmbeddedMigrations, embed_migrations};
use std::net::SocketAddr;
use tracing_subscriber;

mod db;
mod error;
mod models;
mod routes;
mod schema;

use routes::health::health_check;
use routes::user::{create_user, list_users};

// Diesel embedded migrations
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();
    // Start DB connection pool
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .expect("Failed to create database connection pool");

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(health_check))
        .route("/user/list", get(list_users))
        .route("/user/create", post(create_user))
        .with_state(pool); // with_state allows the resource to be accessed as an State argument of type X

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
