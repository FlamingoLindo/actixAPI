use crate::{
    services::errors::inventory_items::create_erros::CreateInventoryItemError,
    services::inventory_items_service::InventoryItemService,
    AppState,
};
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct FetchInventoryRequest {
    pub steam_id: String,
    pub app_id: i32,
}

#[post("/")]
pub async fn fetch_inventory(
    body: web::Json<FetchInventoryRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let request = body.into_inner();
    
    match InventoryItemService::fetch_and_save_inventory(&data.db, &request.steam_id, request.app_id).await {
        Ok(items) => HttpResponse::Ok().json(json!({
            "status": "success",
            "data": { "items": items }
        })),
        Err(e) => match e {
            CreateInventoryItemError::InventoryNotFound => {
                HttpResponse::NotFound().json(json!({
                    "status": "error",
                    "message": "Steam inventory not found or is private"
                }))
            }
            CreateInventoryItemError::SteamApiError(msg) => {
                HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": format!("Steam API error: {}", msg)
                }))
            }
            CreateInventoryItemError::DatabaseError(err) => {
                HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": format!("Database error: {:?}", err)
                }))
            }
            CreateInventoryItemError::InventoryItemAlreadyExists => {
                HttpResponse::Conflict().json(json!({
                    "status": "error",
                    "message": "Inventory item already exists"
                }))
            }
        },
    }
}
