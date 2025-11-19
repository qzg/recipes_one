use models::{CostAnalysis, PricingCatalogEntry, CostSource};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct CostRepository {
    pool: PgPool,
}

impl CostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Save a cost analysis run
    pub async fn save_analysis(&self, analysis: &CostAnalysis) -> Result<(), sqlx::Error> {
        let breakdown_json = serde_json::to_value(&analysis.breakdown)
            .map_err(|e| sqlx::Error::decode(e))?;
        let source_str = serde_json::to_string(&analysis.source)
            .map_err(|e| sqlx::Error::decode(e))?
            .trim_matches('"').to_string();

        sqlx::query!(
            r#"
            INSERT INTO cost_runs (user_id, recipe_id, recipe_version, total_cost_cents,
                                  cost_per_serving_cents, region, store, source, breakdown, computed_at)
            VALUES ((SELECT user_id FROM recipes WHERE id = $1), $1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            analysis.recipe_id,
            analysis.recipe_version,
            analysis.total_cost_cents,
            analysis.cost_per_serving_cents,
            analysis.region,
            analysis.store,
            source_str,
            breakdown_json,
            analysis.computed_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get latest cost analysis for a recipe
    pub async fn get_latest_analysis(&self, recipe_id: Uuid) -> Result<Option<CostAnalysis>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT recipe_id, recipe_version, total_cost_cents, cost_per_serving_cents,
                   region, store, source, breakdown, computed_at
            FROM cost_runs
            WHERE recipe_id = $1
            ORDER BY computed_at DESC
            LIMIT 1
            "#,
            recipe_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            let breakdown = serde_json::from_value(r.breakdown).unwrap_or_default();
            let source = serde_json::from_str(&format!("\"{}\"", r.source)).unwrap_or(CostSource::ModelBased);

            CostAnalysis {
                recipe_id: r.recipe_id,
                recipe_version: r.recipe_version,
                total_cost_cents: r.total_cost_cents,
                cost_per_serving_cents: r.cost_per_serving_cents,
                region: r.region,
                store: r.store,
                source,
                breakdown,
                computed_at: r.computed_at,
            }
        }))
    }

    /// Add or update pricing catalog entry
    pub async fn upsert_catalog_entry(&self, entry: &PricingCatalogEntry) -> Result<(), sqlx::Error> {
        let source_str = serde_json::to_string(&entry.source)
            .map_err(|e| sqlx::Error::decode(e))?
            .trim_matches('"').to_string();

        sqlx::query!(
            r#"
            INSERT INTO cost_catalog (id, ingredient_name, region, store, price_cents, quantity, unit, source, last_updated)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE
            SET price_cents = $5, quantity = $6, unit = $7, source = $8, last_updated = $9
            "#,
            entry.id,
            entry.ingredient_name,
            entry.region,
            entry.store,
            entry.price_cents,
            entry.quantity,
            entry.unit,
            source_str,
            entry.last_updated
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Look up ingredient pricing
    pub async fn lookup_ingredient(
        &self,
        ingredient_name: &str,
        region: &str,
    ) -> Result<Option<PricingCatalogEntry>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT id, ingredient_name, region, store, price_cents, quantity, unit, source, last_updated
            FROM cost_catalog
            WHERE LOWER(ingredient_name) = LOWER($1) AND region = $2
            ORDER BY last_updated DESC
            LIMIT 1
            "#,
            ingredient_name,
            region
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            let source = serde_json::from_str(&format!("\"{}\"", r.source)).unwrap_or(CostSource::ModelBased);
            let quantity_f64 = r.quantity.to_string().parse::<f64>().unwrap_or(0.0);

            PricingCatalogEntry {
                id: r.id,
                ingredient_name: r.ingredient_name,
                region: r.region,
                store: r.store,
                price_cents: r.price_cents,
                quantity: quantity_f64,
                unit: r.unit,
                source,
                last_updated: r.last_updated,
            }
        }))
    }
}
