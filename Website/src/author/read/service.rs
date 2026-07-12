use crate::author::core::author::{self};
use crate::author::core::author_id;
use crate::author::read;
use tokio_postgres::Error;

pub async fn author_read_service(auhor_id: author_id::AuthorID) -> Result<author::Author, Error> {
    println!("INFO: Service Read Autor aufgerufen");

    let author = read::repository::read_autor_repository(auhor_id.into()).await?;
    Ok(author.into())
}

pub async fn author_read_all_service() -> Result<Vec<author::Author>, Error> {
    println!("INFO: Service Read Autor aufgerufen");

    let authors_repository = read::repository::read_autor_all_repository().await?;
    let authors = authors_repository
        .into_iter()
        .map(|auhor| auhor.into())
        .collect();

    Ok(authors)
}
