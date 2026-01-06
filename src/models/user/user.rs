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
    pub gameid: Option<String>,
    pub current_game: Option<String>,
    pub country: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
