mod handler;
mod model;
mod router;

use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;

use crate::router::routes;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let datase_url: String = env::var("DATABASE_URL").expect("can not read env variable");

    let pool: sqlx::Pool<sqlx::Postgres> = sqlx::PgPool::connect(&datase_url)
        .await
        .expect("can not connect to db");

    let app_state: AppState = AppState { pool };

    let app = routes().with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
