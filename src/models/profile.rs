use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub user_id: String,
    pub profile_id: String,
    pub bio: String,
    pub visibility: String,
}