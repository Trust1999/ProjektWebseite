use tokio_postgres::Error;

use crate::author::core::{author_id, author_record};
use crate::database;

pub async fn autor_post_repository(
    author_record: author_record::AuthorRecord,
) -> Result<(), Error> {
    println!("INFO: Repository Post Autor aufgerufen");

    let client = database::connection::connect_to_db().await?;
    client
        .execute(
            "INSERT INTO author (author_id, name) VALUES ($1, $2)",
            &[
                &uuid::Uuid::from(author_record.author_id),
                &author_record.name,
            ],
        )
        .await?;

    Ok(())
}

pub async fn autor_delet_repository(
    author_id_record: author_id::AuthorIDRecord,
) -> Result<(), Error> {
    println!("INFO: Repository Delete Autor aufgerufen");

    let client = database::connection::connect_to_db().await?;
    client
        .execute(
            "DELETE FROM Author WHERE author_id = $1",
            &[&uuid::Uuid::from(author_id_record)],
        )
        .await?;

    Ok(())
}

pub async fn autor_patch_repository(
    author_record: author_record::AuthorRecord,
) -> Result<(), Error> {
    println!("INFO: Repository Patch Autor aufgerufen");

    let client = database::connection::connect_to_db().await?;
    client
        .execute(
            "UPDATE Author SET name = $1 WHERE author_id = $2",
            &[
                &author_record.name,
                &uuid::Uuid::from(author_record.author_id),
            ],
        )
        .await?;

    Ok(())
}
