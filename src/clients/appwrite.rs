use reqwest::Client;
use std::env;
use dotenv::dotenv;

// This struct holds the configuration we need to talk to Appwrite
#[derive(Clone)]
#[allow(dead_code)]
pub struct AppwriteService {
    client: Client,
    pub endpoint: String,
    pub project_id: String,
    pub api_key: String,
    pub database_id: String,
}

impl AppwriteService {
    // This function runs when we start the server. 
    // It loads the configuration and creates the service.
    pub fn new() -> Self {
        // 1. Load the .env file
        dotenv().ok(); 

        // 2. Create the struct with values from the environment
        AppwriteService {
            client: Client::new(),
            endpoint: env::var("APPWRITE_ENDPOINT").expect("APPWRITE_ENDPOINT must be set"),
            project_id: env::var("APPWRITE_PROJECT_ID").expect("APPWRITE_PROJECT_ID must be set"),
            api_key: env::var("APPWRITE_API_KEY").expect("APPWRITE_API_KEY must be set"),
            database_id: env::var("APPWRITE_DATABASE_ID").expect("APPWRITE_DATABASE_ID must be set"),
        }
    }

    // The new function to save data
    pub async fn create_document(
        &self, 
        collection_id: &str, 
        data: serde_json::Value
    ) -> Result<serde_json::Value, String> {
        
        // 1. Construct the specific URL for this collection
        let url = format!(
            "{}/databases/{}/collections/{}/documents", 
            self.endpoint, 
            self.database_id, 
            collection_id
        );

        // 2. Send the POST request
        let response = self.client.post(&url)
            .header("X-Appwrite-Project", &self.project_id)
            .header("X-Appwrite-Key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "documentId": "unique()", // Ask Appwrite to generate a random ID
                "data": data
            }))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        // 3. Check if it worked
        if response.status().is_success() {
            Ok(response.json().await.map_err(|e| e.to_string())?)
        } else {
            Err(format!("Error: {:?}", response.text().await))
        }
    }

    pub async fn delete_document(&self, collection_id: &str, document_id: &str) -> Result<(), String> {
        let url = format!(
            "{}/databases/{}/collections/{}/documents/{}",
            self.endpoint,
            self.database_id,
            collection_id,
            document_id
        );

        let response = self.client.delete(&url) // <--- DELETE request
            .header("X-Appwrite-Project", &self.project_id)
            .header("X-Appwrite-Key", &self.api_key)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("Error deleting document: {:?}", response.text().await))
        }
    }

    pub async fn list_documents(&self, collection_id: &str) -> Result<serde_json::Value, String> {
        let url = format!(
            "{}/databases/{}/collections/{}/documents",
            self.endpoint,
            self.database_id,
            collection_id
        );

        let response = self.client.get(&url)
            .header("X-Appwrite-Project", &self.project_id)
            .header("X-Appwrite-Key", &self.api_key)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if response.status().is_success() {
            Ok(response.json().await.map_err(|e| e.to_string())?)
        } else {
            Err(format!("Error fetching documents: {:?}", response.text().await))
        }
    }


    // ... delete_document is above this ...

    pub async fn update_document(
        &self, 
        collection_id: &str, 
        document_id: &str, 
        data: serde_json::Value
    ) -> Result<serde_json::Value, String> {
        let url = format!(
            "{}/databases/{}/collections/{}/documents/{}",
            self.endpoint,
            self.database_id,
            collection_id,
            document_id
        );

        // We use PATCH for updates (partial changes)
        let response = self.client.patch(&url)
            .header("X-Appwrite-Project", &self.project_id)
            .header("X-Appwrite-Key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "data": data
            }))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if response.status().is_success() {
            Ok(response.json().await.map_err(|e| e.to_string())?)
        } else {
            Err(format!("Error updating document: {:?}", response.text().await))
        }
    }

    
}