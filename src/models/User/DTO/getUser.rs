use serde::{Deserialize, Serialize};
use crate::models::responseStatus::ResponseStatus;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUser {
    pub steam_id: String,
    pub username: String,
    pub avatar: String,
    pub pf_url: String,
    pub country: Option<String>,
    pub current_game: Option<String>,
    pub persona_state: i32,
    pub visibility: i32,
    pub steam_created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUserResponse {
    pub status: ResponseStatus,
    pub user: GetUser,
}