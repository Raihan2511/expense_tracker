use axum::{extract::State, Json};
use serde_json::{json, Value};
use crate::clients::appwrite::AppwriteService;

// This function borrows the "AppwriteService" from the server state!
pub async fn test_connection(
    State(appwrite): State<AppwriteService>
) -> Json<Value> {
    // We are just returning a success message for now
    Json(json!({
        "status": "success",
        "message": "I have access to Appwrite keys!",
        "project_id": appwrite.project_id // Proves we can read the config
    }))
}