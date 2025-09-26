mod agent;
mod cli;
mod client;
mod config;
mod error;
mod formatter;
mod ollama_local;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, Command};
use client::OllamaClient;
use config::Config;
use formatter::{format_fetch_response, format_search_results};
use std::time::Duration;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    init_logging(cli.verbose);

    if let Err(e) = dotenvy::dotenv() {
        tracing::debug!("No .env file found: {}", e);
    }

    let api_key = cli
        .api_key
        .clone()
        .or_else(|| std::env::var("OLLAMA_API_KEY").ok())
        .context("API key not found. Set OLLAMA_API_KEY environment variable or use --api-key flag.\nGet your key at: https://ollama.com")?;

    let mut config = Config::new(api_key);

    if let Some(max_results) = cli.max_results {
        config = config.with_max_results(max_results);
    }

    config = config.with_timeout(Duration::from_secs(cli.timeout));

    let client = OllamaClient::new(config).context("Failed to create Ollama client")?;

    match cli.command {
        Some(Command::Fetch { url }) => {
            info!("Fetching URL: {}", url);
            let response = client.fetch(&url).await.context("Failed to fetch URL")?;

            let output = format_fetch_response(&response, cli.json);
            println!("{}", output);
        }
        Some(Command::Agent {
            query,
            model,
            ollama_url,
            max_iterations: _,
            hide_thinking,
            disable_reasoning,
        }) => {
            info!("Starting agent with model: {}", model);
            println!("🤖 Initializing agent with model: {}\n", model);

            let local_ollama = ollama_local::OllamaLocal::new(Some(ollama_url))
                .context("Failed to create local Ollama client")?;

            let agent = agent::Agent::new(local_ollama, client, model, !hide_thinking, !disable_reasoning);

            println!("🔍 Researching: {}\n", query);

            let result = agent
                .run(&query)
                .await
                .context("Agent execution failed")?;

            println!("\n📝 Final Answer:\n{}", result);
        }
        None => {
            let query = cli
                .get_query()
                .context("Query required. Use 'weavex <query>' or 'weavex --help' for usage information")?;

            info!("Searching for: {}", query);
            let response = client
                .search(query)
                .await
                .context("Search request failed")?;

            let output = format_search_results(&response, cli.json);
            println!("{}", output);
        }
    }

    Ok(())
}

fn init_logging(verbose: bool) {
    let filter = if verbose {
        EnvFilter::new("weavex=debug,info")
    } else {
        EnvFilter::new("weavex=warn")
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .without_time()
        .init();
}
