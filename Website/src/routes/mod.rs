use axum::{
    routing::get,
    Router,
};

use crate::handlers::hello;

pub fn create_router() -> Router {
    Router::new()
        .route("/api/hello", get(hello::hello))
}