use serde::{Deserialize, Serialize};
use axum::{Json, extract::Path, http::StatusCode};
use crate::models;

#[derive(Serialize, Deserialize)]
pub struct Autor {
    pub _id: Option<i32>,
    pub name: String,
    pub land: String,
}

pub async fn autor_get(Path(id): Path<i32>) -> Result<Json<Autor>, StatusCode> {
    println!("INFO: API Get Autor aufgerufen");

     let autor = models::read_autor(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(autor))
}

pub async fn autor_post(Json(autor): Json<Autor>) -> Result<(),StatusCode>{
    println!("INFO: API Post Autor aufgerufen");
    
    models::autor_post(autor)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

