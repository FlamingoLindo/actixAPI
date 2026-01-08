#[derive(Debug)]
pub enum GetGameError {
    GameNotFound,
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for GetGameError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => GetGameError::GameNotFound,
            _ => GetGameError::DatabaseError(error),
        }
    }
}
