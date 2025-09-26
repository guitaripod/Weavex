use crate::client::{FetchResponse, SearchResponse};
use serde_json;

pub fn format_search_results(response: &SearchResponse, as_json: bool) -> String {
    if as_json {
        return serde_json::to_string_pretty(response).unwrap_or_else(|_| "{}".to_string());
    }

    let mut output = String::new();

    if response.results.is_empty() {
        output.push_str("No results found.\n");
        return output;
    }

    output.push_str(&format!("Found {} results:\n\n", response.results.len()));

    for (idx, result) in response.results.iter().enumerate() {
        output.push_str(&format!("{}. {}\n", idx + 1, result.title));
        output.push_str(&format!("   {}\n", result.url));

        let content_preview = if result.content.len() > 200 {
            format!("{}...", &result.content[..200])
        } else {
            result.content.clone()
        };

        output.push_str(&format!("   {}\n\n", content_preview));
    }

    output
}

pub fn format_fetch_response(response: &FetchResponse, as_json: bool) -> String {
    if as_json {
        return serde_json::to_string_pretty(response).unwrap_or_else(|_| "{}".to_string());
    }

    let mut output = String::new();

    output.push_str(&format!("Title: {}\n\n", response.title));

    let content_preview = if response.content.len() > 1000 {
        format!(
            "{}...\n\n[Content truncated. Use --json for full content]",
            &response.content[..1000]
        )
    } else {
        response.content.clone()
    };

    output.push_str(&format!("Content:\n{}\n\n", content_preview));

    if !response.links.is_empty() {
        output.push_str(&format!("Found {} links:\n", response.links.len()));
        for (idx, link) in response.links.iter().take(10).enumerate() {
            output.push_str(&format!("  {}. {}\n", idx + 1, link));
        }
        if response.links.len() > 10 {
            output.push_str(&format!("  ... and {} more\n", response.links.len() - 10));
        }
    }

    output
}
