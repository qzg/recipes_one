// API client for calling backend endpoints

use gloo_net::http::Request;
use models::*;
use uuid::Uuid;

pub struct ApiClient {
    base_url: String,
    token: Option<String>,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            token: None,
        }
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<LoginResponse, String> {
        let request = LoginRequest {
            email: email.to_string(),
            password: password.to_string(),
        };

        let response = Request::post(&format!("{}/api/auth/login", self.base_url))
            .json(&request)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !response.ok() {
            return Err(format!("Login failed: {}", response.status()));
        }

        response.json().await.map_err(|e| e.to_string())
    }

    pub async fn get_recipe(&self, id: Uuid) -> Result<Recipe, String> {
        let mut req = Request::get(&format!("{}/api/recipes/{}", self.base_url, id));

        if let Some(token) = &self.token {
            req = req.header("Authorization", &format!("Bearer {}", token));
        }

        let response = req.send().await.map_err(|e| e.to_string())?;

        if !response.ok() {
            return Err(format!("Failed to fetch recipe: {}", response.status()));
        }

        response.json().await.map_err(|e| e.to_string())
    }

    pub async fn list_recipes(&self) -> Result<Vec<Recipe>, String> {
        let mut req = Request::get(&format!("{}/api/recipes", self.base_url));

        if let Some(token) = &self.token {
            req = req.header("Authorization", &format!("Bearer {}", token));
        }

        let response = req.send().await.map_err(|e| e.to_string())?;

        if !response.ok() {
            return Err(format!("Failed to fetch recipes: {}", response.status()));
        }

        response.json().await.map_err(|e| e.to_string())
    }

    pub async fn create_session(&self, recipe_id: Option<Uuid>) -> Result<CreateSessionResponse, String> {
        let mut req = Request::post(&format!("{}/api/sessions", self.base_url))
            .json(&CreateSessionRequest { recipe_id })
            .map_err(|e| e.to_string())?;

        if let Some(token) = &self.token {
            req = req.header("Authorization", &format!("Bearer {}", token));
        }

        let response = req.send().await.map_err(|e| e.to_string())?;

        if !response.ok() {
            return Err(format!("Failed to create session: {}", response.status()));
        }

        response.json().await.map_err(|e| e.to_string())
    }
}
