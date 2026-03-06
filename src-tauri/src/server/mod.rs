use crate::app_state::AppState;
use axum::http::Method;
use axum::{routing::get, Router};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

mod routes;

pub async fn start(state: Arc<AppState>) {
    let app = Router::new()
        .route("/api/icons", get(routes::list_icons))
        .route("/overlay/countdown", get(routes::overlay_countdown))
        .route("/sse/countdown/{id}", get(routes::sse_countdown))
        .nest_service("/static", ServeDir::new("public"))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods([Method::GET]))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7420")
        .await
        .expect("failed to bind overlay server on :7420");

    axum::serve(listener, app)
        .await
        .expect("overlay server crashed");
}
