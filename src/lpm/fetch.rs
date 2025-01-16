use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{anyhow, Context, Result};
use std::fs;

pub struct Fetcher {
    home_dir: PathBuf,
}

impl Fetcher {
    pub fn new() -> Result<Self> {
        let home = std::env::var("LAYMAN_HOME")
            .map(PathBuf::from)
            .or_else(|_| {
                std::env::var("HOME").map(|h| PathBuf::from(h).join(".layman"))
            })
            .context("Could not determine LAYMAN_HOME")?;
        
        Ok(Self { home_dir: home })
    }

    pub fn ensure_repo(&self, url: &str) -> Result<PathBuf> {
        let repos_dir = self.home_dir.join("repos");
        fs::create_dir_all(&repos_dir)?;

        // Simple hash of URL for directory name
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        url.hash(&mut hasher);
        let hash = format!("{:x}", hasher.finish());
        
        let repo_path = repos_dir.join(&hash);

        if !repo_path.exists() {
            println!("Cloning {}...", url);
            let status = Command::new("git")
                .arg("clone")
                .arg("--bare")
                .arg(url)
                .arg(&repo_path)
                .status()
                .context("Failed to execute git clone")?;

            if !status.success() {
                return Err(anyhow!("Git clone failed for {}", url));
            }
        } else {
            // Optional: fetch updates? For v1 we might assume if we have the repo we can try to find the commit
            // But if we don't find the commit later, we should fetch.
            // For now, let's just fetch to be safe if it exists.
             let status = Command::new("git")
                .current_dir(&repo_path)
                .arg("fetch")
                .arg("origin")
                .status()
                .context("Failed to execute git fetch")?;
             if !status.success() {
                 // Warn but continue? Or fail?
                 eprintln!("Warning: git fetch failed for cached repo {}", url);
             }
        }

        Ok(repo_path)
    }

    pub fn ensure_checkout(&self, repo_path: &Path, rev: &str) -> Result<PathBuf> {
        let checkouts_dir = self.home_dir.join("checkouts");
        fs::create_dir_all(&checkouts_dir)?;

        let repo_name = repo_path.file_name().unwrap().to_string_lossy();
        let checkout_path = checkouts_dir.join(format!("{}_{}", repo_name, rev));

        if !checkout_path.exists() {
            println!("Checking out {}...", rev);
            
            // Clone from the bare repo to the checkout path
            let status = Command::new("git")
                .arg("clone")
                .arg(repo_path)
                .arg(&checkout_path)
                .status()
                .context("Failed to clone from cache to checkout")?;

             if !status.success() {
                return Err(anyhow!("Failed to clone from cache to checkout"));
            }

            // Checkout specific revision
            let status = Command::new("git")
                .current_dir(&checkout_path)
                .arg("checkout")
                .arg(rev)
                .status()
                .context("Failed to checkout revision")?;

            if !status.success() {
                // Try fetching if checkout failed (maybe we didn't fetch enough in ensure_repo?)
                // But ensure_repo should have fetched.
                return Err(anyhow!("Failed to checkout revision {}", rev));
            }
        }

        Ok(checkout_path)
    }

    pub fn install_to_modules(&self, checkout_path: &Path, package_name: &str) -> Result<PathBuf> {
        let cwd = std::env::current_dir()?;
        let modules_dir = cwd.join("modules");
        fs::create_dir_all(&modules_dir)?;

        let target_path = modules_dir.join(package_name);
        
        if target_path.exists() {
            fs::remove_dir_all(&target_path)?;
        }

        // Copy from checkout to modules
        // For now, simple recursive copy. In future, maybe symlink if on unix?
        // Let's do copy for robustness across platforms and to avoid symlink issues with some tools.
        copy_dir_recursive(checkout_path, &target_path)?;

        Ok(target_path)
    }
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if ty.is_dir() {
            if entry.file_name() == ".git" {
                continue;
            }
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
