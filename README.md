# Weavex

[![Crates.io](https://img.shields.io/crates/v/weavex)](https://crates.io/crates/weavex)
[![Crates.io](https://img.shields.io/crates/d/weavex)](https://crates.io/crates/weavex)
[![CI](https://github.com/guitaripod/Weavex/actions/workflows/ci.yml/badge.svg)](https://github.com/guitaripod/Weavex/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

An autonomous AI research agent that combines Ollama's web search with your local LLMs. Watch as your model reasons through complex queries, autonomously searches the web, and synthesizes intelligent answers with citations.

## Features

- **Autonomous Agent** - Local LLM decides when to search, fetch, and synthesize
- **Reasoning Transparency** - See the model's chain-of-thought process in real-time
- **Smart Web Research** - Autonomous web search and page fetching with context
- **Fast & Efficient** - Built with Rust for maximum performance
- **Production Ready** - Comprehensive error handling and logging
- **Highly Configurable** - Multiple models, output formats, and options

## Installation

### From Source

```bash
git clone https://github.com/guitaripod/weavex.git
cd weavex
cargo install --path .
```

### From crates.io

```bash
cargo install weavex
```

## Prerequisites

You need an Ollama API key to use this tool. Get one at [ollama.com/settings/keys](https://ollama.com/settings/keys).

## Quick Start

### Set up your API key

```bash
export OLLAMA_API_KEY="your_api_key_here"
```

Or create a `.env` file:

```bash
echo "OLLAMA_API_KEY=your_api_key_here" > .env
```

### AI Agent Mode (Recommended)

Run autonomous research with your local Ollama models:

```bash
# Use default model (gpt-oss:20b) with reasoning enabled
weavex agent "What are the top 3 Rust developments from 2025?"

# Specify a different model
weavex agent --model qwen3:14b "research quantum computing trends"

# Hide thinking steps (show only final answer)
weavex agent --hide-thinking "query"

# Disable model reasoning mode
weavex agent --disable-reasoning "query"

# Custom Ollama server
weavex agent --ollama-url http://192.168.1.100:11434 "query"

# Limit agent iterations
weavex agent --max-iterations 5 "query"
```

**How it works:**
1. Agent uses your local Ollama model for reasoning
2. Model shows its chain-of-thought (🧠) before responding
3. Autonomously decides when to search the web or fetch URLs
4. Iterates until it has enough information
5. Synthesizes a comprehensive answer with sources

**Agent Output:**
- 🧠 **Reasoning**: Shows the model's thinking process
- 🔎 **Searching**: Web search operations
- 🌐 **Fetching**: URL fetch operations
- 💬 **Response**: Model's synthesized content
- 📝 **Final Answer**: Complete research summary

**Requirements:**
- Local Ollama server running (`ollama serve`)
- Model downloaded locally (`ollama pull gpt-oss:20b`)
- Ollama API key for web search access

**Recommended Models:**
- `gpt-oss:20b` - Best balance of speed and reasoning (default)
- `qwen3:14b` - Good tool-use capabilities
- `qwen3:4b` - Fastest, runs on laptops

<details>
<summary><h3>Direct API Access (Simple Mode)</h3></summary>

For quick searches without the agent, you can use the direct API mode:

### Basic Search

```bash
weavex "what is rust programming"
```

### Limit Results

```bash
weavex --max-results 5 "best practices for async rust"
```

### JSON Output

```bash
weavex --json "machine learning trends 2025"
```

### Fetch a Specific URL

```bash
weavex fetch https://example.com
```

### Advanced Options

```bash
# Pass API key via flag
weavex --api-key YOUR_KEY "query here"

# Verbose logging
weavex --verbose "debugging query"
```

</details>

## Options

<details>
<summary>Click to expand options</summary>

### Global Options
```
  -k, --api-key <API_KEY>          Ollama API key (can also use OLLAMA_API_KEY env var)
  -m, --max-results <NUM>          Maximum number of search results to return
  -j, --json                       Output results as JSON
  -v, --verbose                    Enable verbose logging
      --timeout <SECONDS>          Request timeout in seconds [default: 30]
  -h, --help                       Print help
  -V, --version                    Print version
```

### Commands
```
  fetch  Fetch and parse a specific URL
  agent  Run an AI agent with web search capabilities
  help   Print this message or the help of the given subcommand(s)
```

### Agent Options
```
  -m, --model <MODEL>              Local Ollama model to use [default: gpt-oss:20b]
      --ollama-url <URL>           Local Ollama server URL [default: http://localhost:11434]
      --max-iterations <NUM>       Maximum agent iterations [default: 10]
      --hide-thinking              Hide agent thinking steps (show only final result)
      --disable-reasoning          Disable model reasoning (thinking mode)
```

</details>

## Environment Variables

<details>
<summary>Click to expand environment variables</summary>

- `OLLAMA_API_KEY` - Your Ollama API key (required)
- `OLLAMA_BASE_URL` - Base URL for the API (default: `https://ollama.com/api`)
- `OLLAMA_TIMEOUT` - Request timeout in seconds (default: 30)

</details>

## Examples

### AI Agent Research

```bash
weavex agent "What are the latest benchmarks for Rust async runtimes?"
```

The agent will autonomously:
- Show its reasoning process (chain-of-thought)
- Search for relevant benchmark articles
- Fetch specific benchmark results
- Compare data from multiple sources
- Provide a synthesized summary with citations

### Clean Output Mode

Hide the reasoning steps and only show the final answer:

```bash
weavex agent --hide-thinking "What are the latest benchmarks for Rust async runtimes?"
```

### Traditional Mode (No Reasoning)

Disable reasoning mode for faster responses:

```bash
weavex agent --disable-reasoning "What are the latest benchmarks for Rust async runtimes?"
```

<details>
<summary><h3>Simple Mode Examples</h3></summary>

### Research a Topic

```bash
weavex "latest rust async runtime benchmarks"
```

### Compare Technologies

```bash
weavex --max-results 10 "tokio vs async-std performance"
```

### Extract Page Content

```bash
weavex fetch https://blog.rust-lang.org/
```

### Integrate with Other Tools

```bash
weavex --json "rust web frameworks" | jq '.results[0].url'
```

</details>

## Development

<details>
<summary>Click to expand development info</summary>

### Build

```bash
cargo build
```

### Run Tests

```bash
cargo test
```

### Release Build

```bash
cargo build --release
```

The release binary will be optimized with LTO and stripped of debug symbols.

</details>

## Project Structure

<details>
<summary>Click to expand project structure</summary>

```
src/
├── main.rs        - Application entry point and orchestration
├── agent.rs       - AI agent loop with tool execution
├── cli.rs         - CLI argument parsing with clap
├── client.rs      - Ollama web search API client
├── config.rs      - Configuration management
├── error.rs       - Custom error types with thiserror
├── formatter.rs   - Output formatting (human & JSON)
└── ollama_local.rs - Local Ollama chat API client
```

</details>

## Error Handling

<details>
<summary>Click to expand error handling details</summary>

The tool provides clear, actionable error messages:

- Missing API key → Instructions to set `OLLAMA_API_KEY`
- Network errors → Details about connection failures
- API errors → Status codes and error messages from Ollama
- Invalid responses → Clear parsing error descriptions

</details>

## Security

<details>
<summary>Click to expand security info</summary>

- API keys are never logged or printed
- `.env` files are gitignored by default
- Uses `rustls-tls` for secure HTTPS connections
- No hardcoded credentials or secrets

</details>

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

Built with:
- [clap](https://github.com/clap-rs/clap) - CLI argument parsing
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
- [tokio](https://github.com/tokio-rs/tokio) - Async runtime
- [serde](https://github.com/serde-rs/serde) - Serialization framework

Powered by [Ollama's Web Search API](https://ollama.com/blog/web-search).