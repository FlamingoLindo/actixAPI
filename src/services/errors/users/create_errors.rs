#[derive(Debug)]
pub enum CreateUserError {
    UserAlreadyExists,
    SteamApiError(String),
    SteamUserNotFound,
    DatabaseError(sqlx::Error),
    GameCreationError(String),
}

impl From<sqlx::Error> for CreateUserError {
    fn from(error: sqlx::Error) -> Self {
        CreateUserError::DatabaseError(error)
    }
}

impl From<crate::services::errors::games::create_errors::CreateGameError> for CreateUserError {
    fn from(error: crate::services::errors::games::create_errors::CreateGameError) -> Self {
        CreateUserError::GameCreationError(format!("{:?}", error))
    }
}
