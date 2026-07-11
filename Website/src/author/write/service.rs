use tokio_postgres::Error;
use crate::author::core::author;
use crate::author::core::author_command;
use crate::author::write::repository;

pub async fn autor_post_service(author_command: author_command::AuthorCommandPost) -> Result<(),Error>{
    let author: author::Author = author_command.into();
    repository::autor_post_repository(author.into()).await?;
    Ok(())
}