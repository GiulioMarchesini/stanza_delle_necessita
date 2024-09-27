use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub total_time: u64, // in minuti
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoomState {
    pub is_free: bool,
    pub current_user: Option<String>,
    pub ip_user: Option<String>,
    pub start_time: Option<u64>, // timestamp in secondi
    pub end_time: Option<u64>,   // timestamp in secondi
}

#[derive(Debug)]
pub struct AppState {
    pub room: Mutex<RoomState>,
    pub leaderboard: Mutex<Vec<User>>,
}
