use sqlx::PgPool;

use crate::{
    models::inventory_items::dto::{CreateItemSchema, ItemCreationResponse},
    repositories::inventory_items::InventoryItemsRepository,
    repositories::inventory_repository::InventoryRepository,
    services::errors::inventory_items::create_erros::CreateInventoryItemError,
    steam::steam_inventory_response::SteamInventoryResponse,
};

pub struct InventoryItemService;

impl InventoryItemService {
    async fn fetch_steam_inventory(
        steam_id: &str,
        app_id: i32,
    ) -> Result<SteamInventoryResponse, CreateInventoryItemError> {
        let context_id = if app_id == 753 { 6 } else { 2 };

        let steam_inventory_url = format!(
            "https://steamcommunity.com/inventory/{}/{}/{}?l=english&count=2000&preserve_bbcode=1&raw_asset_properties=1",
            steam_id, app_id, context_id
        );

        let response = reqwest::get(&steam_inventory_url).await.map_err(|e| {
            CreateInventoryItemError::SteamApiError(format!("Failed to fetch inventory: {:?}", e))
        })?;

        if !response.status().is_success() {
            return Err(CreateInventoryItemError::SteamApiError(format!(
                "Steam API returned status: {}",
                response.status()
            )));
        }

        let inventory_data = response
            .json::<SteamInventoryResponse>()
            .await
            .map_err(|e| {
                CreateInventoryItemError::SteamApiError(format!(
                    "Failed to parse inventory: {:?}",
                    e
                ))
            })?;

        if inventory_data.success != 1 {
            return Err(CreateInventoryItemError::InventoryNotFound);
        }

        Ok(inventory_data)
    }

    pub async fn fetch_and_save_inventory(
        pool: &PgPool,
        steam_id: &str,
        app_id: i32,
    ) -> Result<Vec<ItemCreationResponse>, CreateInventoryItemError> {
        let inventory_id = InventoryRepository::get_inventory_id_by_steam_id(pool, steam_id)
            .await?
            .ok_or(CreateInventoryItemError::InventoryNotFound)?;

        let inventory_data = Self::fetch_steam_inventory(steam_id, app_id).await?;

        let mut saved_items = Vec::new();

        let descriptions = inventory_data.descriptions.unwrap_or_default();

        for desc in descriptions {
            let exists =
                InventoryItemsRepository::check_item_exists(pool, inventory_id, &desc.classid)
                    .await?;

            if exists {
                continue;
            }

            let description_value = desc
                .descriptions
                .iter()
                .find(|d| d.detail_type == "html" && d.name == "description")
                .map(|d| d.value.clone())
                .unwrap_or_default();

            let create_schema = CreateItemSchema {
                inventory_id,
                app_id: desc.appid.to_string(),
                classid: desc.classid.clone(),
                icon: desc.icon_url.clone(),
                name: desc.name.clone(),
                color: desc.name_color.clone(),
                item_type: desc.item_type.clone(),
                description: description_value.clone(),
            };

            let item = InventoryItemsRepository::create_items(pool, create_schema).await?;

            saved_items.push(ItemCreationResponse {
                app_id: item.app_id,
                icon: item.icon,
                name: item.name,
                color: item.color,
                item_type: item.item_type,
                description: item.description,
            });
        }

        Ok(saved_items)
    }
}
