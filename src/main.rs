use std::path::PathBuf;

use artifact_cleaner::cleaning::{delete_all_artifact, find_dirs};
use artifact_cleaner::{create_config, get_config, get_full_config_path, Config};
use clap::{Args, Parser, Subcommand, ValueEnum};

use tracing::{debug, error, info, Level};
use tracing_subscriber::FmtSubscriber;

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

fn run_cleaning(args: &RunArgs) -> () {
    info!("Running cleaning in profile {:?}", args.profile);
    info!("Starting from root: {:?}", args.root);
    let config: Config = get_config(get_full_config_path());
    debug!("{:#?}", &config);

    let profile = match args.profile {
        Profile::Py => config.py,
    };

    let mut findings: Vec<PathBuf> = Vec::new();
    let mut ignore = Vec::new();
    ignore.extend(config.ignore);
    ignore.extend(profile.ignore);

    match find_dirs(
        &mut findings,
        args.root.as_path(),
        &profile.artifact_names,
        &ignore,
        5,
    ) {
        Ok(()) => info!("Search completet"),
        Err(e) => error!("Error: {e:?}"),
    }
    if !findings.is_empty() {
        if args.dry_run {
            info!("dry-run set. Found {:#?}", findings);
        } else {
            delete_all_artifact(&findings).unwrap();
        }
    }
}

fn run_config_init() -> () {
    match create_config(get_full_config_path()) {
        Ok(()) => (),
        Err(e) => error!("Default config could not be created: {e}"),
    }
}

fn main() {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let cli = Cli::parse();
    match &cli.command {
        Commands::Run(args) => run_cleaning(args),
        Commands::Config => run_config_init(),
    }
}
