use serde::{Deserialize, Deserializer};

// Helper function to deserialize strings or integers as i32
fn deserialize_string_or_int<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        String(String),
        Int(i32),
    }

    match StringOrInt::deserialize(deserializer)? {
        StringOrInt::String(s) => s.parse::<i32>().map_err(serde::de::Error::custom),
        StringOrInt::Int(i) => Ok(i),
    }
}

// Helper function for optional string or int
fn deserialize_option_string_or_int<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        String(String),
        Int(i32),
    }

    let opt: Option<StringOrInt> = Option::deserialize(deserializer)?;
    match opt {
        Some(StringOrInt::String(s)) => {
            s.parse::<i32>().map(Some).map_err(serde::de::Error::custom)
        }
        Some(StringOrInt::Int(i)) => Ok(Some(i)),
        None => Ok(None),
    }
}

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
    #[serde(deserialize_with = "deserialize_string_or_int")]
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
    #[serde(deserialize_with = "deserialize_option_string_or_int")]
    pub initial: Option<i32>,
    #[serde(
        rename = "final",
        deserialize_with = "deserialize_option_string_or_int"
    )]
    pub final_price: Option<i32>,
    #[serde(deserialize_with = "deserialize_option_string_or_int")]
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
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub score: i32,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Category {
    #[serde(deserialize_with = "deserialize_string_or_int")]
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
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub id: i32,
    pub path_thumbnail: String,
    pub path_full: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Movie {
    #[serde(deserialize_with = "deserialize_string_or_int")]
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
    #[serde(deserialize_with = "deserialize_string_or_int")]
    pub total: i32,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Achievements {
    #[serde(deserialize_with = "deserialize_string_or_int")]
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
