mod config;
mod llm;

use clap::Parser;
use config::Config;

#[derive(Parser, Debug)]
#[command(name = "th", version, about, long_about = None)]
struct Cli {
    #[arg(name = "QUERY")]
    query: Option<String>,

    #[arg(long)]
    setup: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.setup {
        Config::setup();
        return;
    }

    let query = match cli.query {
        Some(q) => q,
        None => {
            println!("Please provide a query or run `th --help` for usage.");
            std::process::exit(1);
        }
    };

    let config = match Config::load() {
        Some(c) => c,
        None => {
            println!("Error: Configuration not found or invalid. Please run `th --setup` to configure your API key.");
            std::process::exit(1);
        }
    };

    println!("Analyzing query: '{}'...", query);
    
    match llm::get_command(&config.gemini_api_key, &query).await {
        Ok(command) => {
            println!("\nSuggested command:");
            println!("  {}\n", command);
            // We'll add [R]un, [C]opy, [E]dit in Phase 5
        }
        Err(e) => {
            println!("\nFailed to get a command:\n  {}", e);
            std::process::exit(1);
        }
    }
}
