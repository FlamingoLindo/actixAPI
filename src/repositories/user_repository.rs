use crate::models::{
    dto::{CreateUserSchema, GetUser, update_user::UpdateUser},
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
        (steam_id, username, pf_url, avatar, persona_state, visibility, steam_created_at, current_game, gameid, country) 
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) 
        returning *",
        body.steam_id,
        body.personaname,
        body.profileurl,
        body.avatar,
        body.personastate,
        body.communityvisibilitystate,
        body.formatted_steam_created_at,
        body.gameextrainfo,
        body.gameid,
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
            "SELECT steam_id, username, avatar, pf_url, country, current_game, persona_state, visibility, steam_created_at, gameid FROM users WHERE steam_id = $1",
            steam_id
        ).fetch_one(pool).await?;

        Ok(fetched_user)
    }

    pub async fn get_user_id_by_steam_id(pool: &PgPool, steam_id: &str) -> Result<Uuid, SqlxError> {
        let user_id = sqlx::query_scalar!("SELECT id FROM users WHERE steam_id = $1", steam_id)
            .fetch_one(pool)
            .await?;

        Ok(user_id)
    }

    pub async fn update_user(
        pool: &PgPool,
        body: UpdateUser,
        steam_id: &str,
    ) -> Result<UpdateUser, SqlxError> {
        let fetched_user = sqlx::query_as!(
            UpdateUser,
            "UPDATE users SET 
                username = COALESCE($1, username), 
                pf_url = COALESCE($2, pf_url), 
                avatar = COALESCE($3, avatar), 
                persona_state = COALESCE($4, persona_state), 
                visibility = COALESCE($5, visibility), 
                current_game = $6, 
                country = COALESCE($7, country), 
                gameid = $8, 
                updated_at = NOW() 
            WHERE steam_id = $9
            RETURNING username, pf_url, avatar, persona_state, visibility, current_game, country, gameid",
            body.username,
            body.pf_url,
            body.avatar,
            body.persona_state,
            body.visibility,
            body.current_game,
            body.country,
            body.gameid,
            steam_id
        ).fetch_one(pool).await?;

        Ok(fetched_user)
    }
}
