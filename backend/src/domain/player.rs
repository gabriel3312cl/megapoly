// Player Logic
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub money: i64, // Stored in smallest unit? Or dollars? Assuming dollars as per rules, but handled as i64
    pub position: usize,
    pub in_jail: bool,
    pub jail_turns: u8,
}
