use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct BindUserToGameSchema {
    pub user_id: Uuid,
    pub game_id: Uuid,
}
