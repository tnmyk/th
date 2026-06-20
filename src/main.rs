use clap::Parser;

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
        println!("Setup mode: To be implemented in Phase 2");
        return;
    }

    match cli.query {
        Some(query) => {
            println!("Query received: {}", query);
            println!("LLM processing: To be implemented in Phase 3");
        }
        None => {
            println!("Please provide a query or run `th --help` for usage.");
        }
    }
}
