use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    pub steam_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreationResponse {
    pub username: String,
    pub pf_url: String,
    pub avatar: String,
}
