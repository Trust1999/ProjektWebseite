use dotenvy::dotenv;
use std::env;
use tokio_postgres::{Client, Error, NoTls};

pub async fn connect_to_db() -> Result<Client, Error> {
    dotenv().ok();

    let params = &env::var("DATABASE_URL").expect("DATABASE_URL fehlt in .env");

    let (client, connection) = tokio_postgres::connect(params, NoTls).await?;

    tokio::spawn(async move {
        if let Err(error) = connection.await {
            eprintln!("Postgres connection error: {}", error);
        }
    });

    Ok(client)
}
