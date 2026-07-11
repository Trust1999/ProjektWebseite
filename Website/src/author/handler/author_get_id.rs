use axum::{Json, extract::Path, http::StatusCode};
use crate::author::core::author_data;
use crate::author::read;

pub async fn author_get_id(Path(id): Path<i32>) -> Result<Json<author_data::AuthorData>, StatusCode> {
    println!("INFO: API Get Autor aufgerufen");

     let autor = read::service::author_read_service(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(autor.into()))
}