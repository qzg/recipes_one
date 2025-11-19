mod api;
mod auth;
mod tools;
mod middleware;
mod config;
mod state;

use axum::{
    Router,
    routing::{get, post},
};
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::net::SocketAddr;

use crate::config::Config;
use crate::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug,tower_http=debug,axum=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Realtime Recipe Assistant Server");

    // Load configuration
    dotenvy::dotenv().ok();
    let config = Config::from_env()?;

    // Initialize database
    let database = db::Database::new(&config.database_url).await?;
    database.migrate().await?;
    tracing::info!("Database connected and migrated");

    // Initialize OpenAI client
    let openai_client = openai::OpenAIClient::new(config.openai_api_key.clone());

    // Create application state
    let state = AppState::new(database, openai_client, config.clone());

    // Build application router
    let app = Router::new()
        // Health check
        .route("/health", get(health_check))

        // Auth routes
        .route("/api/auth/register", post(api::auth::register))
        .route("/api/auth/login", post(api::auth::login))

        // Session routes
        .route("/api/sessions", post(api::sessions::create_session))

        // Recipe routes
        .route("/api/recipes", get(api::recipes::list_recipes))
        .route("/api/recipes", post(api::recipes::create_recipe))
        .route("/api/recipes/:id", get(api::recipes::get_recipe))
        .route("/api/recipes/:id", post(api::recipes::update_recipe))
        .route("/api/recipes/:id", axum::routing::delete(api::recipes::delete_recipe))

        // User profile routes
        .route("/api/profile", get(api::profile::get_profile))
        .route("/api/profile", post(api::profile::update_profile))

        // Analytics routes
        .route("/api/analytics/event", post(api::analytics::log_event))
        .route("/api/analytics/conversion", post(api::analytics::record_conversion))
        .route("/api/analytics/recipe/:id", get(api::analytics::get_recipe_analytics))

        // Webhook for OpenAI Realtime
        .route("/api/webhook/realtime", post(api::webhook::realtime_webhook))

        // CORS layer
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )

        // Tracing layer
        .layer(TraceLayer::new_for_http())

        // Application state
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
