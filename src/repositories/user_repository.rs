use crate::models::{
    dto::{CreateUserSchema, GetUser},
    user::{dto::get_users::GetUsers, user::UserModel},
};
use sqlx::{Error as SqlxError, PgPool};
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
    pub async fn get_users_paginated(
        pool: &PgPool,
        username_filter: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<GetUsers>, SqlxError> {
        match username_filter {
            Some(username) => {
                let search_pattern = format!("%{}%", username);
                sqlx::query_as!(
                    GetUsers,
                    "SELECT steam_id, username, avatar, pf_url, current_game 
                     FROM users 
                     WHERE username ILIKE $1
                     ORDER BY username
                     LIMIT $2 OFFSET $3",
                    search_pattern,
                    limit,
                    offset
                )
                .fetch_all(pool)
                .await
            }
            None => {
                sqlx::query_as!(
                    GetUsers,
                    "SELECT steam_id, username, avatar, pf_url, current_game 
                     FROM users 
                     ORDER BY username
                     LIMIT $1 OFFSET $2",
                    limit,
                    offset
                )
                .fetch_all(pool)
                .await
            }
        }
    }

    pub async fn count_users(
        pool: &PgPool,
        username_filter: Option<&str>,
    ) -> Result<i64, SqlxError> {
        let count = match username_filter {
            Some(username) => {
                let search_pattern = format!("%{}%", username);
                sqlx::query_scalar!(
                    "SELECT COUNT(*) FROM users WHERE username ILIKE $1",
                    search_pattern
                )
                .fetch_one(pool)
                .await?
            }
            None => {
                sqlx::query_scalar!("SELECT COUNT(*) FROM users")
                    .fetch_one(pool)
                    .await?
            }
        };

        Ok(count.unwrap_or(0))
    }

    pub async fn get_users_with_count(
        pool: &PgPool,
        username_filter: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<GetUsers>, i64), SqlxError> {
        let users = Self::get_users_paginated(pool, username_filter, limit, offset).await?;
        let total = Self::count_users(pool, username_filter).await?;
        Ok((users, total))
    }

    pub async fn check_if_user_exits(pool: &PgPool, steam_id: &str) -> Result<bool, SqlxError> {
        let existing_query = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE steam_id = $1)",
            steam_id
        )
        .fetch_one(pool)
        .await?;

        Ok(existing_query.unwrap_or(false))
    }

    pub async fn create_user(
        pool: &PgPool,
        body: CreateUserSchema,
    ) -> Result<UserModel, SqlxError> {
        sqlx::query_as!(
            UserModel,
        "INSERT into users 
        (steam_id, username, pf_url, avatar, persona_state, visibility, steam_created_at, current_game, country) 
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9) 
        returning *",
        body.steam_id,
        body.personaname,
        body.profileurl,
        body.avatar,
        body.personastate,
        body.communityvisibilitystate,
        body.formatted_steam_created_at,
        body.gameextrainfo,
        body.loccountrycode
        ).fetch_one(pool).await
    }

    pub async fn delete_user(pool: &PgPool, user_id: Uuid) -> Result<u64, SqlxError> {
        let rows = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(pool)
            .await?
            .rows_affected();

        Ok(rows)
    }

    pub async fn get_user(pool: &PgPool, steam_id: &str) -> Result<GetUser, SqlxError> {
        let fetched_user = sqlx::query_as!(
            GetUser,
            "SELECT steam_id, username,avatar,pf_url,country,current_game,persona_state,visibility,steam_created_at FROM users WHERE steam_id = $1",
            steam_id
        ).fetch_one(pool).await?;

        Ok(fetched_user)
    }
}
