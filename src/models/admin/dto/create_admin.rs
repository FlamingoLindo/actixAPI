use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAdminSchema {
    pub username: String,
    pub password: String,
    pub role_name: String,
}
