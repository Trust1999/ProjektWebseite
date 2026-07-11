mod handlers;
mod models;
mod routes;
mod database;

#[tokio::main]
async fn main() {

    // Erzeugt Axum router
    let router01 = routes::create_router();
    
    // IP und port listener definieren
    let addresse = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addresse).await.expect("Fehler");

    //axum server starten
    axum::serve(listener, router01).await.expect("Fehler");
}