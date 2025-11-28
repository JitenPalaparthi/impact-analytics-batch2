mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod state;

use crate::config::Config;
use crate::db::create_pool;
use crate::routes::create_router;
use crate::state::AppState;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "axum_users_service=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();

    let pool = create_pool(&config.database_url).await?;
    let state = AppState { pool };

    let app = create_router(state);

    tracing::info!("listening on {}", config.server_addr);
    let listener = tokio::net::TcpListener::bind(config.server_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
