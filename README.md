# Artifact Cleaner

A CLI tool for cleaning artifact directories.

## Running the tool

```
Usage: artifact_cleaner [OPTIONS] <COMMAND>

Commands:
  config  Create a default config in you home directory
  run     Run the artifact cleaning
  help    Print this message or the help of the given subcommand(s)

Options:
      --log-level <LOG_LEVEL>  Set the level for the logger [default: Info]
  -h, --help                   Print help
  -V, --version                Print version
```

### Subcommand: run

Run the cleaning. If a configation file `.artifact_cleaner.toml` is present in the users home directory it is used. Otherwise a default configuation is used. 

```
Run the artifact cleaning

Usage: artifact_cleaner run [OPTIONS] <ROOT> <PROFILE>

Arguments:
  <ROOT>     Root directory to start the search
  <PROFILE>  Cleaner profile. Depeding on the choise different directories can be configured [possible values: py, rust, user]

Options:
      --dry-run                If passed, the cleanable directories will be listed but not deleted
      --max-depth <MAX_DEPTH>  Maximum depth from the root the tool will look for artifacts [default: 10]
  -h, --help                   Print help
  -V, --version                Print version
```

#### Default config:

```toml
ignore_directories = [".git", ".github"]

[py]
artifact_directories = ["__pycache__", ".mypy_cache", ".ruff_cache", "dist"]
ignore_directories = []

[rust]
artifact_directories = ["target"]
ignore_directories = []

[user]
artifact_directories = []
ignore_directories = []
```


### Subcommand: config

Create a toml file with the default configuration in the home directory of the used. This version can be 
modified for personal preference and is then loaded in the run command.

```
Create a default config in you home directory

Usage: artifact_cleaner config

Options:
  -h, --help     Print help
  -V, --version  Print version
```
