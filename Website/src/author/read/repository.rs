use tokio_postgres::Error;

use crate::author::core::author_record;
use crate::database;

pub async fn read_autor_repository(id: i32) -> Result<author_record::AuthorRecord, Error> {
    let client = database::connection::connect_to_db().await?;

    let row = client.query_one(
        "SELECT id, name, land FROM autor WHERE id = $1",
        &[&id],
    )
    .await?;

    Ok(author_record::AuthorRecord {
        _id: row.get(0),
        name: row.get(1),
    })
}