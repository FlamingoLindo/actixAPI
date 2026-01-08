#[derive(Debug)]
pub enum CreateGameError {
    #[allow(dead_code)]
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
