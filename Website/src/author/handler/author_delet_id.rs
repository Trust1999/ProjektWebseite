use crate::author::core::author_id;
use crate::author::write;
use axum::{extract::Path, http::StatusCode};

pub async fn author_get_id(
    Path(author_id_data): Path<author_id::AuthorIDData>,
) -> Result<(), StatusCode> {
    println!("INFO: API Get Autor aufgerufen mit der ID");

    write::service::author_delete_service(author_id_data.into())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
