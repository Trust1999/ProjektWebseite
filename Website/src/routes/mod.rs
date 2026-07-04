use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::hello;

pub fn create_router() -> Router {
    Router::new()
        .route("/api/hello", get(hello::hello))
        .route("/api/greet", post(hello::greet))
}