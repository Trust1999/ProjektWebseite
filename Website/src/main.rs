mod handlers;
mod models;
mod routes;
mod state;

use axum::Router;
use routes::create_router;
use tokio::net::TcpListener;

use tower_http::cors::{
    Any,
    CorsLayer,
};

#[tokio::main]
async fn main() {

    let cors = CorsLayer::new()
        .allow_origin(Any)
        //.allow_origin("http://localhost:5173".parse().unwrap())
        .allow_methods(Any)
        .allow_headers(Any);

    let app = create_router()
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server läuft auf http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}