use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    // Again, we map "$id" to "id"
    #[serde(rename = "$id")]
    pub id: String,

    pub username: String,
    pub email: String,
    // We don't store passwords here for security!
}