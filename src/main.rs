use axum::{
    routing::{get, post}, // <--- We now import 'post' as well
    Router,
};
use std::net::SocketAddr;

// Register the modules so Rust knows they exist
mod clients;
mod handlers;
mod models;
mod error; 

use clients::appwrite::AppwriteService;
// Import the actual logic function we wrote
use handlers::expense::{create_expense, get_expenses, delete_expense, update_expense}; // <--- Add update_expense
// use handlers::user::*; // Uncomment if needed, or keeping it clean for now.

#[tokio::main]
async fn main() {
    // 1. Load environment variables
    dotenv::dotenv().ok();

    // 2. Initialize the Appwrite Service (The Master Key)
    let appwrite_service = AppwriteService::new();
    println!("✅ Appwrite Service initialized!");

    // 3. Define the Routes
    let app = Router::new()
        .route("/", get(health_check))
        // We use POST here because we are CREATING an expense
        .route("/expenses", post(create_expense).get(get_expenses))
        // NEW ROUTE: Note the "/:id" part!
        .route("/expenses/:id", axum::routing::delete(delete_expense).patch(update_expense)) 
        .with_state(appwrite_service);

    // 4. Start the Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server started on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// A simple check to ensure the server is up
async fn health_check() -> &'static str {
    "Expense Tracker Backend is Running!"
}