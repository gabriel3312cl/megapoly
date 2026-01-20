use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    // in real app, hash this.
    pub password: String,
}

#[derive(Clone)]
pub struct UserManager {
    // username -> User
    users: Arc<Mutex<HashMap<String, User>>>,
}

impl UserManager {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register(&self, username: String, password: String) -> Result<User, String> {
        let mut users = self.users.lock().unwrap();
        if users.contains_key(&username) {
            return Err("Username already exists".to_string());
        }

        let user = User {
            id: Uuid::new_v4().to_string(),
            username: username.clone(),
            password,
        };

        users.insert(username, user.clone());
        Ok(user)
    }

    pub fn login(&self, username: String, password: String) -> Option<User> {
        let users = self.users.lock().unwrap();
        if let Some(user) = users.get(&username) {
            if user.password == password {
                return Some(user.clone());
            }
        }
        None
    }
}
