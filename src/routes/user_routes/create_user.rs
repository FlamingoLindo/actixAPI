use crate::{
    AppState,
    models::dto::create_user::CreateUserRequest,
    services::user_service::{UserService, CreateUserError},
};
use actix_web::{HttpResponse, Responder, post, web};
use serde_json::json;

#[post("")]
pub async fn create_user(
    body: web::Json<CreateUserRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let request = body.into_inner();
    match UserService::create_user(&data.db, request.steam_id).await {
        Ok(user_response) => HttpResponse::Ok().json(json!({
            "status": "success",
            "data": { "user": user_response }
        })),
        Err(e) => match e {
            CreateUserError::UserAlreadyExists => HttpResponse::Conflict().json(json!({
                "status": "error",
                "message": "This user is already registered in the database!"
            })),
            CreateUserError::SteamUserNotFound => HttpResponse::NotFound().json(json!({
                "status": "error",
                "message": "Steam user not found"
            })),
            CreateUserError::SteamApiError(msg) => {
                HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": format!("Steam API error: {}", msg)
                }))
            }
            CreateUserError::DatabaseError(err) => {
                HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": format!("Database error: {:?}", err)
                }))
            }
        },
    }
}
