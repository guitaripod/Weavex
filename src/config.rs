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

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_max_results(mut self, max_results: usize) -> Self {
        self.max_results = Some(max_results);
        self
    }
}
