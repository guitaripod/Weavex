# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Weavex is an autonomous research agent combining Ollama's web search API with local LLM models. Built in Rust for AI-powered web research with clean output and optional reasoning transparency.

## Bash Commands

```bash
# Build and test
cargo build
cargo test --verbose
cargo build --release

# Code quality (run after changes)
cargo fmt
cargo clippy -- -D warnings

# Run locally
cargo run -- "query here"
cargo run -- agent "query here"  # requires local Ollama server
cargo run -- fetch https://example.com
```

## Architecture

**Core flow:** `main.rs` routes to three modes via clap CLI:
1. Default search → `client.rs` (Ollama web API)
2. `fetch` command → `client.rs` (URL parsing)
3. `agent` command → `agent.rs` (autonomous loop with local LLM)

**Agent workflow (`agent.rs`):**
- Local Ollama model decides which tools to call autonomously
- Tools: `web_search` and `web_fetch` (defined in `ollama_local.rs`)
- Iterates up to 50 times by default, truncates results to 8000 chars for context management
- Reasoning mode enabled by default (`think: true` parameter)
- Default mode: opens final result in browser with markdown rendering (🌐)
- `--no-preview` flag: disables browser preview, shows terminal output instead
- `--show-thinking` flag: displays reasoning (🧠), tool calls (🔎 🌐), and responses (💬)

**Key quirk:** Agent's thinking output in `ChatMessage.thinking` field, only displayed when `--show-thinking` is used

## Environment Variables

REQUIRED: `OLLAMA_API_KEY` (get from ollama.com/settings/keys)

## Releasing

IMPORTANT: Create tags without "v" prefix (e.g., `1.0.5`, not `v1.0.5`). The CI workflow expects semver format `[0-9]+.[0-9]+.[0-9]+`.