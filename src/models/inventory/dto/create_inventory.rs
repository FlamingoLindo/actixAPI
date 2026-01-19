use uuid::Uuid;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateInventorySchema {
    pub steam_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateInventoryResponse {
    pub steam_id: String,
    pub inventory_id: Uuid,
}
