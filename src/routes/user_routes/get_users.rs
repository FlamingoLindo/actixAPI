use crate::AppState;
use crate::models::user::constants::{MAX_PAGE_SIZE, MIN_PAGE_SIZE};
use crate::models::user::pagination::user_pagination::QueryParams;
use crate::services::user_service::UserService;
use actix_web::{HttpResponse, get, web};
use serde_json::json;

#[get("")]
pub async fn get_users(data: web::Data<AppState>, params: web::Query<QueryParams>) -> HttpResponse {
    let page = params.page.max(1);
    let limit = params.limit.clamp(MIN_PAGE_SIZE, MAX_PAGE_SIZE);

    let username_filter = params.username.as_deref();

    match UserService::get_users_paginated(&data.db, username_filter, page, limit).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            eprintln!("Database error fetching users: {:?}", e);

            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Failed to fetch users"
            }))
        }
    }
}
