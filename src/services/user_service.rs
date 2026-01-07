use crate::models::ResponseStatus;
use crate::models::dto::{
    CreateUserSchema, GetUserResponse, UpdateUser, UpdateUserResponse, UserCreationResponse,
};
use crate::models::user::dto::get_users::{GetUsersResponse, PaginationMeta};
use crate::repositories::user_repository::UserRepository;
use crate::services::errors::users::create_errors::CreateUserError;
use crate::services::errors::users::delete_erros::DeleteUserError;
use crate::services::errors::users::get_user::GetUserError;
use crate::services::errors::users::update_erros::UpdateUserError;
use crate::services::game_service::GameService;
use crate::steam::steam_api_response::SteamResponse;
use chrono::DateTime;
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserService;

impl UserService {
    pub async fn get_users(
        pool: &PgPool,
        username: Option<&str>,
        page: i64,
        limit: i64,
    ) -> Result<GetUsersResponse, sqlx::Error> {
        let offset = (page - 1) * limit;

        let (users, total) =
            UserRepository::get_users_with_count(pool, username, limit, offset).await?;

        let total_pages = if total == 0 {
            1
        } else {
            (total + limit - 1) / limit
        };

        let total_in_page = users.len();

        let response = GetUsersResponse {
            status: ResponseStatus::Success,
            users,
            pagination: PaginationMeta {
                total_in_page,
                total,
                total_pages,
                current_page: page,
                page_size: limit,
            },
        };

        Ok(response)
    }

    async fn fetch_steam_data(steam_id: &str) -> Result<SteamResponse, CreateUserError> {
        dotenv::dotenv().ok();
        let key = std::env::var("STEAM_KEY")
            .map_err(|_| CreateUserError::SteamApiError("Steam API Key not found".to_string()))?;

        let steam_api = format!(
            "http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key={}&steamids={}&format=json",
            key, steam_id
        );

        let response = reqwest::get(&steam_api)
            .await
            .map_err(|e| CreateUserError::SteamApiError(format!("Failed to fetch: {:?}", e)))?;

        response
            .json()
            .await
            .map_err(|e| CreateUserError::SteamApiError(format!("Failed to parse: {:?}", e)))
    }

    pub async fn create_user(
        pool: &PgPool,
        steam_id: String,
    ) -> Result<UserCreationResponse, CreateUserError> {
        let existing_user = UserRepository::check_if_user_exits(pool, &steam_id).await?;
        if existing_user {
            return Err(CreateUserError::UserAlreadyExists);
        }

        let steam_data = Self::fetch_steam_data(&steam_id).await?;

        let players = steam_data.response.players;
        let user = players
            .into_iter()
            .next()
            .ok_or(CreateUserError::SteamUserNotFound)?;

        let timestamp = user.timecreated.unwrap_or(0);
        let formatted_steam_created_at = DateTime::from_timestamp(timestamp, 0);

        let gameid = user.gameid.clone();

        let create_schema = CreateUserSchema {
            steam_id,
            personaname: user.personaname,
            profileurl: user.profileurl,
            avatar: user.avatar,
            personastate: user.personastate,
            communityvisibilitystate: user.communityvisibilitystate,
            formatted_steam_created_at,
            gameextrainfo: user.gameextrainfo,
            loccountrycode: user.loccountrycode,
            gameid: user.gameid,
        };

        let db_user = UserRepository::create_user(pool, create_schema).await?;

        if let Some(game_id) = gameid {
            let _ = GameService::create_game(pool, game_id).await;
        }

        Ok(UserCreationResponse {
            username: db_user.username,
            pf_url: db_user.pf_url,
            avatar: db_user.avatar,
        })
    }

    pub async fn delete_user(pool: &PgPool, user_id: Uuid) -> Result<u64, DeleteUserError> {
        let rows = UserRepository::delete_user(pool, user_id).await?;
        if rows == 0 {
            return Err(DeleteUserError::UserNotFound);
        }

        Ok(rows)
    }

    pub async fn get_user(pool: &PgPool, steam_id: &str) -> Result<GetUserResponse, GetUserError> {
        let existing_user = UserRepository::check_if_user_exits(pool, steam_id).await?;
        if !existing_user {
            return Err(GetUserError::UserNotFound);
        }

        let user = UserRepository::get_user(pool, steam_id).await?;

        let response = GetUserResponse {
            status: ResponseStatus::Success,
            user,
        };

        Ok(response)
    }

    pub async fn update_user(
        pool: &PgPool,
        steam_id: &str,
    ) -> Result<UpdateUserResponse, UpdateUserError> {
        let existing_user = UserRepository::check_if_user_exits(pool, steam_id).await?;
        if !existing_user {
            return Err(UpdateUserError::UserNotFound);
        }

        let steam_data = Self::fetch_steam_data(&steam_id)
            .await
            .map_err(|e| match e {
                CreateUserError::SteamApiError(msg) => UpdateUserError::SteamApiError(msg),
                CreateUserError::SteamUserNotFound => UpdateUserError::SteamUserNotFound,
                CreateUserError::DatabaseError(err) => UpdateUserError::DatabaseError(err),
                CreateUserError::UserAlreadyExists => {
                    UpdateUserError::DatabaseError(sqlx::Error::RowNotFound)
                }
            })?;

        let players = steam_data.response.players;
        let steam_user = players
            .into_iter()
            .next()
            .ok_or(UpdateUserError::SteamUserNotFound)?;

        let update_body = UpdateUser {
            username: Some(steam_user.personaname),
            pf_url: Some(steam_user.profileurl),
            avatar: Some(steam_user.avatar),
            persona_state: Some(steam_user.personastate),
            visibility: Some(steam_user.communityvisibilitystate),
            current_game: steam_user.gameextrainfo,
            country: steam_user.loccountrycode,
        };

        UserRepository::update_user(pool, update_body, steam_id).await?;

        Ok(UpdateUserResponse {
            message: ResponseStatus::Success,
        })
    }
}
