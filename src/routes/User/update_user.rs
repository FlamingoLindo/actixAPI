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

#[put("/user/{id}")]
async fn update_user(
    steam_id: web::Path<String>,
    data: web::Data<AppState>,
    body: web::Json<UpdateUserSchema>,
) -> impl Responder {
    let steam_id = steam_id.into_inner();

    let exiting_query = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE steam_id = $1",
        steam_id
    )
    .fetch_one(&data.db)
    .await;

    if exiting_query.is_err() {
        let message = format!("User with ID: {} not found!", steam_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let now = Utc::now();
    let user = exiting_query.unwrap();

    let query_result = sqlx::query_as!(
        UserModel,
        "UPDATE users SET username = $1, pf_url = $2, avatar = $3, persona_state = $4, visibility = $5, current_game = $6, country = $7, updated_at = $8 WHERE steam_id = $9 RETURNING *",
        body.username.as_ref().unwrap_or(&user.username),
        body.pf_url.as_ref().unwrap_or(&user.pf_url),
        body.avatar.as_ref().unwrap_or(&user.avatar),
        body.persona_state.unwrap_or(user.persona_state),
        body.visibility.unwrap_or(user.visibility),
        body.current_game.as_ref().or(user.current_game.as_ref()),
        body.country.as_ref().or(user.country.as_ref()),
        now,
        steam_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(user) => {
            let response = UserUpdateResponse {
                username: user.username,
                pf_url: user.pf_url,
                avatar: user.avatar,
            };
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": format!("There has been an error when updating the user data: {:?}", e)}));
        }
    }
}
