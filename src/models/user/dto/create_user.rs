use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserRequest {
    pub steam_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    pub steam_id: String,
    pub personaname: String,
    pub profileurl: String,
    pub avatar: String,
    pub personastate: i32,
    pub communityvisibilitystate: i32,
    pub formatted_steam_created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub gameextrainfo: Option<String>,
    pub loccountrycode: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreationResponse {
    pub username: String,
    pub pf_url: String,
    pub avatar: String,
}
