mod app;
mod net;
mod room;

use axum::{routing::get, Router};
use std::{error::Error, net::SocketAddr};
use tracing_subscriber::EnvFilter;

use crate::app::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let state = AppState::new();

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/ws", get(net::ws::ws_handler))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server listening on ws://{}/ws", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
