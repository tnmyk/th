mod config;
mod llm;

use clap::Parser;
use config::Config;
use indicatif::{ProgressBar, ProgressStyle};
use inquire::Text;
use std::env;
use std::process::Command;
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
            println!(
                "Error: Configuration not found or invalid. Please run `th --setup` to configure your API key."
            );
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
        Ok(command) => match Text::new("Command:").with_initial_value(&command).prompt() {
            Ok(final_cmd) => {
                let final_cmd = final_cmd.trim();
                if final_cmd.is_empty() {
                    println!("Aborted.");
                    return;
                }

                let shell = env::var("SHELL").unwrap_or_else(|_| "sh".to_string());

                // Attempt to add to shell history
                if let Some(base_dirs) = directories::BaseDirs::new() {
                    let home = base_dirs.home_dir();
                    if shell.contains("zsh") {
                        let hist_file = env::var("HISTFILE").unwrap_or_else(|_| {
                            home.join(".zsh_history").to_string_lossy().to_string()
                        });
                        let timestamp = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs();
                        let hist_entry = format!(": {}:0;{}\n", timestamp, final_cmd);
                        if let Ok(mut file) = std::fs::OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(hist_file)
                        {
                            use std::io::Write;
                            let _ = file.write_all(hist_entry.as_bytes());
                        }
                    } else if shell.contains("bash") {
                        let hist_file = env::var("HISTFILE").unwrap_or_else(|_| {
                            home.join(".bash_history").to_string_lossy().to_string()
                        });
                        if let Ok(mut file) = std::fs::OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(hist_file)
                        {
                            use std::io::Write;
                            let _ = file.write_all(format!("{}\n", final_cmd).as_bytes());
                        }
                    }
                }

                let status = Command::new(&shell).arg("-c").arg(final_cmd).status();

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
        },
        Err(e) => {
            println!("\nFailed to get a command:\n  {}", e);
            std::process::exit(1);
        }
    }
}
