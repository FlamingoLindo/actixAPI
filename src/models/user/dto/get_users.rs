use serde::{Deserialize, Serialize};

use crate::models::ResponseStatus;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUsers {
    pub steam_id: String,
    pub username: String,
    pub avatar: String,
    pub pf_url: String,
    pub current_game: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUsersResponse {
    pub status: ResponseStatus,
    pub count: usize,
    pub users: Vec<GetUsers>,
}
