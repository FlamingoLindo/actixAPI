#[derive(Debug)]
pub enum GetUserError {
    UserNotFound,
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for GetUserError {
    fn from(error: sqlx::Error) -> Self {
        GetUserError::DatabaseError(error)
    }
}
