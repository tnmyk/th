use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub gemini_api_key: String,
}

impl Config {
    fn get_config_dir() -> Option<PathBuf> {
        // This will resolve to something like:
        // Linux:   ~/.config/th
        // Windows: C:\Users\Alice\AppData\Roaming\th\config
        // macOS:   ~/Library/Application Support/th
        if let Some(proj_dirs) = ProjectDirs::from("", "", "th") {
            Some(proj_dirs.config_dir().to_path_buf())
        } else {
            None
        }
    }

    pub fn get_config_path() -> Option<PathBuf> {
        Self::get_config_dir().map(|dir| dir.join("config.toml"))
    }

    pub fn load() -> Option<Self> {
        let path = Self::get_config_path()?;
        if path.exists() {
            let contents = fs::read_to_string(path).ok()?;
            toml::from_str(&contents).ok()
        } else {
            None
        }
    }

    pub fn setup() {
        println!("=== Terminal Helper Setup ===");
        println!("Please enter your Gemini API Key:");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut api_key = String::new();
        io::stdin().read_line(&mut api_key).expect("Failed to read line");
        let api_key = api_key.trim().to_string();

        if api_key.is_empty() {
            println!("API key cannot be empty. Setup aborted.");
            return;
        }

        let config = Config { gemini_api_key: api_key };

        let config_dir = match Self::get_config_dir() {
            Some(dir) => dir,
            None => {
                println!("Error: Could not determine configuration directory for your OS.");
                return;
            }
        };

        if let Err(e) = fs::create_dir_all(&config_dir) {
            println!("Error: Failed to create config directory: {}", e);
            return;
        }

        let config_path = config_dir.join("config.toml");
        let toml_string = toml::to_string(&config).expect("Failed to serialize config");

        if let Err(e) = fs::write(&config_path, toml_string) {
            println!("Error: Failed to write config file: {}", e);
            return;
        }

        println!("\nSuccess! Configuration saved to: {}", config_path.display());
    }
}
