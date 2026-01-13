# ActixAPI

A robust REST API built with Actix-web and PostgreSQL, featuring user management, game tracking, and authentication.

## Features

- RESTful API architecture with Actix-web framework
- PostgreSQL database with SQLx for type-safe queries
- JWT-based authentication and authorization
- Role-based access control (Admin/User roles)
- User management with pagination support
- Game management with Steam API integration
- Password hashing with Argon2
- CORS configuration for cross-origin requests
- Database migrations with SQLx-cli
- Comprehensive error handling

## Tech Stack

- **Framework**: Actix-web 4
- **Database**: PostgreSQL with SQLx
- **Authentication**: JWT (jsonwebtoken), Argon2 password hashing
- **External APIs**: Steam API integration
- **Serialization**: Serde
- **HTTP Client**: Reqwest

## Project Structure

```bash
src/
├── main.rs              # Application entry point
├── middleware/          # Authentication middleware
├── models/              # Data models and DTOs
│   ├── admin/          # Admin models
│   ├── game/           # Game models
│   ├── user/           # User models
│   └── role/           # Role models
├── repositories/        # Database access layer
├── routes/              # API route handlers
│   ├── admin_routes/   # Admin endpoints
│   ├── auth_routes/    # Authentication endpoints
│   ├── game_routes/    # Game endpoints
│   └── user_routes/    # User endpoints
├── services/            # Business logic layer
└── steam/              # Steam API integration
```

## Prerequisites

- Rust (2024 edition or later)
- PostgreSQL
- SQLx-cli for database migrations

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

## Installation

1. Clone the repository:

```bash
git clone <repository-url>
cd actixAPI
```

1. Set up environment variables:
Create a `.env` file in the project root:

```env
DATABASE_URL=postgres://username:password@localhost/database_name
RUST_LOG=actix_web=info
JWT_SECRET=your-secret-key-here
JWT_MAXAGE=3600
```

1. Run database migrations:

```bash
make migrate
```

1. Build the project:

```bash
make build
```

## Running the Application

Development mode with auto-reload:

```bash
make run
```

Standard run:

```bash
cargo run
```

The server will start on `http://localhost:8080` (or configured port).

## Implementation Details

### Application Setup

The main application entry point configures the database connection pool, CORS, and routes:

```rust
// src/main.rs
use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header, middleware::Logger};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("Database URL not found!");
    
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
            
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(actix_web::web::Data::new(AppState { 
                db: pool.clone() 
            }))
            .configure(config)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
```

### JWT Authentication Middleware

The authentication middleware validates JWT tokens and protects routes:

```rust
// src/middleware/auth.rs
use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation};
use chrono::{Duration, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub token_type: String,
}

pub fn create_jwt(username: &str, refresh: bool) -> Result<String, jsonwebtoken::errors::Error> {
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("No jwt secret in the .env!");

    let now = Utc::now();
    let expires_at = if refresh {
        now + Duration::days(7)
    } else {
        now + Duration::days(1)
    };

    let claims = Claims {
        sub: username.to_owned(),
        exp: expires_at.timestamp() as usize,
        iat: now.timestamp() as usize,
        token_type: if refresh { "refresh" } else { "access" }.to_string(),
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
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("No jwt secret in the .env!");
    let token = credentials.token();

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token_data) => {
            if token_data.claims.token_type != "access" {
                return Err((actix_web::error::ErrorUnauthorized(
                    "Invalid token type"
                ), req));
            }
            req.extensions_mut().insert(token_data.claims);
            Ok(req)
        }
        Err(_) => Err((actix_web::error::ErrorUnauthorized(
            "Invalid or expired token"
        ), req)),
    }
}
```

### Route Configuration

Routes are organized with scopes and middleware:

```rust
// src/routes/config.rs
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::middleware::auth::validator;

pub fn config(conf: &mut web::ServiceConfig) {
    let auth_scope = web::scope("/api/auth")
        .service(login);

    let auth_middleware = HttpAuthentication::bearer(validator);

    let users_scope = web::scope("/api/users")
        .service(create_user)
        .service(get_user)
        .service(get_users)
        .service(
            web::scope("")
                .wrap(auth_middleware.clone())
                .service(delete_user)
        );

    let games_scope = web::scope("/api/games")
        .service(create_game)
        .service(get_game);

    conf.service(auth_scope)
        .service(users_scope)
        .service(games_scope);
}
```

### Data Models

Models use SQLx's FromRow for database mapping:

```rust
// src/models/user/user.rs
use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct UserModel {
    pub id: Uuid,
    pub steam_id: String,
    pub username: String,
    pub pf_url: String,
    pub avatar: String,
    pub persona_state: i32,
    pub visibility: i32,
    
    #[serde(rename = "steamCreatedAt")]
    pub steam_created_at: chrono::DateTime<chrono::Utc>,
    pub gameid: Option<String>,
    pub current_game: Option<String>,
    pub country: Option<String>,
    
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
```

### Route Handlers

Route handlers process requests and return responses:

