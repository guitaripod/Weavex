use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use std::fs;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::html::{styled_line_to_highlighted_html, IncludeBackground};
use syntect::parsing::SyntaxSet;

pub fn open_markdown_in_browser(markdown_content: &str) -> Result<()> {
    let html = create_html_document(markdown_content);
    let html_size = html.len();

    const MAX_DATA_URL_SIZE: usize = 2_000_000;

    if html_size > MAX_DATA_URL_SIZE {
        tracing::debug!(
            "HTML size ({} bytes) exceeds data URL limit, using temp file fallback",
            html_size
        );
        open_html_via_temp_file(&html)
            .context("Failed to open HTML via temp file")
    } else {
        let encoded = STANDARD.encode(html.as_bytes());
        let data_url = format!("data:text/html;charset=utf-8;base64,{}", encoded);

        if data_url.len() > MAX_DATA_URL_SIZE {
            tracing::debug!(
                "Encoded data URL ({} bytes) exceeds limit, using temp file fallback",
                data_url.len()
            );
            open_html_via_temp_file(&html)
                .context("Failed to open HTML via temp file")
        } else {
            webbrowser::open(&data_url)
                .context("Failed to open browser with data URL")
        }
    }
}

fn open_html_via_temp_file(html: &str) -> Result<()> {
    let temp_dir = std::env::temp_dir();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let file_name = format!("weavex_result_{}.html", timestamp);
    let temp_path = temp_dir.join(file_name);

    fs::write(&temp_path, html)
        .context("Failed to write HTML to temp file")?;

    let url = format!("file://{}", temp_path.display());
    webbrowser::open(&url)
        .context("Failed to open browser with temp file")?;

    tracing::debug!("Opened HTML in browser from temp file: {:?}", temp_path);
    Ok(())
}

fn create_html_document(markdown_content: &str) -> String {
    let html_content = markdown_to_html(markdown_content);

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="color-scheme" content="light dark">
    <title>Weavex Result</title>
    <style>
        :root {{
            color-scheme: light dark;
        }}

        @media (prefers-color-scheme: dark) {{
            :root {{
                --bg: #0d1117;
                --bg-secondary: #161b22;
                --text: #c9d1d9;
                --text-secondary: #8b949e;
                --accent: #58a6ff;
                --border: #30363d;
                --border-light: #21262d;
            }}
        }}

        @media (prefers-color-scheme: light) {{
            :root {{
                --bg: #ffffff;
                --bg-secondary: #f6f8fa;
                --text: #24292f;
                --text-secondary: #57606a;
                --accent: #0969da;
                --border: #d0d7de;
                --border-light: #d8dee4;
            }}
        }}

        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Noto Sans', Helvetica, Arial, sans-serif;
            line-height: 1.6;
            max-width: 900px;
            margin: 0 auto;
            padding: 2rem;
            background: var(--bg);
            color: var(--text);
        }}

        h1, h2, h3, h4, h5, h6 {{
            margin-top: 24px;
            margin-bottom: 16px;
            font-weight: 600;
            line-height: 1.25;
            color: var(--accent);
        }}

        h1 {{ font-size: 2em; border-bottom: 1px solid var(--border-light); padding-bottom: 0.3em; }}
        h2 {{ font-size: 1.5em; border-bottom: 1px solid var(--border-light); padding-bottom: 0.3em; }}
        h3 {{ font-size: 1.25em; }}
        h4 {{ font-size: 1em; }}
        h5 {{ font-size: 0.875em; }}
        h6 {{ font-size: 0.85em; color: var(--text-secondary); }}

        p {{ margin-bottom: 16px; }}

        a {{
            color: var(--accent);
            text-decoration: none;
        }}

        a:hover {{
            text-decoration: underline;
        }}

        code {{
            background: var(--bg-secondary);
            padding: 0.2em 0.4em;
            border-radius: 6px;
            font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace;
            font-size: 85%;
        }}

        .code-block-wrapper {{
            position: relative;
            margin-bottom: 16px;
        }}

        .code-block-header {{
            background: var(--bg-secondary);
            padding: 8px 12px;
            border-radius: 6px 6px 0 0;
            border-bottom: 1px solid var(--border);
            display: flex;
            justify-content: space-between;
            align-items: center;
            font-size: 12px;
            color: var(--text-secondary);
        }}

        .code-lang {{
            font-weight: 600;
            text-transform: uppercase;
        }}

        .copy-button {{
            background: var(--bg);
            border: 1px solid var(--border);
            color: var(--text);
            padding: 4px 8px;
            border-radius: 4px;
            cursor: pointer;
            font-size: 11px;
            transition: all 0.2s;
        }}

        .copy-button:hover {{
            background: var(--border-light);
        }}

        .copy-button.copied {{
            background: #238636;
            color: white;
            border-color: #238636;
        }}

        pre {{
            background: var(--bg-secondary);
            padding: 16px;
            border-radius: 0 0 6px 6px;
            overflow-x: auto;
            line-height: 1.45;
            margin: 0;
        }}

        .code-block-wrapper.no-header pre {{
            border-radius: 6px;
        }}

        pre code {{
            background: none;
            padding: 0;
            display: block;
        }}

        ul, ol {{
            padding-left: 2em;
            margin-bottom: 16px;
        }}

        li {{
            margin-bottom: 0.25em;
        }}

        blockquote {{
            padding: 0 1em;
            color: var(--text-secondary);
            border-left: 0.25em solid var(--border);
            margin: 0 0 16px 0;
        }}

        table {{
            border-collapse: collapse;
            width: 100%;
            margin-bottom: 16px;
            display: block;
            overflow-x: auto;
        }}

        th, td {{
            border: 1px solid var(--border);
            padding: 6px 13px;
        }}

        th {{
            font-weight: 600;
            background: var(--bg-secondary);
        }}

        tr:nth-child(2n) {{
            background: var(--bg-secondary);
        }}

        hr {{
            height: 0.25em;
            padding: 0;
            margin: 24px 0;
            background-color: var(--border);
            border: 0;
        }}

        img {{
            max-width: 100%;
            height: auto;
            border-radius: 6px;
        }}

        .meta {{
            color: var(--text-secondary);
            font-size: 0.9em;
            margin-bottom: 2rem;
            padding-bottom: 1rem;
            border-bottom: 1px solid var(--border-light);
        }}
    </style>
