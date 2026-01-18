use axum::{routing::get, Router};
use std::net::SocketAddr;

// 1. We need to tell Rust to look at our "clients" folder.
// What line goes here to import the clients module?
mod clients; 
mod handlers; // We'll need this later too
mod models;

use clients::appwrite::AppwriteService;

#[tokio::main]
async fn main() {
    // 2. Initialize the Appwrite Service
    // This calls the 'new()' function you wrote in src/clients/appwrite.rs
    let appwrite_service = AppwriteService::new();

    println!("✅ Connection to Appwrite configured!");

    // 3. Create the Router
    // We attach the service to the router so our handlers can access it later.
    let app = Router::new()
        .route("/", get(health_check))
        .with_state(appwrite_service); // <--- This is the magic sauce!

    // 4. Run the Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server started on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// A simple check to ensure the server is up
async fn health_check() -> &'static str {
    "Expense Tracker Backend is Running!"
}