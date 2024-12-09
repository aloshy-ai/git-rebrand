use anyhow::{Context, Result};
use clap::{Arg, Command};
use git_rebrand::{setup_logger, Config, GitRebrander};
use log::debug;
use std::path::PathBuf;

// Version from Cargo.toml
const VERSION: &str = env!("CARGO_PKG_VERSION");
const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");

fn build_cli() -> Command {
    Command::new("git-rebrand")
        .version(VERSION)
        .about(ABOUT)
        .arg(
            Arg::new("path")
                .help("Path to the Git repository")
                .default_value(".")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose logging")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .help("Show what would be done, without making any changes")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-backup")
                .long("no-backup")
                .help("Skip creating backup branch (USE WITH CAUTION)")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Path to configuration file")
                .value_parser(clap::value_parser!(PathBuf)),
        )
}

fn main() -> Result<()> {
    // Set up logging first
    setup_logger().context("Failed to initialize logging")?;

    // Parse command line arguments
    let matches = build_cli().get_matches();

    // Get repository path
    let repo_path = matches
        .get_one::<PathBuf>("path")
        .cloned()
        .unwrap_or_else(|| PathBuf::from("."));

    debug!("Using repository path: {}", repo_path.display());

    // Handle verbose flag
    if matches.get_flag("verbose") {
        debug!("Verbose logging enabled");
    }

    // Load config either from file or through interactive prompts
    let mut config = if let Some(config_path) = matches.get_one::<PathBuf>("config") {
        Config::from_file(config_path).context("Failed to load configuration file")?
    } else {
        Config::prompt_interactive().context("Failed to get configuration from user")?
    };

    // Set repository path in config
    config.repo_path = repo_path;
    config.create_backup = !matches.get_flag("no-backup");

    // Create GitRebrander instance
    let rebrander = GitRebrander::new(config).context("Failed to initialize git-rebrand")?;

    // Execute based on dry-run flag
    if matches.get_flag("dry-run") {
        debug!("Performing dry run");
        rebrander.dry_run().context("Dry run failed")?;
    } else {
        debug!("Performing actual rewrite");
        rebrander.run().context("Rewrite operation failed")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        build_cli().debug_assert();
    }

    #[test]
    fn test_cli_with_args() {
        let matches =
            build_cli().get_matches_from(vec!["git-rebrand", "--dry-run", "/path/to/repo"]);

        assert!(matches.get_flag("dry-run"));
        assert_eq!(
            matches
                .get_one::<PathBuf>("path")
                .unwrap()
                .to_str()
                .unwrap(),
            "/path/to/repo"
        );
    }
}
