use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    GameStarted {
        timestamp: i64,
    },
    DiceRolled {
        player_id: String,
        die1: u8,
        die2: u8,
        timestamp: i64,
    },
    PlayerMoved {
        player_id: String,
        from: usize,
        to: usize,
        timestamp: i64,
    },
    Transaction {
        tx_id: String,
        description: String,
        amount: i64,
        from: String,
        to: String,
        timestamp: i64,
    },
    PropertyBought {
        player_id: String,
        property_id: usize,
        price: i64,
        timestamp: i64,
    },
    TurnEnded {
        player_id: String,
        timestamp: i64,
    },
    // Add more as needed
}

#[derive(Clone)]
pub struct EventLog {
    pub events: Arc<Mutex<Vec<GameEvent>>>,
}

impl EventLog {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn log(&self, event: GameEvent) {
        self.events.lock().unwrap().push(event);
    }

    pub fn get_history(&self) -> Vec<GameEvent> {
        self.events.lock().unwrap().clone()
    }
}
