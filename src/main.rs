use std::fs;
use std::path::PathBuf;

use artifact_cleaner::{delete_all_artifact, find_dirs};
use clap::{Parser, ValueEnum};
use directories::UserDirs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    py: ProfileConfig,
}

#[derive(Deserialize, Debug)]
struct ProfileConfig {
    artifact_names: Vec<String>,
}

impl Config {
    fn new() -> Self {
        Self {
            py: ProfileConfig {
                artifact_names: vec![String::from("__pycache__")],
            },
        }
    }
}

/// Tool for cleaning artifacts of programming languages.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Root directory to start the search
    root: std::path::PathBuf,

    /// Cleaner profile. Depeding on the choise different directories can be configured
    #[arg(value_enum)]
    profile: Profile,

    /// If passed, the cleanable directories will be listed but not deleted
    #[arg(short, long)]
    dry_run: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Profile {
    Py,
}

fn get_config() -> Config {
    let user_dir = UserDirs::new();
    if let Some(dir) = user_dir {
        let config_data = fs::read_to_string(dir.home_dir().join(".artifact_cleaner.toml"));
        if let Ok(file) = config_data {
            return toml::from_str(&file).expect("Invalid toml config file");
        }
    }
    Config::new()
}

fn main() {
    let args = Args::parse();
    dbg!(&args);

    let config: Config = get_config();
    dbg!(&config);

    let mut findings: Vec<PathBuf> = Vec::new();

    match find_dirs(
        &mut findings,
        args.root.as_path(),
        &config.py.artifact_names,
        5,
    ) {
        Ok(()) => println!("Done"),
        Err(e) => println!("Error: {e:?}"),
    }
    dbg!(&findings);
    if !findings.is_empty() && !args.dry_run {
        delete_all_artifact(&findings).unwrap();
    }
}
