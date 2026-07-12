use crate::author::core::{author_data, author_id};
use crate::author::read;
use axum::{Json, extract::Path, http::StatusCode};

pub async fn author_get_id(
    Path(author_id_data): Path<author_id::AuthorIDData>,
) -> Result<Json<author_data::AuthorData>, StatusCode> {
    println!("INFO: API Get Autor aufgerufen mit der ID");

    let author = read::service::author_read_service(author_id_data.into())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(author.into()))
}
