use sqlx::{Error as SqlxError, PgPool};

use crate::models::game::{dto::CreateGameSchema, game::GameModel};

pub struct GameRepository;

impl GameRepository {
    pub async fn check_if_game_exists(pool: &PgPool, appid: &str) -> Result<bool, SqlxError> {
        let existing_query =
            sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM games WHERE appid = $1)", appid)
                .fetch_one(pool)
                .await?;

        Ok(existing_query.unwrap_or(false))
    }

    pub async fn create_game(
        pool: &PgPool,
        body: CreateGameSchema,
    ) -> Result<GameModel, SqlxError> {
        sqlx::query_as!(
        GameModel,
        "INSERT into games (appid,name,short_description,header_image,screenshots) values ($1,$2,$3,$4,$5) returning *",
        body.appid,
        body.name,
        body.short_description,
        body.header_image,
        &body.screenshots
    ).fetch_one(pool).await
    }

    pub async fn get_game_by_appid(pool: &PgPool, appid: &str) -> Result<GameModel, SqlxError> {
        sqlx::query_as!(GameModel, "SELECT * FROM games WHERE appid = $1", appid)
            .fetch_one(pool)
            .await
    }

    pub async fn get_game_by_id(pool: &PgPool, id: &uuid::Uuid) -> Result<GameModel, SqlxError> {
        sqlx::query_as!(GameModel, "SELECT * FROM games WHERE id = $1", id)
            .fetch_one(pool)
            .await
    }
}
