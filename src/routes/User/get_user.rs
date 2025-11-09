use crate::AppState;
use crate::models::ResponseStatus;
use crate::models::User::user::UserModel;
use crate::models::dto::{
    CreateUserSchema, GetUser, GetUserResponse, GetUsers, GetUsersResponse, UpdateUserSchema,
    UserCreationResponse, UserUpdateResponse,
};
use crate::steam::steam_api_response::SteamResponse;

use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use chrono::{DateTime, Utc};
use serde_json::json;
use uuid::Uuid;

#[get("/user/{id}")]
async fn get_user(steam_id: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(
        GetUser,
        "SELECT steam_id, username, avatar, pf_url, current_game, country, persona_state, steam_created_at, visibility FROM users WHERE steam_id = $1",
        steam_id.as_ref()
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(user) => {
            let response = GetUserResponse {
                status: ResponseStatus::Success,
                user: user,
            };
            return HttpResponse::Ok().json(response);
        }
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "User not found"
        })),
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": format!("There has been an error when fetching the user data: {:?}", e)}));
        }
    }
}
