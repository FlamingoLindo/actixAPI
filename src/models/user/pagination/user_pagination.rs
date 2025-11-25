use crate::models::user::constants::{DEFAULT_PAGE, DEFAULT_PAGE_SIZE};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct QueryParams {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
    pub username: Option<String>,
}

fn default_page() -> i64 {
    DEFAULT_PAGE
}

fn default_limit() -> i64 {
    DEFAULT_PAGE_SIZE
}
