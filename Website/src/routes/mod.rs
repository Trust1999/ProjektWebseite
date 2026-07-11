use axum::Router;
use crate::author;


pub fn create_router() -> Router {

    Router::new()
        .merge(author::routes::router())

}
