mod config;
mod llm;

use clap::Parser;
use config::Config;
use inquire::Text;
use indicatif::{ProgressBar, ProgressStyle};
use std::process::Command;
use std::env;
use std::time::Duration;

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

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.set_message("Thinking...");
    pb.enable_steady_tick(Duration::from_millis(100));
    
    let cmd_result = llm::get_command(&config.gemini_api_key, &query).await;
    
    pb.finish_and_clear();

    match cmd_result {
        Ok(command) => {
            match Text::new("Command:")
                .with_initial_value(&command)
                .prompt()
            {
                Ok(final_cmd) => {
                    let final_cmd = final_cmd.trim();
                    if final_cmd.is_empty() {
                        println!("Aborted.");
                        return;
                    }
                    
                    let shell = env::var("SHELL").unwrap_or_else(|_| "sh".to_string());
                    let status = Command::new(&shell)
                        .arg("-c")
                        .arg(final_cmd)
                        .status();
                    
                    match status {
                        Ok(s) => {
                            if !s.success() {
                                println!("\nProcess exited with: {}", s);
                            }
                        }
                        Err(e) => println!("\nFailed to execute process: {}", e),
                    }
                }
                Err(_) => {
                    println!("Aborted.");
                }
            }
        }
        Err(e) => {
            println!("\nFailed to get a command:\n  {}", e);
            std::process::exit(1);
        }
    }
}
