use crate::client::{FetchResponse, SearchResponse};

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
            format!("{}...", truncate_utf8(&result.content, 200))
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
            truncate_utf8(&response.content, 1000)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_utf8_ascii() {
        let text = "Hello, World!";
        assert_eq!(truncate_utf8(text, 5), "Hello");
        assert_eq!(truncate_utf8(text, 100), text);
    }

    #[test]
    fn test_truncate_utf8_emoji() {
        let text = "Hello 👋 World 🌍";
        let result = truncate_utf8(text, 10);
        assert!(result.len() <= 10);
        assert!(result.is_char_boundary(result.len()));
    }

    #[test]
    fn test_truncate_utf8_chinese() {
        let text = "你好世界";
        let result = truncate_utf8(text, 6);
        assert!(result.len() <= 6);
        assert!(result.is_char_boundary(result.len()));
    }

    #[test]
    fn test_truncate_utf8_at_boundary() {
        let text = "🚀";
        let result = truncate_utf8(text, 2);
        assert_eq!(result, "");
    }

    #[test]
    fn test_truncate_utf8_empty() {
        let text = "";
        assert_eq!(truncate_utf8(text, 10), "");
    }
}
