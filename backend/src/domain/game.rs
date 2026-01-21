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

impl Game {
    pub fn new(id: String) -> Self {
        Self {
            id,
            board: crate::domain::board::Board { spaces: vec![] }, // Should load from config
            players: vec![],
            current_player_index: 0,
            state: GameState::WaitingForStart,
            dice: Dice::new(),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn roll_dice(&mut self) {
        let (d1, d2) = self.dice.roll();
        // Move current player
        if let Some(player) = self.players.get_mut(self.current_player_index) {
            let moves = (d1 + d2) as usize;
            player.position = (player.position + moves) % 40;
            // Handle Go, Jail, etc later
        }
        self.state = GameState::WaitingForAction;
    }

    pub fn end_turn(&mut self) {
        if self.players.is_empty() {
            return;
        }
        self.current_player_index = (self.current_player_index + 1) % self.players.len();
        self.state = GameState::WaitingForRoll;
    }
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
