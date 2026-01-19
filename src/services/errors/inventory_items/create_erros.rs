#[derive(Debug)]
pub enum CreateInventoryItemError {
    #[allow(dead_code)]
    InventoryItemAlreadyExists,
    #[allow(dead_code)]
    DatabaseError(sqlx::Error),
    #[allow(dead_code)]
    SteamApiError(String),
    #[allow(dead_code)]
    InventoryNotFound,
}

impl From<sqlx::Error> for CreateInventoryItemError {
    fn from(error: sqlx::Error) -> Self {
        CreateInventoryItemError::DatabaseError(error)
    }
}
