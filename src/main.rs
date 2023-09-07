use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get},
    Json, Router,
};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use sqlx::postgres::{PgPool, PgPoolOptions, PgQueryResult};
use sqlx::Error;
use std::net::SocketAddr;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("can't connect to database");
    let app = Router::new()
        .route("/", get(home).post(save_url))
        .route("/:code", get(handle_url))
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
#[derive(serde::Deserialize, Debug)]
struct UrlReq {
    url: String,
}
async fn home() -> &'static str {
    "Hello, World!"
}
fn get_random_string() -> String {
    let rng = thread_rng();
    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();
    random_string
}
async fn save_url(
    State(pool): State<PgPool>,
    req: Option<Json<UrlReq>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let url_req = match req {
        Some(url) => url,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                "You need to send an url via the body".to_string(),
            ))
        }
    };
    let random_string = get_random_string();
    let result: Result<PgQueryResult, Error> =
        sqlx::query("INSERT INTO url.urls (hash_url, real_url) VALUES ($1, $2)")
            .bind(&random_string)
            .bind(&url_req.url)
            .execute(&pool)
            .await;
    match result {
        Ok(_) => {
            let url = format!("http://localhost:3000/{}", random_string);
            Ok((StatusCode::CREATED, url))
        }
        Err(err) => {
            println!("Error: {}", err);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Error".to_string()));
        }
    }
}

async fn handle_url(
    State(pool): State<PgPool>,
    Path((code,)): Path<(String,)>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result: Result<String, Error> =
        sqlx::query_scalar("SELECT real_url FROM url.urls WHERE hash_url = $1")
            .bind(code.to_string())
            .fetch_one(&pool)
            .await;
    match result {
        Ok(url) => Ok(Redirect::permanent(&url).into_response()),
        Err(err) => {
            println!("Error: {}", err);
            Err((StatusCode::NOT_FOUND, "URL not found".to_string()))
        }
    }
}
