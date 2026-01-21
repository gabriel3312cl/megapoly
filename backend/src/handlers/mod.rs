use crate::domain::game::Game;
use crate::AppState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

// --- Auth Handlers ---

#[derive(Deserialize)]
pub struct AuthPayload {
    username: String,
    password: String, // In real app, this is plaintext to be hashed
}

#[derive(Serialize)]
pub struct AuthResponse {
    token: String, // In real app, JWT. Here, just username/ID
    user_id: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> impl IntoResponse {
    match state
        .user_manager
        .register(payload.username, payload.password)
    {
        Ok(user) => (
            StatusCode::CREATED,
            Json(AuthResponse {
                token: user.username,
                user_id: user.id,
            }),
        )
            .into_response(),
        Err(e) => (StatusCode::CONFLICT, e).into_response(),
    }
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> impl IntoResponse {
    match state.user_manager.login(payload.username, payload.password) {
        Some(user) => (
            StatusCode::OK,
            Json(AuthResponse {
                token: user.username,
                user_id: user.id,
            }),
        )
            .into_response(),
        None => (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
    }
}

// --- Game Handlers ---

#[derive(Deserialize)]
pub struct CreateGamePayload {
    host_id: String, // In real app, extract from Token
}

#[derive(Serialize)]
pub struct CreateGameResponse {
    game_code: String,
}

pub async fn create_game(
    State(state): State<AppState>,
    Json(payload): Json<CreateGamePayload>,
) -> impl IntoResponse {
    let code = state.game_manager.create_game(payload.host_id);
    (
        StatusCode::CREATED,
        Json(CreateGameResponse { game_code: code }),
    )
        .into_response()
}

#[derive(Deserialize)]
pub struct JoinGamePayload {
    user_id: String,
    game_code: String,
}

pub async fn join_game(
    State(state): State<AppState>,
    Json(payload): Json<JoinGamePayload>,
) -> impl IntoResponse {
    if let Some(game) = state.game_manager.get_game(&payload.game_code) {
        // Logic to add player to game...
        // For now just return 200 OK
        return (StatusCode::OK, "Joined").into_response();
    }
    (StatusCode::NOT_FOUND, "Game not found").into_response()
}

pub async fn list_games(State(state): State<AppState>) -> impl IntoResponse {
    let games = state.game_manager.list_public_games();
    (StatusCode::OK, Json(games)).into_response()
}

pub async fn get_game_state(
    State(state): State<AppState>,
    Path(game_code): Path<String>,
) -> impl IntoResponse {
    if let Some(game) = state.game_manager.get_game(&game_code) {
        let game_lock = game.lock().await;
        // Return JSON representation of game
        return (StatusCode::OK, Json(game_lock.clone())).into_response();
    }
    (StatusCode::NOT_FOUND, "Game not found").into_response()
}

#[derive(Deserialize)]
pub struct GameActionPayload {
    action: String, // "roll", "end_turn"
    user_id: String,
}

pub async fn perform_game_action(
    State(state): State<AppState>,
    Path(game_code): Path<String>,
    Json(payload): Json<GameActionPayload>,
) -> impl IntoResponse {
    if let Some(game) = state.game_manager.get_game(&game_code) {
        let mut game_lock = game.lock().await;

        // Simple logic for now
        match payload.action.as_str() {
            "roll" => {
                game_lock.roll_dice();
                return (StatusCode::OK, "Rolled").into_response();
            }
            "end_turn" => {
                game_lock.end_turn();
                return (StatusCode::OK, "Turn Ended").into_response();
            }
            _ => return (StatusCode::BAD_REQUEST, "Unknown Action").into_response(),
        }
    }
    (StatusCode::NOT_FOUND, "Game not found").into_response()
}
