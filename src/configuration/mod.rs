use std::fs;
use std::path::PathBuf;

use config::{Config, ConfigError};
use directories::ProjectDirs;

use crate::logger::{info, warn, err};

const CONFIG_FILE: &str = "config.toml";

pub fn config_path() -> Result<PathBuf, ConfigError> {
    let proj_dirs = ProjectDirs::from("com", "Rimaki", "YourApp")
        .ok_or(ConfigError::Message("Failed to get config directory".into()))?;

    let dir = proj_dirs.config_dir();

    // Ensure directory exists
    if let Err(e) = fs::create_dir_all(dir) {
        return Err(ConfigError::Message(format!(
            "Failed to create config directory: {e}"
        )));
    }

    Ok(dir.join(CONFIG_FILE))
}

pub fn check_file(path: &PathBuf) {
    info("Config", "Checking configuration received\nProcessing...");

    if !path.exists() {
        warn("Config", "Config file does not exist", 1);
    }
}

pub fn init() -> Result<Config, ConfigError> {
    info("Config", "Config initialization requested");

    let path = config_path()?;
    check_file(&path);

    let settings = Config::builder()
        .add_source(config::File::from(path.clone()).required(false))
        .build();

    match settings {
        Ok(cfg) => {
            info("Config", "Config loaded successfully");
            Ok(cfg)
        }
        Err(e) => {
            err("Config", &format!("Failed to load config: {e}"), 1);
            Err(e)
        }
    }
}