// src/main.rs

mod models;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use models::{RegisterRequest, LoginRequest};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is up and running!")
}

// Placeholder for registration; would involve storing the user's public data
async fn register_user(info: web::Json<RegisterRequest>) -> impl Responder {
    println!("Registering user: {}", info.username);
    // Here, you'd store the username and public_data in a database
    HttpResponse::Ok().json(info.0) // Echo back the received info
}

// Placeholder for login; would involve verifying the ZKP proof
async fn login_user(info: web::Json<LoginRequest>) -> impl Responder {
    println!("User login attempt: {}", info.username);
    // Here, you'd fetch the user's stored public data and verify the provided ZKP proof
    // Let's assume verification is always successful for this example
    HttpResponse::Ok().body("Login successful")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
