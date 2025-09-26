use crate::config::Config;
use crate::error::{OllamaError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};

#[derive(Debug, Serialize)]
struct SearchRequest {
    query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_results: Option<usize>,
}

#[derive(Debug, Serialize)]
struct FetchRequest {
    url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FetchResponse {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub links: Vec<String>,
}

pub struct OllamaClient {
    client: Client,
    config: Config,
}

impl OllamaClient {
    pub fn new(config: Config) -> Result<Self> {
        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(OllamaError::RequestFailed)?;

        Ok(Self { client, config })
    }

    #[instrument(skip(self))]
    pub async fn search(&self, query: &str) -> Result<SearchResponse> {
        let url = format!("{}/web_search", self.config.base_url);

        debug!("Sending search request to: {}", url);

        let request = SearchRequest {
            query: query.to_string(),
            max_results: self.config.max_results,
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OllamaError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let search_response = response.json::<SearchResponse>().await.map_err(|e| {
            OllamaError::InvalidResponse(format!("Failed to parse search response: {}", e))
        })?;

        Ok(search_response)
    }

    #[instrument(skip(self))]
    pub async fn fetch(&self, target_url: &str) -> Result<FetchResponse> {
        let url = format!("{}/web_fetch", self.config.base_url);

        debug!("Sending fetch request to: {}", url);

        let request = FetchRequest {
            url: target_url.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(OllamaError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let fetch_response = response.json::<FetchResponse>().await.map_err(|e| {
            OllamaError::InvalidResponse(format!("Failed to parse fetch response: {}", e))
        })?;

        Ok(fetch_response)
    }
}
