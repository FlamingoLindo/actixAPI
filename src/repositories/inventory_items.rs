use sqlx::{Error as SqlxError, PgPool};

use crate::models::inventory_items::{dto::CreateItemSchema, inventory_items::InventoryItemModel};

pub struct InventoryItemsRepository;

impl InventoryItemsRepository {
    pub async fn create_items(
        pool: &PgPool,
        body: CreateItemSchema,
    ) -> Result<InventoryItemModel, SqlxError> {
        sqlx::query_as!(
            InventoryItemModel,
            "INSERT into inventory_items (inventory_id, app_id, classid, icon, name, color, item_type, description)
            values ($1,$2,$3,$4,$5,$6,$7,$8) returning *",
            body.inventory_id,
            body.app_id,
            body.classid,
            body.icon,
            body.name,
            body.color,
            body.item_type,
            body.description,
        )
        .fetch_one(pool)
        .await
    }

    pub async fn check_item_exists(
        pool: &PgPool,
        inventory_id: uuid::Uuid,
        classid: &str,
    ) -> Result<bool, SqlxError> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(
                SELECT 1 FROM inventory_items 
                WHERE inventory_id = $1 AND classid = $2
            )",
            inventory_id,
            classid
        )
        .fetch_one(pool)
        .await?;

        Ok(exists.unwrap_or(false))
    }
}
