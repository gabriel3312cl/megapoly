// Game logic
use crate::domain::board::Board;
use crate::domain::player::Player;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub board: Board,
    pub players: Vec<Player>,
    pub current_player_index: usize,
    pub state: GameState,
    pub dice: Dice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameState {
    WaitingForStart,
    WaitingForRoll,
    Moving,
    WaitingForAction, // Buy, Auction, Pay Rent
    TurnEnded,
    GameOver,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dice {
    pub die1: u8,
    pub die2: u8,
}

impl Dice {
    pub fn new() -> Self {
        Self { die1: 0, die2: 0 }
    }

    pub fn roll(&mut self) -> (u8, u8) {
        let mut rng = rand::rng();
        self.die1 = rng.random_range(1..=6);
        self.die2 = rng.random_range(1..=6);
        (self.die1, self.die2)
    }

    pub fn total(&self) -> u8 {
        self.die1 + self.die2
    }

    pub fn is_double(&self) -> bool {
        self.die1 == self.die2 && self.die1 > 0
    }
}
