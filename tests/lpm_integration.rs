use std::process::Command;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_lpm_integration() {
    // 1. Setup temp directory
    let temp_dir = std::env::temp_dir().join("layman_lpm_test");
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir).unwrap();
    }
    fs::create_dir_all(&temp_dir).unwrap();
    
    // Get absolute path to layman binary
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let layman_bin = manifest_dir.join("target/debug/layman");
    
    if !layman_bin.exists() {
        panic!("Layman binary not found at {}. Run 'cargo build' first.", layman_bin.display());
    }

    // 2. Setup local git repo from fixture
    let fixture_path = manifest_dir.join("tests/fixtures/hello-layman");
    let repo_path = temp_dir.join("hello-layman");
    fs::create_dir_all(&repo_path).unwrap();
    
    // Copy fixture to repo_path
    copy_dir_recursive(&fixture_path, &repo_path).unwrap();
    
    // Initialize git repo
    // Use --initial-branch=main to ensure deterministic branch name across environments (CI might default to master)
    let init_status = Command::new("git")
        .args(&["init", "--initial-branch=main"])
        .current_dir(&repo_path)
        .status();

    // Fallback for older git versions that don't support --initial-branch
    if init_status.is_err() || !init_status.as_ref().unwrap().success() {
        Command::new("git")
            .arg("init")
            .current_dir(&repo_path)
            .status()
            .expect("Failed to init git repo");
            
        Command::new("git")
            .args(&["checkout", "-b", "main"])
            .current_dir(&repo_path)
            .status()
            .ok(); // Ignore error if already on main
    }
        
    Command::new("git")
        .args(&["config", "user.email", "test@example.com"])
        .current_dir(&repo_path)
        .status()
        .unwrap();
        
    Command::new("git")
        .args(&["config", "user.name", "Test User"])
        .current_dir(&repo_path)
        .status()
        .unwrap();

    Command::new("git")
        .args(&["add", "."])
        .current_dir(&repo_path)
        .status()
        .expect("Failed to git add");
        
    Command::new("git")
        .args(&["commit", "-m", "Initial commit"])
        .current_dir(&repo_path)
        .status()
        .expect("Failed to git commit");

    // 3. Init project
    let project_dir = temp_dir.join("my_project");
    fs::create_dir_all(&project_dir).unwrap();
    
    let status = Command::new(&layman_bin)
        .arg("init")
        .current_dir(&project_dir)
        .status()
        .expect("Failed to run layman init");
    assert!(status.success());
    assert!(project_dir.join("src/main.lay").exists());

    // 4. Install from local git repo
    println!("Installing hello-layman from local git...");
    let status = Command::new(&layman_bin)
        .arg("install")
        .arg(format!("file://{}", repo_path.display()))
        .current_dir(&project_dir)
        .status()
        .expect("Failed to run layman install");
    assert!(status.success());

    // 5. Verify installation
    assert!(project_dir.join("layman.lock").exists());
    assert!(project_dir.join("modules/hello-layman").exists()); // Name derived from folder name

    // 6. Test 'graph' command
    println!("Testing 'graph' command...");
    let output = Command::new(&layman_bin)
        .arg("graph")
        .current_dir(&project_dir)
        .output()
        .expect("Failed to run layman graph");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("hello-layman"));

    // 7. Test 'sync' command (delete modules and restore)
    println!("Testing 'sync' command...");
    fs::remove_dir_all(project_dir.join("modules")).unwrap();
    assert!(!project_dir.join("modules/hello-layman").exists());
    
    let status = Command::new(&layman_bin)
        .arg("sync")
        .current_dir(&project_dir)
        .status()
        .expect("Failed to run layman sync");
    assert!(status.success());
    assert!(project_dir.join("modules/hello-layman").exists());

    // 8. Create a test file from fixture
    let main_lay = project_dir.join("test_import.lay");
    let fixture_consumer = manifest_dir.join("tests/fixtures/test-consumer/main.lay");
    fs::copy(&fixture_consumer, &main_lay).expect("Failed to copy test consumer fixture");

    // 9. Run the file
    let output = Command::new(&layman_bin)
        .arg("run")
        .arg("test_import.lay")
        .current_dir(&project_dir)
        .output()
        .expect("Failed to run layman run");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("Stdout: {}", stdout);
    println!("Stderr: {}", stderr);
    
    assert!(output.status.success());
    assert!(stdout.contains("Imported successfully"));
    assert!(stdout.contains("Hello from Layman Package!"));

    // 8. Uninstall
    let status = Command::new(&layman_bin)
        .arg("uninstall")
        .arg("hello-layman")
        .current_dir(&project_dir)
        .status()
        .expect("Failed to run layman uninstall");
    assert!(status.success());
    
    // Cleanup
    fs::remove_dir_all(&temp_dir).unwrap();
}

fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

#[test]
fn test_init_variants() {
    let temp_dir = std::env::temp_dir().join("layman_init_test");
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir).unwrap();
    }
    fs::create_dir_all(&temp_dir).unwrap();

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let layman_bin = manifest_dir.join("target/debug/layman");

    // 1. Test init with directory argument
    let status = Command::new(&layman_bin)
        .arg("init")
        .arg("new_project")
        .current_dir(&temp_dir)
        .status()
        .expect("Failed to run layman init new_project");
    assert!(status.success());
    
    let new_project_dir = temp_dir.join("new_project");
    assert!(new_project_dir.exists());
    assert!(new_project_dir.join("layman.toml").exists());
    assert!(new_project_dir.join("src/main.lay").exists());

    // 2. Test init with --name argument in current directory
    let custom_dir = temp_dir.join("custom_dir");
    fs::create_dir(&custom_dir).unwrap();
    
    let status = Command::new(&layman_bin)
        .arg("init")
        .arg("--name")
        .arg("my_custom_pkg")
        .current_dir(&custom_dir)
        .status()
        .expect("Failed to run layman init --name");
    assert!(status.success());
    
    let manifest_content = fs::read_to_string(custom_dir.join("layman.toml")).unwrap();
    assert!(manifest_content.contains("name = \"my_custom_pkg\""));

    // Cleanup
    fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_smart_cli() {
    let temp_dir = std::env::temp_dir().join("layman_smart_cli_test");
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir).unwrap();
    }
    fs::create_dir_all(&temp_dir).unwrap();

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let layman_bin = manifest_dir.join("target/debug/layman");

    // Create a project
    let project_dir = temp_dir.join("my_project");
    let status = Command::new(&layman_bin)
        .arg("init")
        .arg("my_project")
        .current_dir(&temp_dir)
        .status()
        .expect("Failed to init project");
    assert!(status.success());
    
    // 1. Test 'layman run' from within project root (should default to src/main.lay)
    let output = Command::new(&layman_bin)
        .arg("run")
        .current_dir(&project_dir)
        .output()
        .expect("Failed to run layman run (no args)");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("Stdout 1: {}", stdout);
    println!("Stderr 1: {}", stderr);
    assert!(stdout.contains("Hello from Layman!"));
    
    // 2. Test 'layman run <dir>' from outside (should resolve to dir/src/main.lay)
    let output = Command::new(&layman_bin)
        .arg("run")
        .arg(&project_dir)
        .current_dir(&temp_dir)
        .output()
        .expect("Failed to run layman run <dir>");
        
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Hello from Layman!"));

    // Cleanup
    fs::remove_dir_all(&temp_dir).unwrap();
}
