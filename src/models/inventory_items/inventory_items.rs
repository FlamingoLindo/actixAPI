use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct InventoryModel {
    pub id: Uuid,
    pub inventory_id: Uuid,
    pub app_id: String,
    pub icon: String,
    pub name: String,
    pub color: String,
    pub item_type: String,
    pub description: String,
}
