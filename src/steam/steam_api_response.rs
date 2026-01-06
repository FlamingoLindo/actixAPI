use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]

pub struct SteamResponse {
    pub response: SteamPlayers,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]

pub struct SteamPlayers {
    pub players: Vec<SteamPlayer>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SteamPlayer {
    pub steamid: String,
    pub personaname: String,
    pub profileurl: String,
    pub avatar: String,
    pub avatarmedium: String,
    pub avatarfull: String,
    pub personastate: i32,
    pub communityvisibilitystate: i32,
    pub timecreated: Option<i64>,
    pub loccountrycode: Option<String>,
    pub gameextrainfo: Option<String>,
    pub gameid: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SteamGameResponse {
    #[serde(flatten)]
    pub games: std::collections::HashMap<String, SteamGameWrapper>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SteamGameWrapper {
    pub success: bool,
    pub data: Option<SteamGame>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SteamGame {
    #[serde(rename = "type")]
    pub game_type: String,
    pub name: String,
    pub steam_appid: u32,
    pub required_age: i32,
    pub is_free: bool,
    pub controller_support: Option<String>,
    pub dlc: Option<Vec<u32>>,
    pub detailed_description: String,
    pub about_the_game: String,
    pub short_description: String,
    pub supported_languages: String,
    pub header_image: String,
    pub capsule_image: String,
    pub capsule_imagev5: String,
    pub website: Option<String>,
    pub pc_requirements: Option<Requirements>,
    pub mac_requirements: Option<Requirements>,
    pub linux_requirements: Option<Requirements>,
    pub developers: Vec<String>,
    pub publishers: Vec<String>,
    pub price_overview: Option<PriceOverview>,
    pub platforms: Platforms,
    pub metacritic: Option<Metacritic>,
    pub categories: Option<Vec<Category>>,
    pub genres: Option<Vec<Genre>>,
    pub screenshots: Option<Vec<Screenshot>>,
    pub movies: Option<Vec<Movie>>,
    pub recommendations: Option<Recommendations>,
    pub achievements: Option<Achievements>,
    pub release_date: ReleaseDate,
    pub support_info: SupportInfo,
    pub background: Option<String>,
    pub background_raw: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[allow(dead_code)]
pub enum Requirements {
    Full {
        minimum: Option<String>,
        recommended: Option<String>,
    },
    Empty(Vec<()>),
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct PriceOverview {
    pub currency: Option<String>,
    pub initial: Option<i32>,
    #[serde(rename = "final")]
    pub final_price: Option<i32>,
    pub discount_percent: Option<i32>,
    pub initial_formatted: Option<String>,
    pub final_formatted: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Platforms {
    pub windows: bool,
    pub mac: bool,
    pub linux: bool,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Metacritic {
    pub score: i32,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Category {
    pub id: i32,
    pub description: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Genre {
    pub id: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Screenshot {
    pub id: i32,
    pub path_thumbnail: String,
    pub path_full: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Movie {
    pub id: i32,
    pub name: String,
    pub thumbnail: String,
    pub webm: Option<WebmFormats>,
    pub mp4: Option<Mp4Formats>,
    pub highlight: bool,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct WebmFormats {
    #[serde(rename = "480")]
    pub res_480: Option<String>,
    pub max: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Mp4Formats {
    #[serde(rename = "480")]
    pub res_480: Option<String>,
    pub max: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Recommendations {
    pub total: i32,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Achievements {
    pub total: i32,
    pub highlighted: Option<Vec<Achievement>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Achievement {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ReleaseDate {
    pub coming_soon: bool,
    pub date: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SupportInfo {
    pub url: String,
    pub email: String,
}
