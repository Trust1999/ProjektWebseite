use axum::{
    Router,
    routing::{get, post},
};

use crate::{handlers::autor};

pub fn create_router() -> Router {
    Router::new()
        .route("/api/autor/{id}", get(autor::autor_get))
        .route("/api/autor", post(autor::autor_post))
}
