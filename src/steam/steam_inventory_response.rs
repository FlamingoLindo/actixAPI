use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SteamInventoryResponse {
    pub assets: Option<Vec<Asset>>,
    pub descriptions: Option<Vec<Description>>,
    pub total_inventory_count: i32,
    pub success: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Asset {
    pub appid: i32,
    pub contextid: String,
    pub assetid: String,
    pub classid: String,
    pub instanceid: String,
    pub amount: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Description {
    pub appid: i32,
    pub classid: String,
    pub instanceid: String,
    #[serde(default)]
    pub icon_url: String,
    pub name: String,
    #[serde(default)]
    pub name_color: String,
    #[serde(rename = "type", default)]
    pub item_type: String,
    #[serde(default)]
    pub descriptions: Vec<DescriptionDetail>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DescriptionDetail {
    #[serde(rename = "type", default)]
    pub detail_type: String,
    pub value: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub color: String,
}
