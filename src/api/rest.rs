use super::auth;
use reqwest::Client;

/// HTTP client wrapper for Upbit REST API with JWT authentication
#[derive(Clone)]
pub struct RestClient {
    client: Client,
    base_url: String,
    access_key: Option<String>,
    secret_key: Option<String>,
}

impl RestClient {
    /// Create a new RestClient with base URL and API credentials
    pub fn new(base_url: &str, access_key: Option<String>, secret_key: Option<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            access_key,
            secret_key,
        }
    }

    /// Send GET request with JWT auth. Builds query string from params and signs with HMAC-SHA512
    pub async fn get(&self, path: &str, query: &[(&str, &str)]) -> Result<String, crate::error::AppError> {
        let access_key = self.access_key.as_deref().unwrap_or("");
        let secret_key = self.secret_key.as_deref().unwrap_or("");
        let qs = auth::query_string(query);
        let jwt = auth::create_jwt(access_key, secret_key, Some(&qs));
        let url = format!("{}{}", self.base_url, path);
        let resp = self.client.get(&url).bearer_auth(&jwt).query(query).send().await?;
        Ok(resp.text().await?)
    }

    /// Send POST request with JWT auth. Converts body to query string for HMAC-SHA512 signing
    pub async fn post(&self, path: &str, body: &serde_json::Value) -> Result<String, crate::error::AppError> {
        let access_key = self.access_key.as_deref().unwrap_or("");
        let secret_key = self.secret_key.as_deref().unwrap_or("");
        let qs = auth::body_to_query_string(body);
        let jwt = auth::create_jwt(access_key, secret_key, Some(&qs));
        let url = format!("{}{}", self.base_url, path);
        let resp = self.client.post(&url).bearer_auth(&jwt).json(body).send().await?;
        Ok(resp.text().await?)
    }

    /// Send DELETE request with JWT auth. Builds query string and signs with HMAC-SHA512
    pub async fn delete(&self, path: &str, query: &[(&str, &str)]) -> Result<String, crate::error::AppError> {
        let access_key = self.access_key.as_deref().unwrap_or("");
        let secret_key = self.secret_key.as_deref().unwrap_or("");
        let qs = auth::query_string(query);
        let jwt = auth::create_jwt(access_key, secret_key, Some(&qs));
        let url = format!("{}{}", self.base_url, path);
        let resp = self.client.delete(&url).bearer_auth(&jwt).query(query).send().await?;
        Ok(resp.text().await?)
    }
}
