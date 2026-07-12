use crate::author::core::author;
use crate::author::core::author_command;
use crate::author::core::author_id::AuthorID;
use crate::author::core::author_record;
use crate::author::read;
use crate::author::write::repository;
use tokio_postgres::Error;

pub async fn autor_post_service(
    author_command: author_command::AuthorCommandPost,
) -> Result<(), Error> {
    println!("INFO: Service Post Autor aufgerufen");

    let author: author::Author = author_command.into();
    repository::autor_post_repository(author.into()).await?;
    Ok(())
}

pub async fn author_delete_service(author_id: AuthorID) -> Result<(), Error> {
    println!("INFO: Service Delete Autor aufgerufen");

    repository::autor_delet_repository(author_id.into()).await?;
    Ok(())
}

pub async fn autor_patch_service(
    author_command: author_command::AuthorCommandPatch,
    author_id: AuthorID,
) -> Result<author::Author, Error> {
    println!("INFO: Service Patch Autor aufgerufen");

    let author_record = author_record::AuthorRecord {
        author_id: author_command.author_id.into(),
        name: author_command.name,
    };

    repository::autor_patch_repository(author_record).await?;
    let author_record = read::repository::read_autor_repository(author_id.into()).await?;

    Ok(author_record.into())
}
