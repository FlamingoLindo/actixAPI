use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserModel {
    pub id: Uuid,
    pub steam_id: String,
    pub username: String,
    pub pf_url: String,
    pub avatar: String,
    pub persona_state: i32,
    pub visibility: i32,

    #[serde(rename = "steamCreatedAt")]
    pub steam_created_at: chrono::DateTime<chrono::Utc>,
    pub current_game: Option<String>,
    pub country: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub enum PersonaState {
//     Offline = 0,
//     Online = 1,
//     Busy = 2,
//     Away = 3,
//     Snooze = 4,
//     LookingToTrade = 5,
//     LookingToPlay = 6,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    pub steam_id: String,
    pub username: String,
    pub pf_url: String,
    pub avatar: String,
    pub persona_state: i32,
    pub visibility: i32,
    pub steam_created_at: chrono::DateTime<chrono::Utc>,
    pub current_game: Option<String>,
    pub country: Option<String>,
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
