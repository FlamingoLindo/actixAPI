use serde::{Deserialize, Serialize};

use crate::models::ResponseStatus;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub pf_url: Option<String>,
    pub avatar: Option<String>,
    pub persona_state: Option<i32>,
    pub visibility: Option<i32>,
    pub current_game: Option<String>,
    pub country: Option<String>,
    pub gameid: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserResponse {
    pub message: ResponseStatus,
}
