use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lockfile {
    pub schema_version: u32,
    pub root: PackageId,
    pub packages: Vec<PackageEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PackageId {
    pub name: String,
    pub source: String,
    pub rev: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageEntry {
    pub id: PackageId,
    pub dependencies: Vec<PackageId>,
    #[serde(default)]
    pub checksum: Option<String>,
}

impl Lockfile {
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read lockfile at {}", path.display()))?;
        let lockfile: Lockfile = toml::from_str(&content)
            .with_context(|| format!("Failed to parse lockfile at {}", path.display()))?;
        Ok(lockfile)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)
            .with_context(|| format!("Failed to write lockfile to {}", path.display()))?;
        Ok(())
    }
}
