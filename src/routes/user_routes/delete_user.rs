use crate::AppState;

use actix_web::{HttpResponse, Responder, delete, web};
use uuid::Uuid;

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