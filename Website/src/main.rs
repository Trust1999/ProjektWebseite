mod handlers;
mod models;
mod routes;
mod state;

use axum::Router;
use routes::create_router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app: Router = create_router();

    let listener: TcpListener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server läuft auf http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}