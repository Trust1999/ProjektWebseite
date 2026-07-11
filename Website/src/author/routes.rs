use axum::{
    Router,
    routing::{get, post},
};

use crate::author::handler;


pub fn router() -> Router {

    Router::new()
       .route(
            "/author", 
            post(handler::author_post::autor_post))

        .route(
            "/author/{id}",
            get(handler::author_get_id::author_get_id))
}