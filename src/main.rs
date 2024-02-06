use clap::{Parser, ValueEnum};

/// Tool for cleaning artifacts of programming languages
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    root: std::path::PathBuf,

    #[arg(value_enum)]
    profile: Profile,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Profile {
    Py,
}

fn main() {
    let args = Args::parse();
    println!("pattern: {:?}, path: {:?}", args.root, args.profile)
}
