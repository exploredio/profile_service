use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub profile_id: String,
    pub user_id: String,
    pub bio: String,
    pub is_private: bool,
}