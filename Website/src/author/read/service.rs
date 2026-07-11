use tokio_postgres::Error;
use crate::author::core::author;
use crate::author::read;

pub async fn author_read_service(id: i32) -> Result<author::Author, Error>{
    let author = read::repository::read_autor_repository(id).await?;
    Ok(author.into())
}