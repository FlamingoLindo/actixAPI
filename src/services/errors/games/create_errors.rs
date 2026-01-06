#[derive(Debug)]
pub enum CreateGameError {
    GameAlreadyExists,
    SteamApiError(String),
    SteamGameNotFound,
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for CreateGameError {
    fn from(error: sqlx::Error) -> Self {
        CreateGameError::DatabaseError(error)
    }
}
