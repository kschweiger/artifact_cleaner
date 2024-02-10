//! # Artifact Cleaner
//!
//! Linrary containing the logic for the artifact cleaner cli toold
use directories::UserDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use tracing::info;

pub mod cleaning;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub py: ProfileConfig,
    pub ignore: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileConfig {
    pub artifact_names: Vec<String>,
    pub ignore: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            py: ProfileConfig {
                artifact_names: vec![String::from("__pycache__")],
                ignore: vec![],
            },
            ignore: vec![String::from(".git")],
        }
    }
}

pub fn get_full_config_path() -> PathBuf {
    UserDirs::new()
        .expect("Could not retrieve user directory")
        .home_dir()
        .join(".artifact_cleaner.toml")
}

pub fn get_config(path: PathBuf) -> Config {
    match fs::read_to_string(path) {
        Ok(file) => toml::from_str(&file).expect("Invalid toml config file"),
        Err(_) => Config::new(),
    }
}

pub fn create_config(config_path: PathBuf) -> io::Result<()> {
    let mut file = fs::File::create(&config_path)?;
    let deserialized_config = toml::to_string(&Config::new()); // Deal with this error
    match deserialized_config {
        Ok(cfg) => file.write_all(cfg.as_bytes())?,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };
    info!("Created new default config at {:?}", config_path);
    Ok(())
}
