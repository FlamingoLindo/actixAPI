use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGameRequest {
    pub appid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGameSchema {
    pub appid: String,
    pub name: String,
    pub short_description: String,
    pub header_image: String,
    pub screenshots: Vec<String>,
}