use crate::author::core::author_data;
use crate::author::read;
use axum::{Json, http::StatusCode};

pub async fn author_get() -> Result<Json<Vec<author_data::AuthorData>>, StatusCode> {
    println!("INFO: API Get All Autor aufgerufen");

    let author = read::service::author_read_all_service()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let authors_data = author.into_iter().map(|author| author.into()).collect();

    Ok(Json(authors_data))
}
