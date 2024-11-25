use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct CurrentServer {
    #[serde(rename = "gameId")]
    pub game_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GamePlayerInfo {
    pub rank: u64,
    pub latency: u64,
    pub slot: u64,
    pub join_time: u64,
    pub localization: String,
    pub user_id: u64,
    pub player_id: u64,
    pub name: String,
    pub platform: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GamePlayers {
    pub players: std::collections::HashMap<u64, GamePlayerInfo>,
    pub team_1: Vec<u64>,
    pub team_2: Vec<u64>,
    pub spectators: Vec<u64>,
    pub loading: Vec<u64>,
    pub que: Vec<u64>,
    pub server_info: ServerInfo,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerInfo {
    server_name: String,
    admins: Vec<String>,
    country: String,
    description: String,
    experience: String,
    fairfight: String,
    level: String,
    mode: String,
    lowrankonly: String,
    maps: Vec<String>,
    owner: String,
    settings: Vec<String>,
    vips: Vec<String>,
    region: String,
    servertype: String,
}
