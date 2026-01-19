use sqlx::{Error as SqlxError, PgPool};

use crate::models::inventory::{dto::CreateInventorySchema, inventory::InventoryModel};

pub struct InventoryRepository;

impl InventoryRepository {
    pub async fn check_if_user_has_inventory(
        pool: &PgPool,
        steam_id: &str,
    ) -> Result<bool, SqlxError> {
        let existing_query = sqlx::query_scalar!(
            "SELECT EXISTS(
            SELECT 1 
            FROM users u
            INNER JOIN inventories i ON u.id = i.user_id
            WHERE u.steam_id = $1
        );",
            steam_id
        )
        .fetch_one(pool)
        .await?;

        Ok(existing_query.unwrap_or(false))
    }

    pub async fn create_inventory(
        pool: &PgPool,
        body: CreateInventorySchema,
    ) -> Result<InventoryModel, SqlxError> {
        sqlx::query_as!(
            InventoryModel,
            "INSERT into inventories (user_id) 
            SELECT id FROM users WHERE steam_id = $1 
            RETURNING *",
            body.steam_id
        )
        .fetch_one(pool)
        .await
    }
}
