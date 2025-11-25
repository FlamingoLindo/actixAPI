use serde::{Deserialize, Serialize};

use crate::models::ResponseStatus;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUsers {
    pub steam_id: String,
    pub username: String,
    pub avatar: String,
    pub pf_url: String,
    pub current_game: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginationMeta {
    pub total_in_page: usize,
    pub total: i64,
    pub total_pages: i64,
    pub current_page: i64,
    pub page_size: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUsersResponse {
    pub status: ResponseStatus,
    pub users: Vec<GetUsers>,
    pub pagination: PaginationMeta,
}
