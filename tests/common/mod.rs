use anyhow::Result;
use git2::{Repository, Signature};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

pub struct TestRepo {
    temp_dir: TempDir,
    repo: Repository,
}

impl TestRepo {
    pub fn new() -> Result<Self> {
        let temp_dir = TempDir::new()?;
        let repo = Repository::init(&temp_dir)?;
        
        // Create initial commit
        let sig = Signature::now("Test User", "test@example.com")?;
        let tree_id = {
            let mut index = repo.index()?;
            index.write_tree()?
        };
        
        // Move this block into its own scope so the tree borrow is dropped
        {
            let tree = repo.find_tree(tree_id)?;
            repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;
        }

        Ok(Self { temp_dir, repo })
    }

    pub fn path(&self) -> &Path {
        self.temp_dir.path()
    }

    pub fn create_file(&self, name: &str, content: &str) -> Result<PathBuf> {
        let path = self.temp_dir.path().join(name);
        std::fs::write(&path, content)?;
        Ok(path)
    }

    pub fn commit_with_author(
        &self,
        message: &str,
        name: &str,
        email: &str
    ) -> Result<()> {
        let sig = Signature::now(name, email)?;
        
        // Create a file to commit
        let file_path = self.create_file(
            &format!("file_{}.txt", chrono::Utc::now().timestamp()),
            "test content"
        )?;
        
        // Add file to index
        let mut index = self.repo.index()?;
        index.add_path(file_path.strip_prefix(self.path())?)?;
        let tree_id = index.write_tree()?;
        index.write()?;
        
        let tree = self.repo.find_tree(tree_id)?;
        let parent = self.repo.head()?.peel_to_commit()?;
        
        self.repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            message,
            &tree,
            &[&parent]
        )?;
        
        Ok(())
    }
}