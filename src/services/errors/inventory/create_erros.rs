#[derive(Debug)]
pub enum CreateInventoryError {
    #[allow(dead_code)]
    InventoryAlreadyExists,
    #[allow(dead_code)]
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for CreateInventoryError {
    fn from(error: sqlx::Error) -> Self {
        CreateInventoryError::DatabaseError(error)
    }
}
