#[derive(Debug)]
pub enum UpdateUserError {
    UserNotFound,
    SteamUserNotFound,
    SteamApiError(String),
    DatabaseError(sqlx::Error),
    GameCreationError(String),
}

impl From<sqlx::Error> for UpdateUserError {
    fn from(error: sqlx::Error) -> Self {
        UpdateUserError::DatabaseError(error)
    }
}

impl From<crate::services::errors::games::create_errors::CreateGameError> for UpdateUserError {
    fn from(error: crate::services::errors::games::create_errors::CreateGameError) -> Self {
        match error {
            crate::services::errors::games::create_errors::CreateGameError::GameAlreadyExists => {
                UpdateUserError::GameCreationError("Game already exists".to_string())
            }
            crate::services::errors::games::create_errors::CreateGameError::SteamApiError(msg) => {
                UpdateUserError::SteamApiError(msg)
            }
            crate::services::errors::games::create_errors::CreateGameError::SteamGameNotFound => {
                UpdateUserError::GameCreationError("Steam game not found".to_string())
            }
            crate::services::errors::games::create_errors::CreateGameError::DatabaseError(err) => {
                UpdateUserError::DatabaseError(err)
            }
        }
    }
}
