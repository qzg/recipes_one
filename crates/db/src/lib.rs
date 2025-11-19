pub mod recipes;
pub mod users;
pub mod sessions;
pub mod analytics;
pub mod cost;
pub mod publisher;

use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

pub use recipes::RecipeRepository;
pub use users::UserRepository;
pub use sessions::SessionRepository;
pub use analytics::AnalyticsRepository;
pub use cost::CostRepository;
pub use publisher::PublisherRepository;

/// Database connection pool and repositories
#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Create a new database connection from a connection string
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .acquire_timeout(Duration::from_secs(30))
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    /// Run database migrations
    pub async fn migrate(&self) -> Result<(), sqlx::Error> {
        sqlx::migrate!("../../migrations")
            .run(&self.pool)
            .await?;
        Ok(())
    }

    /// Get the underlying connection pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Recipe repository
    pub fn recipes(&self) -> RecipeRepository {
        RecipeRepository::new(self.pool.clone())
    }

    /// User repository
    pub fn users(&self) -> UserRepository {
        UserRepository::new(self.pool.clone())
    }

    /// Session repository
    pub fn sessions(&self) -> SessionRepository {
        SessionRepository::new(self.pool.clone())
    }

    /// Analytics repository
    pub fn analytics(&self) -> AnalyticsRepository {
        AnalyticsRepository::new(self.pool.clone())
    }

    /// Cost repository
    pub fn cost(&self) -> CostRepository {
        CostRepository::new(self.pool.clone())
    }

    /// Publisher repository
    pub fn publisher(&self) -> PublisherRepository {
        PublisherRepository::new(self.pool.clone())
    }
}
