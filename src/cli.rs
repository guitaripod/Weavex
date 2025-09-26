use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "weavex",
    version,
    about = "Weave together web search and AI reasoning for autonomous research",
    long_about = "Weave combines Ollama's web search API with local LLMs for intelligent, autonomous research.\n\n\
                  Requires an API key from https://ollama.com - set via OLLAMA_API_KEY environment variable or --api-key flag.",
    after_help = "EXAMPLES:\n    \
                  # Basic search\n    \
                  weavex \"what is rust programming\"\n    \n\
                  # Limit results\n    \
                  weavex --max-results 5 \"best practices for async rust\"\n    \n\
                  # JSON output\n    \
                  weavex --json \"machine learning trends 2025\"\n    \n\
                  # Fetch a URL\n    \
                  weavex fetch https://example.com\n    \n\
                  # AI agent (default shows only final answer with loading animation)\n    \
                  weavex agent \"what are the latest rust async runtime benchmarks\"\n    \n\
                  # Use different model\n    \
                  weavex agent --model qwen3:14b \"research topic\"\n    \n\
                  # Show thinking steps and reasoning process\n    \
                  weavex agent --show-thinking \"query\"\n    \n\
                  # Disable reasoning mode\n    \
                  weavex agent --disable-reasoning \"query\"\n    \n\
                  # Custom API key\n    \
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

    #[arg(
        long,
        help = "Output result as a clickable data URL for browser preview"
    )]
    pub preview: bool,

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
            default_value = "50",
            help = "Maximum agent iterations"
        )]
        max_iterations: usize,

        #[arg(
            long,
            help = "Show agent thinking steps and reasoning process. \n\
                    By default, only the final answer is displayed with a loading animation. \n\
                    Use this flag to see the model's reasoning (🧠), \n\
                    tool calls (🔎 🌐), and responses (💬) for transparency."
        )]
        show_thinking: bool,

        #[arg(
            long,
            help = "Disable model reasoning (thinking mode). \n\
                    By default, reasoning is enabled to show the model's \n\
                    chain-of-thought process. Use this flag for faster responses."
        )]
        disable_reasoning: bool,

        #[arg(
            long,
            help = "Output result as a clickable data URL for browser preview"
        )]
        preview: bool,
    },
}

impl Cli {
    pub fn get_query(&self) -> Option<&str> {
        self.query.as_deref()
    }
}
