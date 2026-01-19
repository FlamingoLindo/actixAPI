use sqlx::PgPool;

use crate::{
    models::inventory::dto::{CreateInventoryResponse, CreateInventorySchema},
    repositories::inventory_repository::InventoryRepository,
    services::errors::inventory::create_erros::CreateInventoryError,
};

pub struct InventoryService;

impl InventoryService {
    pub async fn create_inventory(
        pool: &PgPool,
        steam_id: String,
    ) -> Result<CreateInventoryResponse, CreateInventoryError> {
        let existing_inventory =
            InventoryRepository::check_if_user_has_inventory(pool, &steam_id).await?;
        if existing_inventory {
            return Err(CreateInventoryError::InventoryAlreadyExists);
        }

        let create_schema = CreateInventorySchema { steam_id };

        let inventory = InventoryRepository::create_inventory(pool, create_schema).await?;

        Ok(CreateInventoryResponse {
            steam_id: inventory.user_id.to_string(),
            inventory_id: inventory.id,
        })
    }
}
