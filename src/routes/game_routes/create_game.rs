use crate::{
    AppState,
    models::game::dto::CreateGameRequest,
    services::{errors::games::create_errors::CreateGameError, game_service::GameService},
};
use actix_web::{HttpResponse, Responder, post, web};
use serde_json::json;

#[post("")]
pub async fn create_game(
    body: web::Json<CreateGameRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let request = body.into_inner();
    match GameService::create_game(&data.db, request.appid).await {
        Ok(game_response) => HttpResponse::Ok().json(json!({
            "status": "success",
            "data": { "game": game_response }
        })),
        Err(e) => match e {
            CreateGameError::GameAlreadyExists => HttpResponse::Conflict().json(json!({
                "status": "error",
                "message": "This game is already registered in the database!"
            })),
            CreateGameError::SteamGameNotFound => HttpResponse::NotFound().json(json!({
                "status": "error",
                "message": "Steam game not found"
            })),
            CreateGameError::SteamApiError(msg) => {
                HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": format!("Steam API error: {}", msg)
                }))
            }
            CreateGameError::DatabaseError(err) => {
                HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": format!("Database error: {:?}", err)
                }))
            }
        },
    }
}
