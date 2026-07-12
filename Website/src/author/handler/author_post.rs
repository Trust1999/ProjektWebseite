use crate::author::core::author_data;
use crate::author::write::service;
use axum::{Json, http::StatusCode};

pub async fn autor_post(Json(autor): Json<author_data::AuthorData>) -> Result<(), StatusCode> {
    println!("INFO: API Post Autor aufgerufen");

    service::autor_post_service(autor.into())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}
