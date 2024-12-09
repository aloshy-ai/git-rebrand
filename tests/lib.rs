use anyhow::Result;
use git2::Repository;
use git_rebrand::{Config, GitRebrandError, GitRebrander};
use tempfile::TempDir;

mod common;
use common::TestRepo;

#[test]
fn test_empty_repository() -> Result<()> {
    let temp = TempDir::new()?;
    Repository::init(&temp)?;

    let config = Config {
        new_author_name: "New Author".to_string(),
        new_author_email: "new@example.com".to_string(),
        patterns: vec!["old@example.com".to_string()],
        repo_path: temp.path().to_path_buf(),
        create_backup: true,
    };

    let result = GitRebrander::new(config);
    assert!(matches!(
        result.unwrap_err().downcast::<GitRebrandError>().unwrap(),
        GitRebrandError::InvalidRepository(_)
    ));
    Ok(())
}

mod configuration {
    use super::*;
    use std::fs;

    #[test]
    fn test_valid_config_file() -> Result<()> {
        let temp = TempDir::new()?;
        let config_path = temp.path().join("config.yml");

        let config_content = r#"
            new_author_name: "New Author"
            new_author_email: "new@example.com"
            patterns:
              - "old@example.com"
              - "Old Name"
        "#;
        fs::write(&config_path, config_content)?;

        let config = Config::from_file(&config_path)?;
        assert_eq!(config.new_author_name, "New Author");
        assert_eq!(config.new_author_email, "new@example.com");
        assert_eq!(config.patterns.len(), 2);
        Ok(())
    }

    #[test]
    fn test_invalid_email_in_config() -> Result<()> {
        let temp = TempDir::new()?;
        let config_path = temp.path().join("config.yml");

        let config_content = r#"
            new_author_name: "New Author"
            new_author_email: "invalid-email"
            patterns:
              - "old@example.com"
        "#;
        fs::write(&config_path, config_content)?;

        let result = Config::from_file(&config_path);
        assert!(result.is_err());
        Ok(())
    }
}

mod pattern_matching {
    use super::*;

    #[test]
    fn test_exact_email_match() -> Result<()> {
        let repo = TestRepo::new()?;
        repo.commit_with_author("Test commit", "Old Author", "old@example.com")?;

        let config = Config {
            new_author_name: "New Author".to_string(),
            new_author_email: "new@example.com".to_string(),
            patterns: vec!["old@example.com".to_string()],
            repo_path: repo.path().to_path_buf(),
            create_backup: true,
        };

        let rebrander = GitRebrander::new(config)?;
        let matches = rebrander.dry_run()?;
        assert_eq!(matches.len(), 1);
        assert!(matches[0].author.contains("old@example.com"));
        Ok(())
    }

    #[test]
    fn test_email_domain_match() -> Result<()> {
        let repo = TestRepo::new()?;
        repo.commit_with_author("Test commit", "Old Author", "test@oldcompany.com")?;

        let config = Config {
            new_author_name: "New Author".to_string(),
            new_author_email: "new@example.com".to_string(),
            patterns: vec!["@oldcompany.com".to_string()],
            repo_path: repo.path().to_path_buf(),
            create_backup: true,
        };

        let rebrander = GitRebrander::new(config)?;
        let matches = rebrander.dry_run()?;
        assert_eq!(matches.len(), 1);
        Ok(())
    }

    #[test]
    fn test_case_insensitive_name_match() -> Result<()> {
        let repo = TestRepo::new()?;
        repo.commit_with_author("Test commit", "Old Author Name", "test@example.com")?;

        let config = Config {
            new_author_name: "New Author".to_string(),
            new_author_email: "new@example.com".to_string(),
            patterns: vec!["old author".to_string()],
            repo_path: repo.path().to_path_buf(),
            create_backup: true,
        };

        let rebrander = GitRebrander::new(config)?;
        let matches = rebrander.dry_run()?;
        assert_eq!(matches.len(), 1);
        Ok(())
    }
}

