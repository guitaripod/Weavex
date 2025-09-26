use crate::error::{OllamaError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};

#[derive(Debug, Serialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ToolMessage {
    pub role: String,
    pub content: String,
    pub tool_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolCall {
    pub function: FunctionCall,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: serde_json::Value,
}

#[derive(Debug, Serialize, Clone)]
pub struct Tool {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: ToolFunction,
}

#[derive(Debug, Serialize, Clone)]
pub struct ToolFunction {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    think: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub message: ChatMessage,
    pub done: bool,
}

#[derive(Debug, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    #[serde(default)]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(default)]
    pub thinking: Option<String>,
}

pub struct OllamaLocal {
    client: Client,
    base_url: String,
}

impl OllamaLocal {
    pub fn new(base_url: Option<String>) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .map_err(OllamaError::RequestFailed)?;

        Ok(Self {
            client,
            base_url: base_url.unwrap_or_else(|| "http://localhost:11434".to_string()),
        })
    }

    #[instrument(skip(self, messages, tools))]
    pub async fn chat(
        &self,
        model: &str,
        messages: Vec<serde_json::Value>,
        tools: Option<Vec<Tool>>,
        think: bool,
    ) -> Result<ChatResponse> {
        let url = format!("{}/api/chat", self.base_url);

        debug!("Sending chat request to local Ollama at: {}", url);

        let request = ChatRequest {
            model: model.to_string(),
            messages,
            tools,
            stream: false,
            think: if think { Some(true) } else { None },
        };

        let response = self
            .client
            .post(&url)
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

        let chat_response = response.json::<ChatResponse>().await.map_err(|e| {
            OllamaError::InvalidResponse(format!("Failed to parse chat response: {}", e))
        })?;

        Ok(chat_response)
    }
}

pub fn create_web_search_tool() -> Tool {
    Tool {
        tool_type: "function".to_string(),
        function: ToolFunction {
            name: "web_search".to_string(),
            description: "Search the web for information. Returns a list of search results with titles, URLs, and content snippets.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The search query to execute"
                    },
                    "max_results": {
                        "type": "integer",
                        "description": "Maximum number of results to return (optional)"
                    }
                },
                "required": ["query"]
            }),
        },
    }
}

pub fn create_web_fetch_tool() -> Tool {
    Tool {
        tool_type: "function".to_string(),
        function: ToolFunction {
            name: "web_fetch".to_string(),
            description: "Fetch and parse content from a specific URL. Returns the page title, content, and links.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "The URL to fetch and parse"
                    }
                },
                "required": ["url"]
            }),
        },
    }
}