use tokio_postgres::Error;

use crate::author::core::author_id;
use crate::author::core::author_record;
use crate::database;

pub async fn read_autor_repository(
    author_id_record: author_id::AuthorIDRecord,
) -> Result<author_record::AuthorRecord, Error> {
    println!("INFO: Repository Read Autor aufgerufen");

    let client = database::connection::connect_to_db().await?;

    let row = client
        .query_one(
            "SELECT author_id, name FROM author WHERE author_id = $1",
            &[&uuid::Uuid::from(author_id_record)],
        )
        .await?;

    Ok(author_record::AuthorRecord {
        author_id: row.get::<_, uuid::Uuid>("author_id").into(),
        name: row.get(1),
    })
}

pub async fn read_autor_all_repository() -> Result<Vec<author_record::AuthorRecord>, Error> {
    println!("INFO: Repository Read All Autor aufgerufen");

    let client = database::connection::connect_to_db().await?;

    let rows = client
        .query("SELECT author_id, name FROM author", &[])
        .await?;

    let authors = rows
        .iter()
        .map(|row| author_record::AuthorRecord {
            author_id: row.get::<_, uuid::Uuid>("author_id").into(),

            name: row.get("name"),
        })
        .collect();

    Ok(authors)
}
