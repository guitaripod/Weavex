use crate::error::{OllamaError, Result};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub base_url: String,
    pub timeout: Duration,
    pub max_results: Option<usize>,
}

impl Config {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://ollama.com/api".to_string(),
            timeout: Duration::from_secs(30),
            max_results: None,
        }
    }

    pub fn with_base_url(mut self, url: String) -> Self {
        self.base_url = url;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_max_results(mut self, max_results: usize) -> Self {
        self.max_results = Some(max_results);
        self
    }

    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("OLLAMA_API_KEY").map_err(|_| OllamaError::MissingApiKey)?;

        let mut config = Self::new(api_key);

        if let Ok(url) = std::env::var("OLLAMA_BASE_URL") {
            config = config.with_base_url(url);
        }

        if let Ok(timeout_str) = std::env::var("OLLAMA_TIMEOUT") {
            if let Ok(timeout_secs) = timeout_str.parse::<u64>() {
                config = config.with_timeout(Duration::from_secs(timeout_secs));
            }
        }

        Ok(config)
    }
}
