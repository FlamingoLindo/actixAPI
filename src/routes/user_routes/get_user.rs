use crate::AppState;
use crate::services::errors::users::get_user::GetUserError;
use crate::services::user_service::UserService;

use actix_web::{HttpResponse, Responder, get, web};
use serde_json::json;

#[get("/user/{id}")]
async fn get_user(steam_id: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    match UserService::get_user(&data.db, &steam_id).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => match e {
            GetUserError::UserNotFound => HttpResponse::NotFound().json(json!({
                "status": "error",
                "message": "User not found"
            })),
            GetUserError::DatabaseError(msg) => HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": format!("Database error: {}", msg)
            })),
        },
    }
}
