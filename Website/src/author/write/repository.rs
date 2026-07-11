use tokio_postgres::Error;

use crate::author::core::author_record;
use crate::database;

pub async fn autor_post_repository(autor: author_record::AuthorRecord) -> Result<(),Error>{
     let client = database::connection::connect_to_db().await?;
    
    client.execute(
            "INSERT INTO autor(name) VALUES($1)", 
            &[&autor.name])
    .await?;
    
    Ok(())
}