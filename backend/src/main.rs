use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod domain;
mod ledger;

mod handlers;

#[derive(Clone)]
pub struct AppState {
    pub user_manager: domain::user::UserManager,
    pub game_manager: domain::session::GameManager,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // CORS
    let cors = CorsLayer::permissive();

    let user_manager = domain::user::UserManager::new();
    let game_manager = domain::session::GameManager::new();

    let state = AppState {
        user_manager,
        game_manager,
    };

    // build our application with routes
    let app = Router::new()
        .route("/", get(handler))
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
        .route("/game/create", post(handlers::create_game))
        .route("/game/join", post(handlers::join_game))
        .route("/games", get(handlers::list_games))
        .layer(cors)
        .with_state(state);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> &'static str {
    "Megapoly Backend Online (Port 8080)"
}
