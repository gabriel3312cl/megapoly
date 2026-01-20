use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod domain;
mod ledger;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // CORS
    let cors = CorsLayer::permissive();

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .layer(cors);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> &'static str {
    "Megapoly Backend Online"
}
