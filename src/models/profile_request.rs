use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProfileRequest {
    pub user_id: String,
    pub bio: String,
    pub is_private: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutProfileRequest {
    pub bio: String,
    pub is_private: bool,
}