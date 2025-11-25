use crate::models::user::dto::get_users::GetUsers;
use sqlx::{Error as SqlxError, PgPool};

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
}
