use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub openai_api_key: String,
    pub jwt_secret: String,
    pub port: u16,
    #[serde(default = "default_webhook_secret")]
    pub webhook_secret: String,
}

fn default_webhook_secret() -> String {
    "changeme".to_string()
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost/recipes".to_string());

        let openai_api_key = std::env::var("OPENAI_API_KEY")
            .expect("OPENAI_API_KEY must be set");

        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "secret_key_change_in_production".to_string());

        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);

        let webhook_secret = std::env::var("WEBHOOK_SECRET")
            .unwrap_or_else(|_| default_webhook_secret());

        Ok(Self {
            database_url,
            openai_api_key,
            jwt_secret,
            port,
            webhook_secret,
        })
    }
}
