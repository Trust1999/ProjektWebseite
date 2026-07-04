use axum::Json;
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize)]
pub struct HelloResponse {
    pub message: String,
}

pub async fn hello() -> Json<HelloResponse> {
    println!("INFO: API hello aufgerufen");
    
    Json(HelloResponse {
        message: "Hallo aus Axum!".to_string(),
    })
}

#[derive(Deserialize)]
pub struct GreetRequest {
    pub name: String,
    pub age: String,
}

#[derive(Serialize)]
pub struct GreetResponse {
    pub greeting: String,
}

pub async fn greet(
    Json(payload): Json<GreetRequest>,
) -> Json<GreetResponse> {
    println!("INFO: API greet aufgerufen");
    
    Json(GreetResponse {
        greeting: format!("Hallo {} 👋, du bist {} Jahre alt.", payload.name, payload.age),
    })
}