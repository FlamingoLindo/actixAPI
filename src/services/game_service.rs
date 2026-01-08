use sqlx::PgPool;

use crate::{
    models::game::{dto::CreateGameSchema, game::GameCreationResponse},
    repositories::game_repository::GameRepository,
    services::errors::games::{create_errors::CreateGameError, get_errors::GetGameError},
    steam::steam_api_response::SteamGameResponse,
};

pub struct GameService;

impl GameService {
    pub async fn fetch_steam_game_data(appid: &str) -> Result<SteamGameResponse, CreateGameError> {
        let steam_api = format!(
            "https://store.steampowered.com/api/appdetails?appids={}",
            appid
        );

        let response = reqwest::get(&steam_api)
            .await
            .map_err(|e| CreateGameError::SteamApiError(format!("Failed to fetch: {:?}", e)))?;

        response
            .json()
            .await
            .map_err(|e| CreateGameError::SteamApiError(format!("Failed to parse: {:?}", e)))
    }

    pub async fn create_game(
        pool: &PgPool,
        appid: String,
    ) -> Result<GameCreationResponse, CreateGameError> {
        let existing_game = GameRepository::check_if_game_exists(pool, &appid).await?;

        let game_id = if existing_game {
            // If game exists, get its ID instead of creating
            let existing = GameRepository::get_game_by_appid(pool, &appid).await?;
            existing.id
        } else {
            // Create new game
            let steam_game_data = Self::fetch_steam_game_data(&appid).await?;
            let game_wrapper = steam_game_data
                .games
                .get(&appid)
                .ok_or(CreateGameError::SteamGameNotFound)?;
            if !game_wrapper.success {
                return Err(CreateGameError::SteamGameNotFound);
            }
            let game = game_wrapper
                .data
                .as_ref()
                .ok_or(CreateGameError::SteamGameNotFound)?;
            let screenshots = game
                .screenshots
                .as_ref()
                .map(|s| {
                    s.iter()
                        .map(|screenshot| screenshot.path_full.clone())
                        .collect()
                })
                .unwrap_or_default();
            let create_schema = CreateGameSchema {
                appid: appid.clone(),
                name: game.name.clone(),
                short_description: game.short_description.clone(),
                header_image: game.header_image.clone(),
                screenshots,
            };
            let db_game = GameRepository::create_game(pool, create_schema).await?;
            db_game.id
        };

        // Fetch the game details to return
        let game = GameRepository::get_game_by_id(pool, &game_id).await?;

        Ok(GameCreationResponse {
            appid: game.appid,
            name: game.name,
            short_description: game.short_description,
            header_image: game.header_image,
            screenshots: game.screenshots,
        })
    }

    pub async fn get_game_by_appid(
        pool: &PgPool,
        appid: &str,
    ) -> Result<GameCreationResponse, GetGameError> {
        let game = GameRepository::get_game_by_appid(pool, appid).await?;

        Ok(GameCreationResponse {
            appid: game.appid,
            name: game.name,
            short_description: game.short_description,
            header_image: game.header_image,
            screenshots: game.screenshots,
        })
    }
}
