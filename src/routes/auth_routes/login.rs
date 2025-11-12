use crate::middleware::auth::create_jwt;
use crate::{AppState, models::admin::admin::AdminModel};
use actix_web::{HttpResponse, Responder, post, web};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct LoginSchema {
    pub username: String,
    pub password: String,
    pub refresh: Option<bool>,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub status: String,
    pub access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshResponse {
    pub status: String,
    pub access_token: String,
}

#[post("/login")]
pub async fn login(body: web::Json<LoginSchema>, data: web::Data<AppState>) -> impl Responder {
    if let Some(refresh_token) = &body.refresh_token {
        return handle_refresh(refresh_token);
    }

    let want_refresh = body.refresh.unwrap_or(false);

    handle_login(&body.username, &body.password, data, want_refresh).await
}

async fn handle_login(
    username: &str,
    password: &str,
    data: web::Data<AppState>,
    want_refresh: bool,
) -> HttpResponse {
    let query_result = sqlx::query_as!(
        AdminModel,
        "SELECT * FROM admins WHERE username = $1",
        username
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(user) => {
            let parsed_hash = match PasswordHash::new(&user.password) {
                Ok(hash) => hash,
                Err(_) => {
                    return HttpResponse::InternalServerError().json(json!({
                        "status": "error",
                        "message": "Invalid password hash format"
                    }));
                }
            };

            if Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_err()
            {
                return HttpResponse::Unauthorized().json(json!({
                    "status": "error",
                    "message": "Invalid credentials"
                }));
            }

            let access_token = match create_jwt(&user.username, false) {
                Ok(token) => token,
                Err(_) => {
                    return HttpResponse::InternalServerError().json(json!({
                        "status": "error",
                        "message": "Failed to generate access token"
                    }));
                }
            };

            let refresh_token = if want_refresh {
                match create_jwt(&user.username, true) {
                    Ok(token) => Some(token),
                    Err(_) => {
                        return HttpResponse::InternalServerError().json(json!({
                            "status": "error",
                            "message": "Failed to generate refresh token"
                        }));
                    }
                }
            } else {
                None
            };

            let response = LoginResponse {
                status: "success".to_string(),
                access_token,
                refresh_token,
                user: UserInfo {
                    username: user.username,
                },
            };

            HttpResponse::Ok().json(response)
        }
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "User not found. Please register first."
        })),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Database error: {:?}", e)
        })),
    }
}

fn handle_refresh(refresh_token: &str) -> HttpResponse {
    let jwt_secret = std::env::var("JWT_SECRET").expect("No jwt secret in the .env!");

    match decode::<crate::middleware::auth::Claims>(
        refresh_token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token_data) => {
            if token_data.claims.token_type != "refresh" {
                return HttpResponse::Unauthorized().json(json!({
                    "status": "error",
                    "message": "Invalid token type. Must use refresh token."
                }));
            }

            match create_jwt(&token_data.claims.sub, false) {
                Ok(access_token) => {
                    let response = RefreshResponse {
                        status: "success".to_string(),
                        access_token,
                    };
                    HttpResponse::Ok().json(response)
                }
                Err(_) => HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": "Failed to generate new access token"
                })),
            }
        }
        Err(_) => HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Invalid or expired refresh token"
        })),
    }
}
