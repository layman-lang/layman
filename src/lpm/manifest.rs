use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub package: Package,
    #[serde(default)]
    pub dependencies: BTreeMap<String, Dependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub authors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Dependency {
    // Simple version string (not supported in v1 but good to have structure for)
    Version(String),
    // Detailed dependency specification
    Detailed(DependencyDetail),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DependencyDetail {
    pub git: Option<String>,
    pub rev: Option<String>,
    pub path: Option<String>,
    pub tag: Option<String>,
}

impl Manifest {
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read manifest at {}", path.display()))?;
        let manifest: Manifest = toml::from_str(&content)
            .with_context(|| format!("Failed to parse manifest at {}", path.display()))?;
        Ok(manifest)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)
            .with_context(|| format!("Failed to write manifest to {}", path.display()))?;
        Ok(())
    }
}
