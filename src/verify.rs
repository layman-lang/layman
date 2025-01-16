// test verification system that compiles, runs, and compares output

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn verify_all_tests(test_dir: &str) -> Result<(), String> {
    let mut passed = 0;
    let mut failed = 0;
    let mut total = 0;
    let mut errors = Vec::new();
    
    // find all .lay files recursively
    let test_files = collect_lay_files(Path::new(test_dir))?;
    
    println!("==========================================");
    println!("Layman Test Verification System");
    println!("==========================================");
    println!("");
    
    for test_file in test_files {
        total += 1;
        let test_name = test_file.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        
        // skip negative tests (they have different verification)
        if test_file.to_string_lossy().contains("/negative/") {
            continue;
        }
        
        println!("[{}] Testing: {}", total, test_file.display());
        
        match verify_single_test(&test_file) {
            Ok(()) => {
                println!("  ✓ PASSED");
                passed += 1;
            }
            Err(e) => {
                println!("  ✗ FAILED: {}", e);
                failed += 1;
                errors.push((test_name.to_string(), e));
            }
        }
    }
    
    println!("");
    println!("==========================================");
    println!("Results: {} passed, {} failed, {} total", passed, failed, total);
    println!("==========================================");
    
    if !errors.is_empty() {
        println!("\nFailed tests:");
        for (name, error) in errors {
            println!("  {}: {}", name, error);
        }
        return Err(format!("{} test(s) failed", failed));
    }
    
    Ok(())
}

fn verify_single_test(test_file: &Path) -> Result<(), String> {
    let executable = test_file.with_extension("");
    let expected_file = test_file.with_extension("lay.expected");
    
    // step 1: compile
    let compile_output = Command::new("target/debug/layman")
        .arg("compile")
        .arg(test_file)
        .output()
        .map_err(|e| format!("Failed to run compiler: {}", e))?;
    
    if !compile_output.status.success() {
        let stderr = String::from_utf8_lossy(&compile_output.stderr);
        return Err(format!("Compilation failed: {}", stderr));
    }
    
    // step 2: run and capture output
    if !executable.exists() {
        return Err("Executable not created".to_string());
    }
    
    // make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&executable)
            .map_err(|e| format!("Cannot get file metadata: {}", e))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&executable, perms)
            .map_err(|e| format!("Cannot set permissions: {}", e))?;
    }
    
    let run_output = Command::new(&executable)
        .output()
        .map_err(|e| format!("Failed to run executable: {}", e))?;
    
    let actual_output = String::from_utf8_lossy(&run_output.stdout).trim().to_string();
    let actual_stderr = String::from_utf8_lossy(&run_output.stderr).trim().to_string();
    
    // combine stdout and stderr for comparison
    let full_output = if actual_stderr.is_empty() {
        actual_output
    } else {
        format!("{}\n{}", actual_output, actual_stderr)
    };
    
    // step 3: compare to expected output
    if expected_file.exists() {
        let expected_output = fs::read_to_string(&expected_file)
            .map_err(|e| format!("Cannot read expected file: {}", e))?
            .trim()
            .to_string();
        
        if full_output != expected_output {
            return Err(format!(
                "Output mismatch\nExpected:\n{}\nActual:\n{}",
                expected_output, full_output
            ));
        }
    } else {
        // no expected file - just check that it ran without error
        if !run_output.status.success() {
            return Err(format!(
                "Execution failed (exit code: {})\nOutput: {}",
                run_output.status.code().unwrap_or(-1),
                full_output
            ));
        }
        // warn but don't fail if no expected file
        println!("    ⚠ No .expected file (output: {})", full_output);
    }
    
    // cleanup
    let _ = fs::remove_file(&executable);
    
    Ok(())
}

fn collect_lay_files(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    
    if !dir.exists() {
        return Err(format!("Directory {} does not exist", dir.display()));
    }
    
    fn collect_recursive(dir: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                collect_recursive(&path, files)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("lay") {
                files.push(path);
            }
        }
        Ok(())
    }
    
    collect_recursive(dir, &mut files)
        .map_err(|e| format!("Failed to read directory: {}", e))?;
    
    files.sort();
    Ok(files)
}


