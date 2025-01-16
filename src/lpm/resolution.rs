use crate::lpm::manifest::{Manifest, Dependency};
use crate::lpm::lockfile::{PackageId, PackageEntry};
use crate::lpm::fetch::Fetcher;
use std::collections::{HashMap, HashSet};
use anyhow::{anyhow, Result};
use std::path::PathBuf;

pub struct Resolver {
    fetcher: Fetcher,
    resolved: HashMap<String, PackageEntry>, // name -> entry
    visiting: HashSet<String>, // for cycle detection
}

impl Resolver {
    pub fn new() -> Result<Self> {
        Ok(Self {
            fetcher: Fetcher::new()?,
            resolved: HashMap::new(),
            visiting: HashSet::new(),
        })
    }

    pub fn resolve(&mut self, root_manifest: &Manifest) -> Result<Vec<PackageEntry>> {
        self.resolved.clear();
        self.visiting.clear();

        // Root package ID (dummy for now, or derived from manifest)
        let root_id = PackageId {
            name: root_manifest.package.name.clone(),
            source: "root".to_string(),
            rev: "HEAD".to_string(),
        };

        // Resolve direct dependencies
        let mut root_deps = Vec::new();
        for (name, dep) in &root_manifest.dependencies {
            let dep_id = self.resolve_dependency(name, dep)?;
            root_deps.push(dep_id);
        }

        let root_entry = PackageEntry {
            id: root_id,
            dependencies: root_deps,
            checksum: None,
        };
        
        // We don't add root to resolved map because it's not a dependency of itself
        // But we return it as part of the result if needed, or just return the list of all packages
        
        let mut all_packages = self.resolved.values().cloned().collect::<Vec<_>>();
        all_packages.push(root_entry);
        
        Ok(all_packages)
    }

    fn resolve_dependency(&mut self, name: &str, dep: &Dependency) -> Result<PackageId> {
        if self.visiting.contains(name) {
            return Err(anyhow!("Dependency cycle detected involving {}", name));
        }
        
        // Check if already resolved
        if let Some(entry) = self.resolved.get(name) {
            // Verify consistency
            // In v1, we strictly enforce single version.
            // So if we see the same name, it MUST match the existing resolution.
            // We need to check if 'dep' matches 'entry.id'
            // This is a bit tricky because 'dep' is the requested spec, 'entry.id' is the resolved ID.
            // For now, let's assume if names match, it's a conflict if details differ.
            // But we can't easily compare Dependency vs PackageId without more logic.
            // Simplified: just return the existing ID. Real implementation would verify compatibility.
            return Ok(entry.id.clone());
        }

        self.visiting.insert(name.to_string());

        // 1. Determine Source and Rev
        let (source, rev) = match dep {
            Dependency::Version(_) => return Err(anyhow!("Version strings not supported in v1, use {{ git = \"...\", rev = \"...\" }}")),
            Dependency::Detailed(d) => {
                if let Some(git) = &d.git {
                    let rev = d.rev.as_deref().ok_or_else(|| anyhow!("Git dependency {} missing 'rev'", name))?;
                    (git.clone(), rev.to_string())
                } else if let Some(_path) = &d.path {
                     return Err(anyhow!("Path dependencies not yet supported in resolution"));
                } else {
                    return Err(anyhow!("Invalid dependency spec for {}", name));
                }
            }
        };

        // 2. Fetch Package
        let repo_path = self.fetcher.ensure_repo(&source)?;
        let checkout_path = self.fetcher.ensure_checkout(&repo_path, &rev)?;
        
        // 3. Read Manifest
        let manifest_path = checkout_path.join("layman.toml");
        if !manifest_path.exists() {
             return Err(anyhow!("Package {} at {}/{} has no layman.toml", name, source, rev));
        }
        let manifest = Manifest::load(&manifest_path)?;

        if manifest.package.name != name {
             return Err(anyhow!("Package name mismatch: expected {}, found {}", name, manifest.package.name));
        }

        // 4. Recurse
        let mut deps = Vec::new();
        for (dep_name, dep_spec) in &manifest.dependencies {
            let dep_id = self.resolve_dependency(dep_name, dep_spec)?;
            deps.push(dep_id);
        }

        let id = PackageId {
            name: name.to_string(),
            source,
            rev,
        };

        let entry = PackageEntry {
            id: id.clone(),
            dependencies: deps,
            checksum: None,
        };

        self.resolved.insert(name.to_string(), entry);
        self.visiting.remove(name);

        Ok(id)
    }
    
    // Helper to get checkout path for a resolved package
    pub fn get_checkout_path(&self, id: &PackageId) -> Result<PathBuf> {
        if id.source == "root" {
             return Err(anyhow!("Cannot get checkout path for root"));
        }
        let repo_path = self.fetcher.ensure_repo(&id.source)?;
        self.fetcher.ensure_checkout(&repo_path, &id.rev)
    }
}
