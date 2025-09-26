use crate::client::OllamaClient;
use crate::error::Result;
use crate::ollama_local::{create_web_fetch_tool, create_web_search_tool, OllamaLocal, ToolCall};
use serde_json::json;
use tracing::{info, warn};

pub struct Agent {
    local_ollama: OllamaLocal,
    web_client: OllamaClient,
    model: String,
    max_iterations: usize,
    show_thinking: bool,
    enable_reasoning: bool,
}

impl Agent {
    pub fn new(
        local_ollama: OllamaLocal,
        web_client: OllamaClient,
        model: String,
        show_thinking: bool,
        enable_reasoning: bool,
    ) -> Self {
        Self {
            local_ollama,
            web_client,
            model,
            max_iterations: 10,
            show_thinking,
            enable_reasoning,
        }
    }

    pub async fn run(&self, user_query: &str) -> Result<String> {
        let tools = vec![create_web_search_tool(), create_web_fetch_tool()];

        let mut messages = vec![json!({
            "role": "user",
            "content": user_query
        })];

        info!("Starting agent loop with query: {}", user_query);

        for iteration in 0..self.max_iterations {
            info!("Agent iteration {}/{}", iteration + 1, self.max_iterations);

            let response = self
                .local_ollama
                .chat(&self.model, messages.clone(), Some(tools.clone()), self.enable_reasoning)
                .await?;

            if let Some(thinking) = &response.message.thinking {
                if !thinking.is_empty() && self.show_thinking {
                    info!("Model thinking: {}", &thinking[..thinking.len().min(100)]);
                    println!("\n🧠 Reasoning:");
                    println!("   {}", thinking.replace("\n", "\n   "));
                }
            }

            let content = &response.message.content;
            if !content.is_empty() {
                info!("Model response: {}", &content[..content.len().min(100)]);
                if self.show_thinking {
                    println!("\n💬 Response:");
                    println!("   {}", content.replace("\n", "\n   "));
                }
            }

            messages.push(json!({
                "role": "assistant",
                "content": response.message.content,
                "tool_calls": response.message.tool_calls
            }));

            if let Some(tool_calls) = response.message.tool_calls {
                info!("Model requested {} tool call(s)", tool_calls.len());

                for tool_call in tool_calls {
                    if self.show_thinking {
                        match tool_call.function.name.as_str() {
                            "web_search" => {
                                let query = tool_call.function.arguments["query"].as_str().unwrap_or("");
                                println!("   🔎 Searching: {}...", query);
                            }
                            "web_fetch" => {
                                let url = tool_call.function.arguments["url"].as_str().unwrap_or("");
                                println!("   🌐 Fetching: {}...", url);
                            }
                            _ => {}
                        }
                    }
                    let result = self.execute_tool(&tool_call).await?;

                    let truncated_result = if result.len() > 8000 {
                        format!("{}... [truncated]", &result[..8000])
                    } else {
                        result.clone()
                    };

                    info!(
                        "Tool {} executed, result length: {} chars",
                        tool_call.function.name,
                        result.len()
                    );

                    messages.push(json!({
                        "role": "tool",
                        "content": truncated_result,
                        "tool_name": tool_call.function.name
                    }));
                }
            } else {
                info!("Agent completed without tool calls");
                return Ok(response.message.content);
            }
        }

        warn!("Agent reached max iterations ({})", self.max_iterations);
        Ok("Maximum iterations reached. Please try rephrasing your query.".to_string())
    }

    async fn execute_tool(&self, tool_call: &ToolCall) -> Result<String> {
        match tool_call.function.name.as_str() {
            "web_search" => {
                let query = tool_call.function.arguments["query"]
                    .as_str()
                    .unwrap_or("");

                let max_results = tool_call.function.arguments
                    .get("max_results")
                    .and_then(|v| v.as_u64())
                    .map(|v| v as usize);

                info!("Executing web_search: query='{}', max_results={:?}", query, max_results);

                let response = self.web_client.search(query).await?;

                let mut result = String::new();
                for (idx, search_result) in response.results.iter().enumerate() {
                    let truncated_content = truncate_utf8(&search_result.content, 500);
                    result.push_str(&format!(
                        "Result {}:\nTitle: {}\nURL: {}\nContent: {}\n\n",
                        idx + 1,
                        search_result.title,
                        search_result.url,
                        truncated_content
                    ));
                }

                Ok(result)
            }
            "web_fetch" => {
                let url = tool_call.function.arguments["url"]
                    .as_str()
                    .unwrap_or("");

                info!("Executing web_fetch: url='{}'", url);

                let response = self.web_client.fetch(url).await?;

                let truncated_content = truncate_utf8(&response.content, 2000);
                Ok(format!(
                    "Title: {}\n\nContent:\n{}\n\nLinks found: {}",
                    response.title,
                    truncated_content,
                    response.links.len()
                ))
            }
            _ => {
                warn!("Unknown tool: {}", tool_call.function.name);
                Ok(format!("Error: Unknown tool '{}'", tool_call.function.name))
            }
        }
    }
}

fn truncate_utf8(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes {
        return s;
    }

    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}