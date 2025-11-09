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
