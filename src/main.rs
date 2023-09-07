use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Router,

};
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Error;
use std::env;
use std::net::SocketAddr;
use std::time::Duration;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&getdb_url())
        .await
        .expect("can't connect to database");
    let app = Router::new()
        .route("/", post(save_url))
        .route("/:code", get(handle_url))
        .with_state(pool)
        .route("/", get(home));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
fn getdb_url() -> String {
    dotenv::dotenv().ok();
    format!(
        "postgres://{}:{}@{}/{}",
        env::var("POSTGRES_USER").expect("POSTGRES_USER must be set"),
        env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set"),
        env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string()),
        env::var("POSTGRES_DB").expect("POSTGRES_DB must be set"),
    )
}

async fn home() -> &'static str {
    "Hello, World!"
}
async fn save_url() -> impl IntoResponse {}

async fn handle_url(
    State(pool): State<PgPool>,
    Path((code,)): Path<(String,)>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result : Result<String, Error> = sqlx::query_scalar("SELECT real_url FROM url.urls WHERE hash_url = $1")
        .bind(code.to_string())
        .fetch_one(&pool)
        .await;
    match result {
        Ok(url) => Ok(Redirect::permanent(&url).into_response()),
        Err(_) => Err((StatusCode::NOT_FOUND, "URL not found".to_string())),
    }
}
