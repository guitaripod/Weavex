use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "weavex",
    version,
    about = "Weave together web search and AI reasoning for autonomous research",
    long_about = "Weave combines Ollama's web search API with local LLMs for intelligent, autonomous research.\n\n\
                  Requires an API key from https://ollama.com - set via OLLAMA_API_KEY environment variable or --api-key flag.",
    after_help = "EXAMPLES:\n    \
                  weavex \"what is rust programming\"\n    \
                  weavex --max-results 5 \"best practices for async rust\"\n    \
                  weavex --json \"machine learning trends 2025\"\n    \
                  weavex fetch https://example.com\n    \
                  weavex agent \"what are the latest rust async runtime benchmarks\"\n    \
                  weavex agent --model qwen3:4b \"research topic\"\n    \
                  weavex --api-key YOUR_KEY \"query here\""
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(
        value_name = "QUERY",
        help = "Search query (when not using a subcommand)"
    )]
    pub query: Option<String>,

    #[arg(
        short = 'k',
        long,
        env = "OLLAMA_API_KEY",
        help = "Ollama API key (can also use OLLAMA_API_KEY env var)"
    )]
    pub api_key: Option<String>,

    #[arg(
        short = 'm',
        long,
        value_name = "NUM",
        help = "Maximum number of search results to return"
    )]
    pub max_results: Option<usize>,

    #[arg(short = 'j', long, help = "Output results as JSON")]
    pub json: bool,

    #[arg(short = 'v', long, help = "Enable verbose logging")]
    pub verbose: bool,

    #[arg(
        long,
        value_name = "SECONDS",
        default_value = "30",
        help = "Request timeout in seconds"
    )]
    pub timeout: u64,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Fetch and parse a specific URL")]
    Fetch {
        #[arg(value_name = "URL", help = "URL to fetch")]
        url: String,
    },
    #[command(about = "Run an AI agent with web search capabilities")]
    Agent {
        #[arg(value_name = "QUERY", help = "Question or task for the agent")]
        query: String,

        #[arg(
            short = 'm',
            long,
            value_name = "MODEL",
            default_value = "gpt-oss:20b",
            help = "Local Ollama model to use"
        )]
        model: String,

        #[arg(
            long,
            value_name = "URL",
            default_value = "http://localhost:11434",
            help = "Local Ollama server URL"
        )]
        ollama_url: String,

        #[arg(
            long,
            value_name = "NUM",
            default_value = "10",
            help = "Maximum agent iterations"
        )]
        max_iterations: usize,

        #[arg(
            long,
            help = "Hide agent thinking steps (show only final result)"
        )]
        hide_thinking: bool,
    },
}

impl Cli {
    pub fn get_query(&self) -> Option<&str> {
        self.query.as_deref()
    }
}
