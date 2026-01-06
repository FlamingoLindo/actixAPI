#[derive(Debug)]
pub enum UpdateUserError {
    UserNotFound,
    SteamUserNotFound,
    SteamApiError(String),
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for UpdateUserError {
    fn from(error: sqlx::Error) -> Self {
        UpdateUserError::DatabaseError(error)
    }
}
