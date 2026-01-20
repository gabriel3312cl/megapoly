use crate::domain::game::Game;
use dashmap::DashMap;
use rand::{distr::Alphanumeric, Rng};
use std::sync::Arc;
use tokio::sync::Mutex; // Async mutex for Game since it will be accessed by async handlers

#[derive(Clone)]
pub struct GameManager {
    // 4-digit code -> Game
    games: Arc<DashMap<String, Arc<Mutex<Game>>>>,
}

impl GameManager {
    pub fn new() -> Self {
        Self {
            games: Arc::new(DashMap::new()),
        }
    }

    pub fn create_game(&self, host_id: String) -> String {
        let code = self.generate_code();
        let game = Game {
            id: code.clone(),
            board: crate::domain::board::Board { spaces: vec![] }, // Initialize empty for now or default
            players: vec![],
            current_player_index: 0,
            state: crate::domain::game::GameState::WaitingForStart,
            dice: crate::domain::game::Dice::new(),
        };
        self.games.insert(code.clone(), Arc::new(Mutex::new(game)));
        code
    }

    pub fn get_game(&self, code: &str) -> Option<Arc<Mutex<Game>>> {
        self.games.get(code).map(|g| g.clone())
    }

    fn generate_code(&self) -> String {
        let mut rng = rand::rng();
        loop {
            let code: String = (0..4)
                .map(|_| rng.sample(Alphanumeric))
                .map(char::from)
                .collect::<String>()
                .to_uppercase();

            if !self.games.contains_key(&code) {
                return code;
            }
        }
    }

    pub fn list_public_games(&self) -> Vec<String> {
        // Return codes for now
        self.games.iter().map(|entry| entry.key().clone()).collect()
    }
}
