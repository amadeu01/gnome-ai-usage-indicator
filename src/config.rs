use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "default_poll_interval")]
    pub poll_interval_secs: u64,
    #[serde(default = "default_enabled_providers")]
    pub enabled_providers: Vec<String>,
    #[serde(default = "default_ollama_host")]
    pub ollama_host: String,
    #[serde(default)]
    pub anthropic_api_key: String,
}

fn default_poll_interval() -> u64 {
    60
}

fn default_enabled_providers() -> Vec<String> {
    vec!["claude-code".to_string()]
}

fn default_ollama_host() -> String {
    "http://localhost:11434".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            poll_interval_secs: default_poll_interval(),
            enabled_providers: default_enabled_providers(),
            ollama_host: default_ollama_host(),
            anthropic_api_key: String::new(),
        }
    }
}

fn config_path() -> PathBuf {
    let mut p = home_dir();
    p.push(".config/ai-usage-indicator/config.toml");
    p
}

pub fn home_dir() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
}

impl Config {
    pub fn load() -> Self {
        let path = config_path();
        match std::fs::read_to_string(&path) {
            Ok(contents) => match toml::from_str::<Config>(&contents) {
                Ok(cfg) => cfg,
                Err(e) => {
                    eprintln!("Config parse error (using defaults): {e}");
                    Config::default()
                }
            },
            Err(_) => {
                let cfg = Config::default();
                if let Err(e) = cfg.save() {
                    eprintln!("Could not write default config: {e}");
                }
                cfg
            }
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }
}
