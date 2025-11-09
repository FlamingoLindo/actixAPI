use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
    Success,
    Error,
}

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
