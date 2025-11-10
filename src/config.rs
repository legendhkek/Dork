use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub threads: usize,
    pub timeout: u64,
    pub user_agent: String,
    pub retry_attempts: usize,
    pub verbose: bool,
    pub proxy: Option<String>,
    pub rate_limit: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            threads: 10,
            timeout: 30,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            retry_attempts: 3,
            verbose: false,
            proxy: None,
            rate_limit: 100,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = "config.json";
        if std::path::Path::new(config_path).exists() {
            let contents = fs::read_to_string(config_path)?;
            let config: Config = serde_json::from_str(&contents)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_str = serde_json::to_string_pretty(self)?;
        fs::write("config.json", config_str)?;
        Ok(())
    }
}