```rust
// src/routes/user_routes/create_user.rs
use actix_web::{post, web, HttpResponse, Responder};
use serde_json::json;

#[post("")]
pub async fn create_user(
    body: web::Json<CreateUserRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let request = body.into_inner();
    match UserService::create_user(&data.db, request.steam_id).await {
        Ok(user_response) => HttpResponse::Ok().json(json!({
            "status": "success",
            "data": { "user": user_response }
        })),
        Err(e) => match e {
            CreateUserError::UserAlreadyExists => 
                HttpResponse::Conflict().json(json!({
                    "status": "error",
                    "message": "User already registered!"
                })),
            CreateUserError::SteamUserNotFound => 
                HttpResponse::NotFound().json(json!({
                    "status": "error",
                    "message": "Steam user not found"
                })),
            CreateUserError::DatabaseError(err) => 
                HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": format!("Database error: {:?}", err)
                })),
            _ => HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": "Internal server error"
            })),
        },
    }
}
```

### Service Layer

Services contain business logic and external API integration:

```rust
// src/services/user_service.rs
use crate::repositories::user_repository::UserRepository;
use crate::steam::steam_api_response::SteamResponse;

pub struct UserService;

impl UserService {
    pub async fn get_users(
        pool: &PgPool,
        username: Option<&str>,
        page: i64,
        limit: i64,
    ) -> Result<GetUsersResponse, sqlx::Error> {
        let offset = (page - 1) * limit;
        let (users, total) = UserRepository::get_users_with_count(
            pool, username, limit, offset
        ).await?;

        let total_pages = (total + limit - 1) / limit;

        Ok(GetUsersResponse {
            status: ResponseStatus::Success,
            users,
            pagination: PaginationMeta {
                total_in_page: users.len(),
                total,
                total_pages,
                current_page: page,
                page_size: limit,
            },
        })
    }

    async fn fetch_steam_data(steam_id: &str) -> Result<SteamResponse, CreateUserError> {
        let key = std::env::var("STEAM_KEY")
            .map_err(|_| CreateUserError::SteamApiError(
                "Steam API Key not found".to_string()
            ))?;

        let steam_api = format!(
            "http://api.steampowered.com/ISteamUser/GetPlayerSummaries/v0002/?key={}&steamids={}",
            key, steam_id
        );

        let response = reqwest::get(&steam_api).await
            .map_err(|e| CreateUserError::SteamApiError(
                format!("Failed to fetch: {:?}", e)
            ))?;

        response.json().await
            .map_err(|e| CreateUserError::SteamApiError(
                format!("Failed to parse: {:?}", e)
            ))
    }
}
```

## API Endpoints

### Health Check

- `GET /api/healthchecker` - Health check endpoint

```bash
curl http://localhost:8080/api/healthchecker
```

Response:

```json
{
  "status": "success",
  "message": "Build Rust API with actix web"
}
```

### Authentication

- `POST /api/auth/login` - User login

```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "your_username",
    "password": "your_password"
  }'
```

Response:

```json
{
  "status": "success",
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "username": "your_username"
  }
}
```

For refresh token:

```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "your_username",
    "password": "your_password",
    "refresh": true
  }'
```

### User Management

- `GET /api/users` - Get all users (with pagination)

```bash
curl "http://localhost:8080/api/users?page=1&limit=10" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

With username filter:

```bash
curl "http://localhost:8080/api/users?page=1&limit=10&username=john" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

Response:

```json
{
  "status": "success",
  "users": [...],
  "page": 1,
  "limit": 10,
  "total": 50
}
```

- `GET /api/users/:id` - Get user by ID

```bash
curl http://localhost:8080/api/users/123e4567-e89b-12d3-a456-426614174000 \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

- `POST /api/users` - Create new user

```bash
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "steam_id": "76561198012345678"
  }'
```

Response:

```json
{
  "status": "success",
  "data": {
    "user": {
      "id": "123e4567-e89b-12d3-a456-426614174000",
      "steam_id": "76561198012345678",
      "username": "PlayerName",
      "created_at": "2026-01-13T10:30:00Z"
    }
  }
}
```

- `DELETE /api/users/:id` - Delete user

```bash
curl -X DELETE http://localhost:8080/api/users/123e4567-e89b-12d3-a456-426614174000 \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

### Game Management

- `GET /api/games/:appid` - Get game details

```bash
curl http://localhost:8080/api/games/730 \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

- `POST /api/games` - Create/add game

```bash
curl -X POST http://localhost:8080/api/games \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "appid": "730"
  }'
```

### Admin

- `POST /api/admin` - Create admin user

```bash
curl -X POST http://localhost:8080/api/admin \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "username": "admin_user",
    "password": "secure_password"
  }'
```

## Database Migrations

Create a new migration:

```bash
make makemigrations
```

Run migrations:

```bash
make migrate
```

Revert last migration:

```bash
make migrate-revert
```

Prepare SQLx for offline compilation:

```bash
make prepare
```

## Auto-fix Issues

```bash
make fix
```

## CORS Configuration

The API is configured to accept requests from:

- `http://localhost:3000`
- `http://localhost:5173`

Allowed methods: GET, POST, PUT, PATCH, DELETE, OPTIONS

## Error Handling

The application includes comprehensive error handling with custom error types and standardized JSON response formats.

Example error response:

```json
{
  "status": "error",
  "message": "User not found"
}
```

Common error codes:

- `400 Bad Request` - Invalid input data
- `401 Unauthorized` - Missing or invalid authentication
- `403 Forbidden` - Insufficient permissions
- `404 Not Found` - Resource not found
- `409 Conflict` - Resource already exists
- `500 Internal Server Error` - Server-side error

## Security

- Passwords are hashed using Argon2
- JWT tokens for authentication
- Protected routes with authentication middleware
- Role-based authorization
