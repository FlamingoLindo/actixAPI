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

#[get("/")]
pub async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(
        GetUsers,
        "SELECT steam_id, username, avatar, pf_url, current_game FROM users"
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message = "There has been an error when trying to fetch all users, please try again!";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": message}));
    }

    let users = query_result.unwrap();

    let response_dto = GetUsersResponse {
        status: ResponseStatus::Success,
        count: users.len(),
        users: users,
    };

    HttpResponse::Ok().json(response_dto)
}
