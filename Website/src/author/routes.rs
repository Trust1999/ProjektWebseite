use axum::{
    Router,
    routing::{get, post},
};

use crate::author::handler;

pub fn router() -> Router {
    Router::new()
        .route(
            "/author",
            post(handler::author_post::autor_post).get(handler::author_get::author_get),
        )
        .route(
            "/author/{author_id}",
            get(handler::author_get_id::author_get_id)
                .delete(handler::author_delet_id::author_get_id)
                .patch(handler::author_patch_id::autor_patch_id),
        )
}
