use serde::{Deserialize, Serialize};

use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub total_time: u64, // Tempo in minuti
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RoomState {
    pub status: String, // "libera" o "occupata"
    pub current_user: Option<String>,
}
