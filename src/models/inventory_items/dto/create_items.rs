use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateItemSchema {
    pub inventory_id: Uuid,
    pub app_id: String,
    pub classid: String,
    pub icon: String,
    pub name: String,
    pub color: String,
    pub item_type: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemCreationResponse {
    pub app_id: String,
    pub icon: String,
    pub name: String,
    pub color: String,
    pub item_type: String,
    pub description: String,
}
