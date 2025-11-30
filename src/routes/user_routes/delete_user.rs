use crate::{
    AppState,
    services::{errors::users::delete_erros::DeleteUserError, user_service::UserService},
};

use actix_web::{HttpResponse, Responder, delete, web};
use serde_json::json;
use uuid::Uuid;

#[delete("/user/{id}")]
async fn delete_user(user_id: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
    match UserService::delete_user(&data.db, user_id.into_inner()).await {
        Ok(rows) => {
            if rows > 0 {
                HttpResponse::NoContent().finish()
            } else {
                HttpResponse::NotFound().json(json!({
                    "status": "error",
                    "message": "User not found"
                }))
            }
        }
        Err(e) => match e {
            DeleteUserError::UserNotFound => HttpResponse::NotFound().json(json!({
                "status": "error",
                "message": "User not found"
            })),
            DeleteUserError::DatabaseError(msg) => {
                HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": format!("Database error: {}", msg)
                }))
            }
        },
    }
}
