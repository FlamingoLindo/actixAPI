use crate::AppState;
use crate::models::dto::{CreateUserSchema, UserCreationResponse};
use crate::models::user::{UpdateUserSchema, UserModel};
use crate::steam::steam_api_response::SteamResponse;

use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use chrono::{DateTime, Utc};
use serde_json::json;
use uuid::Uuid;

#[post("/")]
pub async fn create_user(
    body: web::Json<CreateUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    dotenv::dotenv().ok();

    let exiting_quey = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE steam_id = $1",
        body.steam_id
    )
    .fetch_one(&data.db)
    .await;

    if exiting_quey.is_ok() {
        return HttpResponse::Conflict()
            .json(serde_json::json!({"status": "error", "message": "This user is already registered in the database!"}));
    }

    let key: String = std::env::var("STEAM_KEY").expect("Steam API Key not found!");
    let steam_api = format!(
        "http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key={}&steamids={}&format=json",
        key, body.steam_id
    );

    let response = match reqwest::get(&steam_api).await {
        Ok(resp) => resp,
        Err(e) => return HttpResponse::InternalServerError().json(
            json!({"status": "error", "message": format!("Failed to fetch user data from Steam: {:?}", e)}),
        ),
    };

    let steam_data: SteamResponse = match response.json().await {
        Ok(data) => data,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": format!("Failed to parse Steam data: {:?}", e)}));
        }
    };

    let user = match steam_data.response.players.first() {
        Some(p) => p,
        None => {
            return HttpResponse::NotFound()
                .json(json!({"status": "error", "message": "Steam user not found"}));
        }
    };

    let timestamp = user.timecreated.unwrap_or(0);
    let formatted_steam_created_at = DateTime::from_timestamp(timestamp, 0);

    let query_result = sqlx::query_as!(
        UserModel,
        "INSERT into users 
        (steam_id, username, pf_url, avatar, persona_state, visibility, steam_created_at, current_game, country) 
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9) 
        returning *",
        body.steam_id,
        user.personaname,
        user.profileurl,
        user.avatar,
        user.personastate,
        user.communityvisibilitystate,
        formatted_steam_created_at,
        user.gameextrainfo,
        user.loccountrycode
    ).fetch_one(&data.db)
    .await;

    match query_result {
        Ok(db_user) => {
            let response_dto = UserCreationResponse {
                username: db_user.username,
                pf_url: db_user.pf_url,
                avatar: db_user.avatar,
            };

            let user_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "user": response_dto
            })});
            return HttpResponse::Ok().json(user_response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": format!("There has been an error during user creation: {:?}", e)}));
        }
    }
}

#[get("/")]
pub async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(UserModel, "SELECT * FROM users")
        .fetch_all(&data.db)
        .await;

    if query_result.is_err() {
        let message = "There has been an error when trying to fetch all users, please try again!";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": message}));
    }

    let users = query_result.unwrap();

    let json_response =
        serde_json::json!({"status": "success", "count": users.len(), "users": users});

    HttpResponse::Ok().json(json_response)
}

#[get("/user/{id}")]
async fn get_user(path: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();
    let query_result = sqlx::query_as!(UserModel, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(user) => {
            let user_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "user": user
            })});
            return HttpResponse::Ok().json(user_response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": format!("There has been an error when fetching the user data: {:?}", e)}));
        }
    }
}

#[put("/user/{id}")]
async fn update_user(
    path: web::Path<Uuid>,
    data: web::Data<AppState>,
    body: web::Json<UpdateUserSchema>,
) -> impl Responder {
    let user_id = path.into_inner();
    let query_result = sqlx::query_as!(UserModel, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let message = format!("User with ID: {} not found!", user_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let now = Utc::now();
    let user = query_result.unwrap();

    let query_result = sqlx::query_as!(
        UserModel,
        "UPDATE users SET username = $1, updated_at = $2 WHERE id = $3 RETURNING *",
        body.username.to_owned().unwrap_or(user.username),
        now,
        user_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(user) => {
            let user_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "user": user
            })});
            return HttpResponse::Ok().json(user_response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": format!("There has been an error when updating the user data: {:?}", e)}));
        }
    }
}

#[delete("/user/{id}")]
async fn delete_user(path: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();
    let rows_affected = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let message = format!("User with ID: {} not found!", user_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    HttpResponse::NoContent().finish()
}
