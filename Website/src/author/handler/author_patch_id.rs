use crate::author::core::author_command;
use crate::author::core::{author_data, author_id};
use crate::author::write::service;
use axum::{Json, extract::Path, http::StatusCode};

#[axum::debug_handler]
pub async fn autor_patch_id(
    Path(author_id_data): Path<author_id::AuthorIDData>,
    Json(author_data): Json<author_data::AuthorData>,
) -> Result<Json<author_data::AuthorData>, StatusCode> {
    println!("INFO: API Post Autor aufgerufen");

    let author_command =
        author_command::AuthorCommandPatch::create(author_data, author_id_data.clone());

    let author = service::autor_patch_service(author_command, author_id_data.into())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(author.into()))
}
