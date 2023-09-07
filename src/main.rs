use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
     extract::Path
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", post(save_url))
        .route("/", get(home))
        .route("/:code", get(handle_url));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn home() -> &'static str {
    "Hello, World!"
}
async fn save_url() -> &'static str {
    "Hello, World!"
}

async fn handle_url(Path((code,)): Path<(String,)>) -> String {
    format!("You passed in the code: {}", code)
}
