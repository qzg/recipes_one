use crate::config::Config;
use db::Database;
use openai::OpenAIClient;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub openai: OpenAIClient,
    pub config: Config,
}

impl AppState {
    pub fn new(db: Database, openai: OpenAIClient, config: Config) -> Self {
        Self { db, openai, config }
    }
}
