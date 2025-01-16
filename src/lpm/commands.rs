use clap::{Subcommand, Args};
use std::path::Path;
use anyhow::{anyhow, Result};
use crate::lpm::manifest::{Manifest, Package, Dependency, DependencyDetail};
use crate::lpm::resolution::Resolver;
use crate::lpm::lockfile::Lockfile;
use crate::lpm::fetch::Fetcher;
use std::collections::BTreeMap;

// We don't need PkgCommands enum anymore as we are flattening into main.rs
// But we can keep the logic functions.

pub fn init(path: Option<String>, name: Option<String>) -> Result<()> {
    let cwd = std::env::current_dir()?;
    
    // Determine target directory
    let target_dir = if let Some(p) = path {
        let p = Path::new(&p);
        if !p.exists() {
            std::fs::create_dir_all(p)?;
            println!("Created directory {}", p.display());
        }
        cwd.join(p)
    } else {
        cwd.clone()
    };

    let manifest_path = target_dir.join("layman.toml");

    if manifest_path.exists() {
        return Err(anyhow!("layman.toml already exists in {}", target_dir.display()));
    }

    let package_name = name.or_else(|| {
        target_dir.file_name()
            .map(|n| n.to_string_lossy().to_string())
    }).unwrap_or_else(|| "unnamed".to_string());

    let manifest = Manifest {
        package: Package {
            name: package_name,
            version: "0.1.0".to_string(),
            description: None,
            authors: vec![],
        },
        dependencies: BTreeMap::new(),
    };

    manifest.save(&manifest_path)?;
    println!("Created layman.toml in {}", target_dir.display());

    // Scaffold src/main.lay
    let src_dir = target_dir.join("src");
    if !src_dir.exists() {
        std::fs::create_dir(&src_dir)?;
    }

    let main_lay_path = src_dir.join("main.lay");
    if !main_lay_path.exists() {
        let content = include_str!("templates/main.lay");
        std::fs::write(&main_lay_path, content)?;
        println!("Created src/main.lay");
    }

    Ok(())
}

pub fn install(package_spec: Option<String>) -> Result<()> {
    // If package_spec is None, just sync (install all deps)
    if package_spec.is_none() {
        return sync();
    }
    
    let spec = package_spec.unwrap();
    
    // Parse spec: "user/repo" or "https://..."
    // We want to support "layman install user/repo" -> git = https://github.com/user/repo, rev = main (default?)
    
    let (name, git_url, rev) = if spec.contains("://") {
        // Full URL
        // derive name from url
        let name = spec.split('/').last()
            .map(|s| s.trim_end_matches(".git"))
            .ok_or_else(|| anyhow!("Invalid git URL"))?
            .to_string();
        (name, spec, "main".to_string())
    } else if let Some((user, repo)) = spec.split_once('/') {
        // user/repo -> github
        let name = repo.to_string();
        let url = format!("https://github.com/{}/{}.git", user, repo);
        (name, url, "main".to_string())
    } else {
        // Just name? Default to layman-lang organization
        let name = spec.clone();
        let url = format!("https://github.com/layman-lang/{}.git", spec);
        (name, url, "main".to_string())
    };

    let cwd = std::env::current_dir()?;
    let manifest_path = cwd.join("layman.toml");

    if !manifest_path.exists() {
        return Err(anyhow!("layman.toml not found. Run 'layman init' first."));
    }

    let mut manifest = Manifest::load(&manifest_path)?;

    let dep = Dependency::Detailed(DependencyDetail {
        git: Some(git_url.clone()),
        rev: Some(rev.clone()),
        path: None,
        tag: None,
    });

    manifest.dependencies.insert(name.clone(), dep);
    manifest.save(&manifest_path)?;
    
    println!("Added dependency {} to layman.toml", name);
    
    // Auto-sync
    sync()?;
    
    Ok(())
}

pub fn uninstall(name: String) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let manifest_path = cwd.join("layman.toml");

    if !manifest_path.exists() {
        return Err(anyhow!("layman.toml not found"));
    }

    let mut manifest = Manifest::load(&manifest_path)?;

    if manifest.dependencies.remove(&name).is_some() {
        manifest.save(&manifest_path)?;
        println!("Removed dependency {} from layman.toml", name);
        
        // Remove from modules
        let modules_dir = cwd.join("modules").join(&name);
        if modules_dir.exists() {
            std::fs::remove_dir_all(&modules_dir)?;
            println!("Removed {} from modules/", name);
        }
        
        sync()?;
    } else {
        println!("Dependency {} not found", name);
    }

    Ok(())
}

pub fn sync() -> Result<()> {
    let cwd = std::env::current_dir()?;
    let manifest_path = cwd.join("layman.toml");
    let lockfile_path = cwd.join("layman.lock");

    if !manifest_path.exists() {
        return Err(anyhow!("layman.toml not found"));
    }

    let manifest = Manifest::load(&manifest_path)?;
    println!("Resolving dependencies...");

    let mut resolver = Resolver::new()?;
    let packages = resolver.resolve(&manifest)?;

    // Create lockfile
    let root_id = packages.iter()
        .find(|p| p.id.name == manifest.package.name && p.id.source == "root")
        .map(|p| p.id.clone())
        .ok_or_else(|| anyhow!("Root package not found in resolution"))?;

    let lockfile = Lockfile {
        schema_version: 1,
        root: root_id,
        packages: packages.clone(),
    };

    lockfile.save(&lockfile_path)?;
    println!("Updated layman.lock");
    
    // Install to modules
    let fetcher = Fetcher::new()?;
    for pkg in packages {
        if pkg.id.source == "root" { continue; }
        
        // Get checkout path (ensure it's fetched)
        let checkout_path = resolver.get_checkout_path(&pkg.id)?;
        
        // Install to modules
        fetcher.install_to_modules(&checkout_path, &pkg.id.name)?;
        println!("Installed {} to modules/{}", pkg.id.name, pkg.id.name);
    }
    
    Ok(())
}

pub fn graph() -> Result<()> {
    let cwd = std::env::current_dir()?;
    let lockfile_path = cwd.join("layman.lock");

    if !lockfile_path.exists() {
        return Err(anyhow!("layman.lock not found. Run 'layman sync' first."));
    }

    let lockfile = Lockfile::load(&lockfile_path)?;
    
    println!("Dependency Graph for {}:", lockfile.root.name);
    
    // Simple recursive printer
    print_deps(&lockfile, &lockfile.root.name, 0);
    
    Ok(())
}

fn print_deps(lockfile: &Lockfile, package_name: &str, depth: usize) {
    // Find entry
    if let Some(entry) = lockfile.packages.iter().find(|p| p.id.name == package_name) {
        for dep_id in &entry.dependencies {
            println!("{:indent$}├── {} ({})", "", dep_id.name, dep_id.rev, indent = depth * 4);
            print_deps(lockfile, &dep_id.name, depth + 1);
        }
    }
}