mod history_rewriting {
    use super::*;

    #[test]
    fn test_successful_rewrite() -> Result<()> {
        let repo = TestRepo::new()?;
        repo.commit_with_author("Test commit", "Old Author", "old@example.com")?;

        let config = Config {
            new_author_name: "New Author".to_string(),
            new_author_email: "new@example.com".to_string(),
            patterns: vec!["old@example.com".to_string()],
            repo_path: repo.path().to_path_buf(),
            create_backup: true,
        };

        let rebrander = GitRebrander::new(config)?;
        rebrander.run()?;

        // Verify the changes
        let git_repo = Repository::open(repo.path())?;
        let head_commit = git_repo.head()?.peel_to_commit()?;
        assert_eq!(head_commit.author().name(), Some("New Author"));
        assert_eq!(head_commit.author().email(), Some("new@example.com"));
        Ok(())
    }

    #[test]
    fn test_backup_branch_creation() -> Result<()> {
        let repo = TestRepo::new()?;
        repo.commit_with_author("Test commit", "Old Author", "old@example.com")?;

        let config = Config {
            new_author_name: "New Author".to_string(),
            new_author_email: "new@example.com".to_string(),
            patterns: vec!["old@example.com".to_string()],
            repo_path: repo.path().to_path_buf(),
            create_backup: true,
        };

        let rebrander = GitRebrander::new(config)?;
        rebrander.run()?;

        // Verify backup branch exists
        let git_repo = Repository::open(repo.path())?;
        let branches: Vec<_> = git_repo
            .branches(None)?
            .map(|b| b.unwrap().0.name().unwrap().unwrap().to_string())
            .collect();

        assert!(branches.iter().any(|name| name.starts_with("backup_")));
        Ok(())
    }

    #[test]
    fn test_no_backup_when_disabled() -> Result<()> {
        let repo = TestRepo::new()?;
        repo.commit_with_author("Test commit", "Old Author", "old@example.com")?;

        let config = Config {
            new_author_name: "New Author".to_string(),
            new_author_email: "new@example.com".to_string(),
            patterns: vec!["old@example.com".to_string()],
            repo_path: repo.path().to_path_buf(),
            create_backup: false,
        };

        let rebrander = GitRebrander::new(config)?;
        rebrander.run()?;

        let git_repo = Repository::open(repo.path())?;
        let branches: Vec<_> = git_repo
            .branches(None)?
            .map(|b| b.unwrap().0.name().unwrap().unwrap().to_string())
            .collect();

        assert!(!branches.iter().any(|name| name.starts_with("backup_")));
        Ok(())
    }
}

mod error_handling {
    use super::*;

    #[test]
    fn test_no_matching_commits() -> Result<()> {
        let repo = TestRepo::new()?;
        repo.commit_with_author("Test commit", "Different Author", "different@example.com")?;

        let config = Config {
            new_author_name: "New Author".to_string(),
            new_author_email: "new@example.com".to_string(),
            patterns: vec!["nonexistent@example.com".to_string()],
            repo_path: repo.path().to_path_buf(),
            create_backup: true,
        };

        let rebrander = GitRebrander::new(config)?;
        let result = rebrander.dry_run();
        assert!(matches!(
            result.unwrap_err().downcast::<GitRebrandError>().unwrap(),
            GitRebrandError::NoMatchingCommits
        ));
        Ok(())
    }

    #[test]
    fn test_uncommitted_changes() -> Result<()> {
        let repo = TestRepo::new()?;
        repo.create_file("test.txt", "content")?;

        let config = Config {
            new_author_name: "New Author".to_string(),
            new_author_email: "new@example.com".to_string(),
            patterns: vec!["test@example.com".to_string()],
            repo_path: repo.path().to_path_buf(),
            create_backup: true,
        };

        let result = GitRebrander::new(config);
        assert!(matches!(
            result.unwrap_err().downcast::<GitRebrandError>().unwrap(),
            GitRebrandError::InvalidRepository(_)
        ));
        Ok(())
    }
}
