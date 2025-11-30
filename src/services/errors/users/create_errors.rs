#[derive(Debug)]
pub enum CreateUserError {
    UserAlreadyExists,
    SteamApiError(String),
    SteamUserNotFound,
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for CreateUserError {
    fn from(error: sqlx::Error) -> Self {
        CreateUserError::DatabaseError(error)
    }
}