</head>
<body>
    <div class="meta">🧵 Generated by Weavex</div>
    {}
    <script>
        function copyCode(button) {{
            const wrapper = button.closest('.code-block-wrapper');
            const code = wrapper.querySelector('pre code');
            const text = code.textContent;

            navigator.clipboard.writeText(text).then(() => {{
                button.textContent = 'Copied!';
                button.classList.add('copied');
                setTimeout(() => {{
                    button.textContent = 'Copy';
                    button.classList.remove('copied');
                }}, 2000);
            }});
        }}
    </script>
</body>
</html>"#,
        html_content
    )
}

fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);

    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];

    let mut in_code_block = false;
    let mut code_buffer = String::new();
    let mut code_lang = String::new();

    let events: Vec<Event> = parser.collect();
    let mut new_events = Vec::new();

    for event in events.iter() {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                code_buffer.clear();
                code_lang = match kind {
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                    CodeBlockKind::Indented => String::new(),
                };
            }
            Event::End(TagEnd::CodeBlock) => {
                if in_code_block {
                    let highlighted = highlight_code(&code_buffer, &code_lang, &ss, theme);

                    let wrapper = if code_lang.is_empty() {
                        format!(
                            r#"<div class="code-block-wrapper no-header"><pre><code>{}</code></pre></div>"#,
                            highlighted
                        )
                    } else {
                        format!(
                            r#"<div class="code-block-wrapper"><div class="code-block-header"><span class="code-lang">{}</span><button class="copy-button" onclick="copyCode(this)">Copy</button></div><pre><code>{}</code></pre></div>"#,
                            escape_html(&code_lang),
                            highlighted
                        )
                    };

                    new_events.push(Event::Html(wrapper.into()));
                    in_code_block = false;
                    code_buffer.clear();
                }
            }
            Event::Text(text) if in_code_block => {
                code_buffer.push_str(text);
            }
            _ => {
                if !in_code_block {
                    new_events.push(event.clone());
                }
            }
        }
    }

    let mut html_output = String::new();
    html::push_html(&mut html_output, new_events.into_iter());
    html_output
}

fn highlight_code(
    code: &str,
    lang: &str,
    ss: &SyntaxSet,
    theme: &syntect::highlighting::Theme,
) -> String {
    let syntax = ss
        .find_syntax_by_token(lang)
        .or_else(|| ss.find_syntax_by_extension(lang))
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    let mut highlighter = HighlightLines::new(syntax, theme);
    let mut result = String::new();

    for line in code.lines() {
        let ranges = highlighter.highlight_line(line, ss).unwrap_or_default();
        let html = styled_line_to_highlighted_html(&ranges[..], IncludeBackground::No)
            .unwrap_or_else(|_| escape_html(line));
        result.push_str(&html);
        result.push('\n');
    }

    if result.ends_with('\n') {
        result.pop();
    }

    result
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("<script>"), "&lt;script&gt;");
        assert_eq!(escape_html("a & b"), "a &amp; b");
    }

    #[test]
    fn test_markdown_tables() {
        let md = r#"
| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |
"#;
        let html = markdown_to_html(md);
        assert!(html.contains("<table"));
        assert!(html.contains("<th"));
        assert!(html.contains("<td"));
    }

    #[test]
    fn test_markdown_code_blocks() {
        let md = r#"
```rust
fn main() {
    println!("Hello, world!");
}
```
"#;
        let html = markdown_to_html(md);
        assert!(html.contains("code-block-wrapper"));
        assert!(html.contains("rust"));
    }
}
