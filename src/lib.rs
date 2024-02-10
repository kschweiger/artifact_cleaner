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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    pub py: ProfileConfig,
    pub ignore: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn new_config_serializable() {
        let cfg = Config::new();
        let result = toml::to_string(&cfg);
        assert!(!result.is_err());
    }

    #[test]
    fn create_new_config() {
        let temp_dir = tempdir().expect("...");
        let dir_path = temp_dir.path();
        let config_name = dir_path.join(".artifact_cleaner.toml");
        let result = create_config(config_name.to_path_buf());
        assert!(!result.is_err());

        let data = fs::read_to_string(config_name);
        assert!(!data.is_err());
    }

    #[test]
    fn get_config_err_returns_new() {
        let temp_dir = tempdir().expect("...");
        let dir_path = temp_dir.path();
        let config_name = dir_path.join(".artifact_cleaner.toml");
        let result = get_config(config_name.clone());
        assert_eq!(result, Config::new());

        // Make sure the file does really not exists
        let data = fs::read_to_string(config_name);
        assert!(data.is_err());
    }

    #[test]
    fn get_config_from_file() {
        let temp_dir = tempdir().expect("...");
        let dir_path = temp_dir.path();
        let config_name = dir_path.join(".artifact_cleaner.toml");

        let mut config = Config::new();
        config.ignore.push(String::from("some_new_value"));

        assert_ne!(config, Config::new());
        let mut file = fs::File::create(&config_name).unwrap();
        file.write_all(toml::to_string(&config).unwrap().as_bytes())
            .unwrap();

        let result = get_config(config_name);
        assert_ne!(result, Config::new());
        assert_eq!(config, result);
    }
}
