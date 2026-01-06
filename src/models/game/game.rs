use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct GameModel {
    pub id: Uuid,
    pub appid: String,
    pub name: String,
    pub short_description: Option<String>,
    pub header_image: Option<String>,
    pub screenshots: Option<Vec<String>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct GameCreationResponse {
    pub appid: String,
    pub name: String,
    pub short_description: Option<String>,
    pub header_image: Option<String>,
    pub screenshots: Option<Vec<String>>,
}
