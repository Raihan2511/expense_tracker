use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::{json, Value};
use crate::models::expense::Expense;
use crate::clients::appwrite::AppwriteService;
use std::env;

pub async fn create_expense(
    State(appwrite): State<AppwriteService>,
    Json(payload): Json<Expense>, 
) -> Json<Value> {
    
    // 1. Get the Collection ID from the environment
    let collection_id = env::var("APPWRITE_COLLECTION_ID_EXPENSES")
        .unwrap_or_else(|_| "expenses".to_string());

    // 2. Prepare the data for Appwrite
    // Note: We don't send "id" because Appwrite generates it.
    let data = json!({
        "title": payload.title,
        "amount": payload.amount,
        "paid_by": payload.paid_by,
        "split_among": payload.split_among
    });

    // 3. Send it using the function we just wrote!
    match appwrite.create_document(&collection_id, data).await {
        Ok(response) => Json(response),
        Err(e) => Json(json!({ "error": "Failed to save expense", "details": e }))
    }
}

pub async fn get_expenses(State(appwrite): State<AppwriteService>) -> Json<Value> {
    let collection_id = env::var("APPWRITE_COLLECTION_ID_EXPENSES")
        .unwrap_or_else(|_| "expenses".to_string());

    match appwrite.list_documents(&collection_id).await {
        Ok(expenses) => Json(expenses),
        Err(e) => Json(json!({ "error": "Failed to fetch expenses", "details": e })),
    }
}

pub async fn delete_expense(
    State(appwrite): State<AppwriteService>,
    Path(id): Path<String>,
) -> Json<Value> {
    let collection_id = env::var("APPWRITE_COLLECTION_ID_EXPENSES")
        .unwrap_or_else(|_| "expenses".to_string());

    match appwrite.delete_document(&collection_id, &id).await {
        Ok(_) => Json(json!({ "status": "deleted", "id": id })),
        Err(e) => Json(json!({ "error": "Failed to delete expense", "details": e })),
    }
}

pub async fn update_expense(
    State(appwrite): State<AppwriteService>,
    Path(id): Path<String>,
    Json(payload): Json<Expense> 
) -> Json<Value> {
    
    let collection_id = env::var("APPWRITE_COLLECTION_ID_EXPENSES")
        .unwrap_or_else(|_| "expenses".to_string());

    // We repackage the data just like we did for creating
    let data = json!({
        "title": payload.title,
        "amount": payload.amount,
        "paid_by": payload.paid_by,
        "split_among": payload.split_among
    });

    match appwrite.update_document(&collection_id, &id, data).await {
        Ok(response) => Json(response),
        Err(e) => Json(json!({ "error": "Failed to update expense", "details": e }))
    }
}