use anyhow::{Context, Result};
use chrono::Local;
use dialoguer::{theme::ColorfulTheme, Input};
use git2::{Repository, Signature};
use log::{debug, error, info, warn};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};
use thiserror::Error;

// Error definitions
#[derive(Error, Debug)]
pub enum GitRebrandError {
    #[error("Invalid repository path: {0}")]
    InvalidRepository(String),

    #[error("Invalid author pattern: {0}")]
    InvalidPattern(String),

    #[error("No commits matched the provided patterns")]
    NoMatchingCommits,

    #[error("Failed to create backup: {0}")]
    BackupFailed(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

// Commit information structure
#[derive(Debug, Clone)]
pub struct CommitInfo {
    pub id: String,
    pub author: String,
    pub timestamp: String,
    pub matched_pattern: String,
}

// Configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub new_author_name: String,
    pub new_author_email: String,
    pub patterns: Vec<String>,
    #[serde(default)]
    pub repo_path: PathBuf,
    #[serde(default = "default_backup")]
    pub create_backup: bool,
}

fn default_backup() -> bool {
    true
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path).context("Failed to read configuration file")?;
        let config: Self =
            serde_yaml::from_str(&content).context("Failed to parse configuration file")?;

        // Validate email format
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .context("Failed to compile email regex")?;

        if !email_regex.is_match(&config.new_author_email) {
            return Err(GitRebrandError::InvalidConfig(format!(
                "Invalid email format: {}",
                config.new_author_email
            ))
            .into());
        }

        Ok(config)
    }

    pub fn prompt_interactive() -> Result<Self> {
        let theme = ColorfulTheme::default();

        // Prompt for new author information
        let new_author_name = Input::<String>::with_theme(&theme)
            .with_prompt("New author name")
            .interact_text()
            .context("Failed to get author name")?;

        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .context("Failed to compile email regex")?;

        let new_author_email = loop {
            let email = Input::<String>::with_theme(&theme)
                .with_prompt("New author email")
                .interact_text()
                .context("Failed to get author email")?;

            if email_regex.is_match(&email) {
                break email;
            }
            warn!("Invalid email format. Please try again.");
        };

        // Prompt for patterns
        println!("\nEnter patterns to match (one per line)");
        println!("Examples:");
        println!("  john.doe@example.com    (matches exact email)");
        println!("  @oldomain.com          (matches domain)");
        println!("  John Doe               (matches exact name)");
        println!("  John                   (matches partial name)");
        println!("Press Ctrl+D (or Ctrl+Z on Windows) when finished:");

        let mut patterns = Vec::new();
        let stdin = std::io::stdin();
        let mut input = String::new();
        while stdin.read_line(&mut input)? > 0 {
            let pattern = input.trim().to_string();
            if !pattern.is_empty() {
                patterns.push(pattern);
            }
            input.clear();
        }

        if patterns.is_empty() {
            return Err(GitRebrandError::InvalidPattern(
                "At least one pattern must be provided".to_string(),
            )
            .into());
        }

        Ok(Config {
            new_author_name,
            new_author_email,
            patterns,
            repo_path: PathBuf::new(),
            create_backup: true,
        })
    }
}

pub struct GitRebrander {
    repo: Repository,
    config: Config,
}

impl std::fmt::Debug for GitRebrander {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GitRebrander")
            .field("repo", &"<Repository>")
            .finish()
    }
}

impl GitRebrander {
    pub fn new(config: Config) -> Result<Self> {
        let repo = Repository::open(&config.repo_path).context("Failed to open git repository")?;
        let rebrander = Self { repo, config };
        // Validate repository immediately upon creation
        rebrander.validate_repository()?;
        Ok(rebrander)
    }

    fn validate_repository(&self) -> Result<()> {
        // Check if repository is empty
        if self.repo.is_empty()? {
            return Err(
                GitRebrandError::InvalidRepository("Repository is empty".to_string()).into(),
            );
        }

        // Check for bare repository
        if self.repo.is_bare() {
            return Err(GitRebrandError::InvalidRepository(
                "Bare repositories are not supported".to_string(),
            )
            .into());
        }

        // Check for uncommitted changes
        let statuses = self.repo.statuses(None)?;
        if statuses.iter().any(|s| s.status() != git2::Status::CURRENT) {
            return Err(GitRebrandError::InvalidRepository(
                "Repository has uncommitted changes".to_string(),
            )
            .into());
        }

        Ok(())
    }

