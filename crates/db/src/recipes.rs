use models::{Recipe, RecipeUpdate, ApplyPatchRequest, RecipeError};
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[derive(Clone)]
pub struct RecipeRepository {
    pool: PgPool,
}

impl RecipeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new recipe
    pub async fn create(&self, recipe: &Recipe) -> Result<Recipe, RecipeError> {
        recipe.validate().map_err(RecipeError::InvalidRecipe)?;

        let ingredients_json = serde_json::to_value(&recipe.ingredients)
            .map_err(|e| RecipeError::SerializationError(e.to_string()))?;
        let steps_json = serde_json::to_value(&recipe.steps)
            .map_err(|e| RecipeError::SerializationError(e.to_string()))?;
        let images_json = serde_json::to_value(&recipe.images)
            .map_err(|e| RecipeError::SerializationError(e.to_string()))?;

        let row = sqlx::query(
            r#"
            INSERT INTO recipes (id, user_id, version, title, servings, description, images, ingredients, steps)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, user_id, version, title, servings, description, images, ingredients, steps, created_at, updated_at
            "#
        )
        .bind(&recipe.id)
        .bind(&recipe.user_id)
        .bind(&recipe.version)
        .bind(&recipe.title)
        .bind(recipe.servings as i32)
        .bind(&recipe.description)
        .bind(&images_json)
        .bind(&ingredients_json)
        .bind(&steps_json)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RecipeError::DatabaseError(e.to_string()))?;

        self.row_to_recipe(row)
    }

    /// Get a recipe by ID
    pub async fn get(&self, recipe_id: Uuid) -> Result<Recipe, RecipeError> {
        let row = sqlx::query(
            r#"
            SELECT id, user_id, version, title, servings, description, images, ingredients, steps, created_at, updated_at
            FROM recipes
            WHERE id = $1
            "#
        )
        .bind(recipe_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RecipeError::DatabaseError(e.to_string()))?
        .ok_or_else(|| RecipeError::NotFound(recipe_id.to_string()))?;

        self.row_to_recipe(row)
    }

    /// Get recipes by user ID
    pub async fn list_by_user(&self, user_id: Uuid, limit: i64, offset: i64) -> Result<Vec<Recipe>, RecipeError> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, version, title, servings, description, images, ingredients, steps, created_at, updated_at
            FROM recipes
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RecipeError::DatabaseError(e.to_string()))?;

        rows.into_iter()
            .map(|row| self.row_to_recipe(row))
            .collect()
    }

    /// Apply a patch update to a recipe with optimistic concurrency control
    pub async fn apply_patch(&self, request: ApplyPatchRequest) -> Result<Recipe, RecipeError> {
        // Start a transaction
        let mut tx = self.pool.begin().await
            .map_err(|e| RecipeError::DatabaseError(e.to_string()))?;

        // Fetch current recipe with row lock
        let current = sqlx::query(
            r#"
            SELECT id, user_id, version, title, servings, description, images, ingredients, steps, created_at, updated_at
            FROM recipes
            WHERE id = $1
            FOR UPDATE
            "#
        )
        .bind(request.recipe_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| RecipeError::DatabaseError(e.to_string()))?
        .ok_or_else(|| RecipeError::NotFound(request.recipe_id.to_string()))?;

        let current_version: i32 = current.get("version");

        // Check version
        if current_version != request.expected_version {
            return Err(RecipeError::VersionConflict {
                expected: request.expected_version,
                actual: current_version,
            });
        }

        // Apply update
        let mut updated = self.row_to_recipe(current)?;
        let new_version = updated.version + 1;

        match request.update {
            RecipeUpdate::FullRecipe(full_recipe) => {
                updated = full_recipe;
                updated.version = new_version;
            }
            RecipeUpdate::PartialUpdate {
                title,
                servings,
                description,
                images,
                ingredients,
                steps,
            } => {
                if let Some(t) = title {
                    updated.title = t;
                }
                if let Some(s) = servings {
                    updated.servings = s;
                }
                if let Some(d) = description {
                    updated.description = Some(d);
                }
                if let Some(i) = images {
                    updated.images = i;
                }
                if let Some(ing) = ingredients {
                    updated.ingredients = ing;
                }
                if let Some(st) = steps {
                    updated.steps = st;
                }
                updated.version = new_version;
            }
        }

        // Validate
        updated.validate().map_err(RecipeError::InvalidRecipe)?;

        // Save version snapshot
        let version_data = serde_json::to_value(&updated)
            .map_err(|e| RecipeError::SerializationError(e.to_string()))?;

        sqlx::query(
            "INSERT INTO recipe_versions (recipe_id, version, data) VALUES ($1, $2, $3)"
        )
        .bind(request.recipe_id)
        .bind(updated.version)
        .bind(version_data)
        .execute(&mut *tx)
        .await
        .map_err(|e| RecipeError::DatabaseError(e.to_string()))?;

        // Update recipe
        let ingredients_json = serde_json::to_value(&updated.ingredients)
            .map_err(|e| RecipeError::SerializationError(e.to_string()))?;
        let steps_json = serde_json::to_value(&updated.steps)
            .map_err(|e| RecipeError::SerializationError(e.to_string()))?;
        let images_json = serde_json::to_value(&updated.images)
            .map_err(|e| RecipeError::SerializationError(e.to_string()))?;

        let row = sqlx::query(
            r#"
            UPDATE recipes
            SET version = $2, title = $3, servings = $4, description = $5,
                images = $6, ingredients = $7, steps = $8, updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, version, title, servings, description, images, ingredients, steps, created_at, updated_at
            "#
        )
        .bind(request.recipe_id)
        .bind(updated.version)
        .bind(&updated.title)
        .bind(updated.servings as i32)
        .bind(&updated.description)
        .bind(&images_json)
        .bind(&ingredients_json)
        .bind(&steps_json)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| RecipeError::DatabaseError(e.to_string()))?;

        tx.commit().await
            .map_err(|e| RecipeError::DatabaseError(e.to_string()))?;

        self.row_to_recipe(row)
    }

    /// Delete a recipe
    pub async fn delete(&self, recipe_id: Uuid) -> Result<(), RecipeError> {
        sqlx::query("DELETE FROM recipes WHERE id = $1")
            .bind(recipe_id)
            .execute(&self.pool)
            .await
            .map_err(|e| RecipeError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Helper to convert database row to Recipe
    fn row_to_recipe(&self, row: sqlx::postgres::PgRow) -> Result<Recipe, RecipeError> {
        let ingredients_json: serde_json::Value = row.get("ingredients");
        let steps_json: serde_json::Value = row.get("steps");
        let images_json: serde_json::Value = row.get("images");

        let ingredients = serde_json::from_value(ingredients_json)
            .map_err(|e| RecipeError::SerializationError(e.to_string()))?;
        let steps = serde_json::from_value(steps_json)
            .map_err(|e| RecipeError::SerializationError(e.to_string()))?;
        let images = serde_json::from_value(images_json)
            .map_err(|e| RecipeError::SerializationError(e.to_string()))?;

        Ok(Recipe {
            id: row.get("id"),
            user_id: row.get("user_id"),
            version: row.get("version"),
            title: row.get("title"),
            servings: row.get::<i32, _>("servings") as u32,
            description: row.get("description"),
            images,
            ingredients,
            steps,
            created_at: Some(row.get("created_at")),
            updated_at: Some(row.get("updated_at")),
        })
    }
}
