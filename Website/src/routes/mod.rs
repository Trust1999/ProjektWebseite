use crate::author;
use axum::Router;

pub fn create_router() -> Router {
    Router::new().merge(author::routes::router())
}