    pub fn dry_run(&self) -> Result<Vec<CommitInfo>> {
        info!("Starting dry run analysis");
        self.validate_repository()?;

        let mut affected_commits = Vec::new();
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(git2::Sort::TIME)?;

        for oid in revwalk {
            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;
            let author = commit.author();

            if let Some(pattern) =
                self.matches_pattern(author.name().unwrap_or(""), author.email().unwrap_or(""))
            {
                affected_commits.push(CommitInfo {
                    id: oid.to_string()[..8].to_string(),
                    author: format!(
                        "{} <{}>",
                        author.name().unwrap_or(""),
                        author.email().unwrap_or("")
                    ),
                    timestamp: commit.time().seconds().to_string(),
                    matched_pattern: pattern,
                });
            }
        }

        if affected_commits.is_empty() {
            return Err(GitRebrandError::NoMatchingCommits.into());
        }

        info!(
            "Dry run complete. Found {} affected commits",
            affected_commits.len()
        );
        Ok(affected_commits)
    }

    pub fn run(&self) -> Result<()> {
        // First do a dry run to validate everything
        let affected_commits = self.dry_run()?;

        // Create backup if needed
        if self.config.create_backup {
            self.create_backup().context("Failed to create backup")?;
        }

        // Perform the rewrite
        self.rewrite_history(&affected_commits)
            .context("Failed to rewrite history")?;

        info!("Successfully rewrote {} commits", affected_commits.len());
        Ok(())
    }

    fn create_backup(&self) -> Result<()> {
        let head = self.repo.head()?;
        let backup_name = format!("backup_{}", Local::now().format("%Y%m%d%H%M%S"));

        debug!("Creating backup branch: {}", backup_name);
        self.repo
            .branch(&backup_name, &head.peel_to_commit()?, false)?;

        info!("Created backup branch: {}", backup_name);
        Ok(())
    }

    fn matches_pattern(&self, author: &str, email: &str) -> Option<String> {
        let author_lower = author.to_lowercase();
        let email_lower = email.to_lowercase();

        for pattern in &self.config.patterns {
            let pattern_lower = pattern.to_lowercase();
            if pattern_lower.contains('@') {
                if email_lower.contains(&pattern_lower) {
                    return Some(pattern.clone());
                }
            } else if author_lower.contains(&pattern_lower) {
                return Some(pattern.clone());
            }
        }
        None
    }

    fn rewrite_history(&self, commits: &[CommitInfo]) -> Result<()> {
        info!("Starting history rewrite");
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::REVERSE)?;

        let commit_ids: HashSet<String> = commits.iter().map(|c| c.id.clone()).collect();
        let mut last_rewritten_id = None;

        for oid in revwalk {
            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;
            let commit_short_id = oid.to_string()[..8].to_string();

            if commit_ids.contains(&commit_short_id) {
                debug!("Rewriting commit: {}", commit_short_id);

                let new_author =
                    Signature::now(&self.config.new_author_name, &self.config.new_author_email)?;
                let tree = commit.tree()?;
                let parents: Vec<_> = commit
                    .parent_ids()
                    .map(|id| self.repo.find_commit(id))
                    .collect::<Result<Vec<_>, _>>()?;
                let parent_refs: Vec<_> = parents.iter().collect();

                let new_id = self.repo.commit(
                    None,
                    &new_author,
                    &new_author,
                    commit.message().unwrap_or(""),
                    &tree,
                    &parent_refs,
                )?;
                last_rewritten_id = Some(new_id);
            }
        }

        if let Some(new_id) = last_rewritten_id {
            let obj = self.repo.find_object(new_id, None)?;
            self.repo.reset(&obj, git2::ResetType::Hard, None)?;
        }

        info!("History rewrite complete");
        Ok(())
    }
}

// Logging setup
pub fn setup_logger() -> Result<()> {
    let env = env_logger::Env::default()
        .filter_or("GIT_REBRAND_LOG", "info")
        .write_style_or("GIT_REBRAND_LOG_STYLE", "auto");

    env_logger::Builder::from_env(env)
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .format_target(false)
        .format_module_path(true)
        .init();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_logger_initialization() {
        // Save current env vars
        let old_log = env::var("GIT_REBRAND_LOG").ok();
        let old_style = env::var("GIT_REBRAND_LOG_STYLE").ok();

        // Test with custom values
        env::set_var("GIT_REBRAND_LOG", "debug");
        env::set_var("GIT_REBRAND_LOG_STYLE", "never");

        assert!(setup_logger().is_ok());

        // Restore env vars
        if let Some(val) = old_log {
            env::set_var("GIT_REBRAND_LOG", val);
        } else {
            env::remove_var("GIT_REBRAND_LOG");
        }

        if let Some(val) = old_style {
            env::set_var("GIT_REBRAND_LOG_STYLE", val);
        } else {
            env::remove_var("GIT_REBRAND_LOG_STYLE");
        }
    }
}
