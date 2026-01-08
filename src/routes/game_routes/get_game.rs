use crate::{
    services::{errors::games::get_errors::GetGameError, game_service::GameService},
    AppState,
};
use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

#[get("/{appid}")]
pub async fn get_game(appid: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let appid = appid.into_inner();

    match GameService::get_game_by_appid(&data.db, &appid).await {
        Ok(game) => HttpResponse::Ok().json(json!({
            "status": "success",
            "data": { "game": game }
        })),
        Err(e) => match e {
            GetGameError::GameNotFound => HttpResponse::NotFound().json(json!({
                "status": "error",
                "message": "Game not found"
            })),
            GetGameError::DatabaseError(err) => {
                HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": format!("Database error: {:?}", err)
                }))
            }
        },
    }
}
