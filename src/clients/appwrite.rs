use reqwest::Client;
use std::env;
use dotenv::dotenv;

// This struct holds the configuration we need to talk to Appwrite
#[derive(Clone)]
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
}