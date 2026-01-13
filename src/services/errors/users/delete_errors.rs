#[derive(Debug)]
pub enum DeleteUserError {
    UserNotFound,
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for DeleteUserError {
    fn from(error: sqlx::Error) -> Self {
        DeleteUserError::DatabaseError(error)
    }
}
