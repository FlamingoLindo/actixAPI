use crate::AppState;
use crate::models::dto::{
    CreateUserSchema, GetUser, GetUserResponse, GetUsers, GetUsersResponse, ResponseStatus,
    UpdateUserSchema, UserCreationResponse, UserUpdateResponse,
};
use crate::models::user::UserModel;
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

    let exiting_query = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE steam_id = $1",
        body.steam_id
    )
    .fetch_one(&data.db)
    .await;

    if exiting_query.is_ok() {
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
                avatar: user.avatar
            };
            return HttpResponse::Ok().json(response);
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
