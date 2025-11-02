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
}
