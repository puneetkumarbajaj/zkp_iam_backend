// src/models.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32, // In a real application, you might use UUIDs or similar
    pub username: String,
    // Public data derived from the user's secret; this could be a hash, for example.
    // In a real ZKP scenario, this would be used for verification without storing the actual secret
    pub public_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub public_data: String, // Client sends public data derived from their secret
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub proof: String, // A ZKP proof sent for login
}
