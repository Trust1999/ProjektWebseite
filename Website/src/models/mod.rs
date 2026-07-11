use tokio_postgres::{self, Error};

use crate::handlers::autor::Autor;
use crate::database;

pub async fn read_autor(id: i32) -> Result<Autor, Error> {
    let client = database::connection::connect_to_db().await?;

    let row = client.query_one(
        "SELECT id, name, land FROM autor WHERE id = $1",
        &[&id],
    )
    .await?;

    Ok(Autor {
        _id: Some(row.get(0)),
        name: row.get(1),
        land: row.get(2),
    })
}

pub async fn autor_post(autor: Autor) -> Result<(),Error>{
     let client = database::connection::connect_to_db().await?;
    
    client.execute(
            "INSERT INTO autor(name, land) VALUES($1, $2)", 
            &[&autor.name, &autor.land])
    .await?;
    
    Ok(())
}