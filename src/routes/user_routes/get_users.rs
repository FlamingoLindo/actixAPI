use crate::AppState;
use crate::models::ResponseStatus;
use crate::models::dto::{
    GetUsers, GetUsersResponse,
};

use actix_web::{HttpResponse, Responder, get, web};
use serde_json::json;

#[get("")]
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
