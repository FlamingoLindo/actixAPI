use actix_web::{Error, HttpMessage, dev::ServiceRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub token_type: String,
}

pub fn create_jwt(steam_id: &str, refresh: bool) -> Result<String, jsonwebtoken::errors::Error> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("No jwt secret in the .env!");

    let now = Utc::now();

    let expires_at = if refresh {
        now + Duration::days(7)
    } else {
        now + Duration::days(1)
    };

    let claims = Claims {
        sub: steam_id.to_owned(),
        exp: expires_at.timestamp() as usize,
        iat: now.timestamp() as usize,
        token_type: if refresh {
            "refresh".to_string()
        } else {
            "access".to_string()
        },
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("No jwt secret in the .env!");

    let token = credentials.token();

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token_data) => {
            if token_data.claims.token_type != "access" {
                eprintln!(
                    "Wrong token type: expected 'access', got '{}'",
                    token_data.claims.token_type
                );
                let error = actix_web::error::ErrorUnauthorized(
                    "Invalid token type. Use access token for API requests.",
                );
                return Err((error, req));
            }

            req.extensions_mut().insert(token_data.claims);
            Ok(req)
        }
        Err(e) => {
            eprintln!("JWT validation failed: {:?}", e);
            let error = actix_web::error::ErrorUnauthorized("Invalid or expired token");
            Err((error, req))
        }
    }
}
