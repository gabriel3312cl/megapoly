use crate::AppState;
use axum::{
    extract::{Json, State},
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
