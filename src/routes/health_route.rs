use actix_web::{HttpResponse, Responder, get};
use serde_json::json;

#[get("/api/healthchecker")]
pub async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Up and running!";
    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}
