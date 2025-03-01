use dirs;
use serde::{Deserialize, Serialize};
use toml;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub diary_db_path: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self, anyhow::Error> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("dia");
        
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }

        let config_file = config_dir.join("config.toml");
        if !config_file.exists() {
            let default_config = Config {
                diary_db_path: config_dir.join("diary.db"),
            };
            let toml = toml::to_string(&default_config)?;
            fs::write(&config_file, toml)?;
            return Ok(default_config);
        }

        let config_str = fs::read_to_string(config_file)?;
        Ok(toml::from_str(&config_str)?)
    }
}
