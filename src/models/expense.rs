use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    // We rename this because Appwrite sends the ID as "$id", 
    // but "$" is not allowed in Rust variable names.
    #[serde(rename = "$id")] 
    pub id: Option<String>, 
    
    pub title: String,
    pub amount: f64,
    pub paid_by: String,
    pub split_among: Vec<String>,
}