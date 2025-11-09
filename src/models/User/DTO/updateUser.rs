use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserSchema {
    pub username: Option<String>,
    pub pf_url: Option<String>,
    pub avatar: Option<String>,
    pub persona_state: Option<i32>,
    pub visibility: Option<i32>,
    pub current_game: Option<String>,
    pub country: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserUpdateResponse {
    pub username: String,
    pub pf_url: String,
    pub avatar: String,
}
