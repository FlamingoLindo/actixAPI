use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserGamesModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub game_id: Uuid,
}
