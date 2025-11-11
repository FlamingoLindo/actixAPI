use crate::models::admin::admin::AdminModel;
use crate::models::role::role::RoleModel;
use crate::{AppState, models::admin::dto::create_admin::CreateAdminSchema};
use actix_web::{HttpResponse, Responder, post, web};
use argon2::Argon2;
use argon2::password_hash::{PasswordHasher, SaltString, rand_core::OsRng};
use serde_json::json;

#[post("/")]
pub async fn create_admin(
    body: web::Json<CreateAdminSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let exiting_query = sqlx::query_as!(
        AdminModel,
        "SELECT * FROM admins WHERE username = $1",
        body.username
    )
    .fetch_one(&data.db)
    .await;

    if exiting_query.is_ok() {
        return HttpResponse::Conflict()
            .json(serde_json::json!({"status": "error", "message": "Invalid data!"}));
    }

    let role_query = sqlx::query_as!(
        RoleModel,
        "SELECT * FROM roles WHERE name = $1",
        body.role_name
    )
    .fetch_one(&data.db)
    .await;

    if role_query.is_err() {
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "error", "message": "Invalid role!"}));
    }

    let role = role_query.unwrap();

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = match argon2.hash_password(body.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(e) => {
            return HttpResponse::InternalServerError().json(
                json!({"status": "error", "message": format!("Password hashing failed: {:?}", e)}),
            );
        }
    };

    let query_result = sqlx::query_as!(
        AdminModel,
        "INSERT INTO admins
        (username, password, role_id) 
        values ($1, $2, $3) 
        returning *",
        body.username,
        password_hash,
        role.id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(_) => {
            let response = serde_json::json!({"status": "success", "message": "Admin created!"});
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": format!("There has been an error during admin creation: {:?}", e)}));
        }
    }
}
