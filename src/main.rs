use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use artifact_cleaner::{delete_all_artifact, find_dirs};
use clap::{Args, Parser, Subcommand, ValueEnum};
use directories::UserDirs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    py: ProfileConfig,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a default config in you home directory
    Config,
    /// Run the artifact cleaning
    Run(RunArgs),
}

/// Tool for cleaning artifacts of programming languages.
#[derive(Args, Debug)]
struct RunArgs {
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

fn get_full_config_path() -> PathBuf {
    UserDirs::new()
        .expect("Could not retrieve user directory")
        .home_dir()
        .join(".artifact_cleaner.toml")
}

fn get_config() -> Config {
    match fs::read_to_string(get_full_config_path()) {
        Ok(file) => toml::from_str(&file).expect("Invalid toml config file"),
        Err(_) => Config::new(),
    }
}

fn create_config() -> io::Result<()> {
    let config_path = get_full_config_path();
    let mut file = fs::File::create(config_path)?;
    let deserialized_config = toml::to_string(&Config::new()).unwrap(); // Deal with this error
    file.write_all(deserialized_config.as_bytes())
}

fn run_cleaning(args: &RunArgs) -> () {
    dbg!(args);
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

fn run_config_init() -> () {
    match create_config() {
        Ok(_) => println!("Created default config"),
        Err(e) => println!("Default config could not be created: {e}"),
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Run(args) => run_cleaning(args),
        Commands::Config => run_config_init(),
    }
}
