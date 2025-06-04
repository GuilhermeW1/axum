use axum::{Router, extract::State, routing::get, serve::Listener};
use dotenvy::dotenv;
use std::{env, process::Output};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let datase_url: String = env::var("DATABASE_URL").expect("can not read env variable");

    let pool = sqlx::PgPool::connect(&datase_url)
        .await
        .expect("can not connect to db");

    let app = Router::new().route("/", get(|| async { "hello world" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
