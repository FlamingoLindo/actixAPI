use crate::AppState;
use crate::services::errors::users::update_erros::UpdateUserError;
use crate::services::user_service::UserService;

use actix_web::{HttpResponse, Responder, patch, web};
use serde_json::json;

#[patch("/user/{id}")]
async fn update_user(steam_id: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    match UserService::update_user(&data.db, &steam_id).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => match e {
            UpdateUserError::UserNotFound => HttpResponse::NotFound().json(json!({
                "status": "error",
                "message": "User not found"
            })),
            UpdateUserError::SteamUserNotFound => HttpResponse::NotFound().json(json!({
                "status": "error",
                "message": "Steam user not found"
            })),
            UpdateUserError::SteamApiError(msg) => HttpResponse::BadGateway().json(json!({
                "status": "error",
                "message": format!("Steam API error: {}", msg)
            })),
            UpdateUserError::DatabaseError(msg) => {
                HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": format!("Database error {}", msg)
                }))
            }
        },
    }
}
