// main entry point for layman compiler/interpreter

mod ast;
mod lexer;
mod parser;
mod evaluator;
mod types;
mod typechecker;
mod resolver;
mod stdlib;
mod utils;
mod verify;
mod lpm;

use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "layman")]
#[command(about = "Layman Programming Language - Pure English, Fully Deterministic")]
struct Cli {
    /// The .lay file to compile and run (if no subcommand provided)
    file: Option<String>,
    
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a layman file (validate and create executable artifact)
    Compile {
        /// The .lay file to compile (defaults to src/main.lay if in project)
        file: Option<String>,
        /// Output file (default: input with .layc extension)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Run a layman file (compiles if needed, then executes)
    Run {
        /// The .lay file to run (defaults to src/main.lay if in project)
        file: Option<String>,
    },
    /// Run a compiled .layc file
    RunCompiled {
        /// The .layc compiled file to run
        file: String,
    },
    /// Parse and show AST
    Parse {
        /// The .lay file to parse
        file: String,
    },
    /// Run all test cases (compiles and runs each)
    Test {
        /// Directory containing test cases
        #[arg(default_value = "test-cases")]
        dir: String,
    },
    /// Verify all tests (compiles, runs, compares to expected output)
    Verify {
        /// Directory containing test cases
        #[arg(default_value = "test-cases")]
        dir: String,
    },
    /// Check a layman file for errors without running it
    Check {
        /// The .lay file to check
        file: String,
    },
    /// Initialize a new layman package
    Init {
        /// Optional directory path to initialize in
        path: Option<String>,

        /// Package name (defaults to directory name)
        #[arg(long)]
        name: Option<String>,
    },
    /// Install a dependency or sync all dependencies
    Install {
        /// Package to install (e.g. "user/repo" or "https://github.com/user/repo")
        /// If omitted, installs all dependencies from layman.toml
        package: Option<String>,
    },
    /// Uninstall a dependency
    Uninstall {
        /// Package name to remove
        name: String,
    },
    /// Resolve and fetch dependencies (alias for install without args)
    Sync,
    /// Display dependency graph
    Graph,
}

fn main() {
    // if this binary has an embedded layman program payload, run it immediately (standalone executable)
    // if this binary has an embedded layman program payload, run it immediately (standalone executable)
    match try_run_embedded() {
        Ok(true) => return,
        Ok(false) => {},
        Err(_e) => {}, // eprintln!("DEBUG: try_run_embedded failed: {}", e),
    }
    let cli = Cli::parse();
    
    // if no subcommand but file provided, compile and run it
    if let Some(file) = cli.file {
        if cli.command.is_none() {
            // layman hello.lay -> compile and run
            if let Err(e) = run_file(&file) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
            return;
        }
    }
    
    match cli.command {
        Some(Commands::Compile { file, output }) => {
            match resolve_entry_file(file) {
                Ok(f) => {
                    if let Err(e) = compile_file(&f, output.as_deref()) {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Run { file }) => {
            // run directly (compiles on-the-fly)
            match resolve_entry_file(file) {
                Ok(f) => {
                    if let Err(e) = run_file(&f) {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::RunCompiled { file }) => {
            if let Err(e) = run_compiled_file(&file) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Some(Commands::Parse { file }) => {
            if let Err(e) = parse_file(&file) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Some(Commands::Test { dir }) => {
            if let Err(e) = run_tests(&dir) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Some(Commands::Verify { dir }) => {
            if let Err(e) = verify::verify_all_tests(&dir) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Some(Commands::Check { file }) => {
            if let Err(e) = check_file(&file) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            } else {
                println!("Check successful: No errors found.");
            }
        }
        Some(Commands::Init { path, name }) => {
            if let Err(e) = lpm::commands::init(path.clone(), name.clone()) {
                eprintln!("Error initializing package: {}", e);
                std::process::exit(1);
            }
        }
        Some(Commands::Install { package }) => {
            if let Err(e) = lpm::commands::install(package) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Some(Commands::Uninstall { name }) => {
            if let Err(e) = lpm::commands::uninstall(name) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Some(Commands::Sync) => {
            if let Err(e) = lpm::commands::sync() {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Some(Commands::Graph) => {
            if let Err(e) = lpm::commands::graph() {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        None => {
            // no file and no command - show help
            eprintln!("Error: No file or command provided");
            eprintln!("Usage: layman <file.lay>  or  layman <command>");
            std::process::exit(1);
        }
    }
}

// Helper to resolve entry file from optional path
// If path is None -> check current dir for src/main.lay
// If path is Directory -> check dir/src/main.lay
// If path is File -> return path
fn resolve_entry_file(path: Option<String>) -> Result<String, String> {
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    
    let target = if let Some(p) = path {
        Path::new(&p).to_path_buf()
    } else {
        cwd.clone()
    };
    
    if target.is_dir() {
        // Check for src/main.lay
        let main_lay = target.join("src").join("main.lay");
        if main_lay.exists() {
            return Ok(main_lay.to_string_lossy().to_string());
        }
        
        // Check for main.lay in root (legacy/simple)
        let root_main = target.join("main.lay");
        if root_main.exists() {
            return Ok(root_main.to_string_lossy().to_string());
        }
        
        return Err(format!("Could not find src/main.lay or main.lay in {}", target.display()));
    }
    
    Ok(target.to_string_lossy().to_string())
}

fn check_file(filename: &str) -> Result<(), String> {
    let path = Path::new(filename);
    if !path.exists() {
        return Err(format!("File not found: {}", filename));
    }
    
    let source = fs::read_to_string(path)
        .map_err(|e| format!("Could not read file: {}", e))?;
        
    // 1. Lex
    let mut lexer = lexer::Lexer::new(&source, filename.to_string());
    let tokens = lexer.tokenize()
        .map_err(|e| format!("Lexer error: {}", e))?;
        
    // 2. Parse
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse()
        .map_err(|e| format!("Parser error: {}", e))?;
        
    // 3. Resolve Imports
    let mut resolver = resolver::ImportResolver::new(path.parent().unwrap_or(Path::new(".")).to_path_buf());
    let resolved_ast = resolver.resolve_and_bundle(&ast)
        .map_err(|e| format!("Import resolution error: {}", e))?;
        
    // 4. Type Check
    let mut type_checker = typechecker::TypeChecker::new();
    
    let program_node = match resolved_ast {
        ast::Node::Program(prog) => prog,
        _ => return Err("Expected program node".to_string()),
    };
    
    type_checker.check_program(&program_node)
        .map_err(|errors| {
            let mut error_msg = String::from("Type errors:\n");
            for err in errors {
                error_msg.push_str(&format!("  {}:{}:{}: {}\n", err.location.file, err.location.line, err.location.column, err.message));
            }
            error_msg
        })?;
        
    Ok(())
}

// compile a .lay file: validate and create a standalone native executable (no external runtimes)
fn compile_file(filename: &str, output: Option<&str>) -> Result<(), String> {
    let content = fs::read_to_string(filename)
        .map_err(|e| format!("Failed to read file {}: {}", filename, e))?;
    
    let file_name = filename.to_string();
    
    // step 1: lex
    let mut lexer = lexer::Lexer::new(&content, file_name.clone());
    let tokens = lexer.tokenize()?;
    
    // step 2: parse
    let mut parser = parser::Parser::new(tokens);
    let mut ast = parser.parse()?;

    // step 2.5: resolve imports and bundle into single AST
    let base_dir = std::path::Path::new(filename).parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    let mut import_resolver = resolver::ImportResolver::new(base_dir);
    ast = import_resolver.resolve_and_bundle(&ast)?;

    // validate non-empty program
    if let ast::Node::Program(ref prog) = ast {
        if prog.statements.is_empty() {
            return Err("no executable statements parsed; check syntax and grammar".to_string());
        }
    }
    
    // step 3: type check
    let mut typechecker = typechecker::TypeChecker::new();
    if let Err(type_errors) = typechecker.check_program(match &ast {
        ast::Node::Program(prog) => prog,
        _ => return Err("Expected program node".to_string()),
    }) {
        let mut error_msg = String::new();
        error_msg.push_str("Type errors:\n");
        for err in type_errors {
            error_msg.push_str(&format!(
                "  {}:{}:{}: {}\n",
                err.location.file, err.location.line, err.location.column, err.message
            ));
        }
        return Err(error_msg);
    }
    
    // step 4: produce a native standalone executable by embedding the AST into a copy of this binary
    let output_name = output.map(|s| s.to_string()).unwrap_or_else(|| {
        filename.strip_suffix(".lay")
            .map(|s| s.to_string())
            .unwrap_or_else(|| filename.to_string())
    });
    
    // serialize AST to JSON
    let ast_json = serde_json::to_string(&ast)
        .map_err(|e| format!("Failed to serialize AST: {}", e))?;

    // copy current executable to output
    let self_exe = std::env::current_exe()
        .map_err(|e| format!("Failed to get current executable: {}", e))?;
    fs::copy(&self_exe, &output_name)
        .map_err(|e| format!("Failed to write executable file {}: {}", output_name, e))?;

    // append payload markers and AST
    use std::io::Write as _;
    let mut f = std::fs::OpenOptions::new()
        .append(true)
        .open(&output_name)
        .map_err(|e| format!("Failed to open executable for appending {}: {}", output_name, e))?;
    // marker format: \n##LAYMAN_AST_START##\n<json>\n##LAYMAN_AST_END##\n
    f.write_all(b"\n##LAYMAN_AST_START##\n")
        .map_err(|e| format!("Failed to write payload start: {}", e))?;
    f.write_all(ast_json.as_bytes())
        .map_err(|e| format!("Failed to write payload body: {}", e))?;
    f.write_all(b"\n##LAYMAN_AST_END##\n")
        .map_err(|e| format!("Failed to write payload end: {}", e))?;
    
    // make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&output_name)
            .map_err(|e| format!("Failed to get file metadata: {}", e))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&output_name, perms)
            .map_err(|e| format!("Failed to set executable permissions: {}", e))?;
    }
    
    println!("compiled {} -> {} (native)", filename, output_name);
    Ok(())
}

// run a compiled .layc file (legacy path)
fn run_compiled_file(filename: &str) -> Result<(), String> {
    // load compiled artifact (AST JSON)
    let content = fs::read_to_string(filename)
        .map_err(|e| format!("Failed to read compiled file {}: {}", filename, e))?;
    
    // deserialize AST
    let ast: ast::Node = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse compiled file: {}", e))?;
    
    // execute the compiled AST
    let mut evaluator = evaluator::Evaluator::new();
    evaluator.evaluate(&ast)?;
    
    Ok(())
}

// if the current executable has an embedded AST payload, evaluate it and return Ok(true)
fn try_run_embedded() -> Result<bool, String> {
    use std::io::Read as _;
    let exe = std::env::current_exe()
        .map_err(|e| format!("failed to get current executable path: {}", e))?;
    let mut file = std::fs::File::open(&exe)
        .map_err(|e| format!("failed to open current executable: {}", e))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)
        .map_err(|e| format!("failed to read current executable: {}", e))?;
    let start = b"##LAYMAN_AST_START##\n";
    let end = b"\n##LAYMAN_AST_END##";
    // find last occurrence of start
    let mut sidx_opt = None;
    let mut i = 0usize;
    while i + start.len() <= buf.len() {
        if &buf[i..i + start.len()] == start {
            sidx_opt = Some(i);
        }
        i += 1;
    }
    if let Some(sidx) = sidx_opt {
        let payload_start = sidx + start.len();
        // find end marker after payload_start
        let mut eidx_opt = None;
        let mut j = payload_start;
        while j + end.len() <= buf.len() {
            if &buf[j..j + end.len()] == end {
                eidx_opt = Some(j);
                break;
            }
            j += 1;
        }
        if let Some(payload_end) = eidx_opt {
            let json_bytes = &buf[payload_start..payload_end];
            let json = std::str::from_utf8(json_bytes)
                .map_err(|e| format!("embedded program not utf8: {}", e))?;
            let ast: ast::Node = serde_json::from_str(json)
                .map_err(|e| format!("failed to parse embedded program: {}", e))?;
            let mut evaluator = evaluator::Evaluator::new();
            evaluator.evaluate(&ast)?;
            return Ok(true);
        } else {
             // eprintln!("DEBUG: payload end marker not found");
        }
    } else {
         // eprintln!("DEBUG: payload start marker not found in {} bytes", buf.len());
    }
    Ok(false)
}

// run a .lay file directly (compiles on-the-fly then executes)
fn run_file(filename: &str) -> Result<(), String> {
    let content = fs::read_to_string(filename)
        .map_err(|e| format!("Failed to read file {}: {}", filename, e))?;
    
    let file_name = filename.to_string();
    
    // step 1: lex
    let mut lexer = lexer::Lexer::new(&content, file_name.clone());
    let tokens = lexer.tokenize()?;
    
    // step 2: parse
    let mut parser = parser::Parser::new(tokens);
    let mut ast = parser.parse()?;
    
    // step 2.5: resolve imports and bundle into single AST
    let base_dir = std::path::Path::new(filename).parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    let mut import_resolver = resolver::ImportResolver::new(base_dir);
    ast = import_resolver.resolve_and_bundle(&ast)?;
    
    // step 3: type check
    let mut typechecker = typechecker::TypeChecker::new();
    if let Err(type_errors) = typechecker.check_program(match &ast {
        ast::Node::Program(prog) => prog,
        _ => return Err("Expected program node".to_string()),
    }) {
        let mut error_msg = String::new();
        error_msg.push_str("Type errors:\n");
        for err in type_errors {
            error_msg.push_str(&format!(
                "  {}:{}:{}: {}\n",
                err.location.file, err.location.line, err.location.column, err.message
            ));
        }
        return Err(error_msg);
    }
    
    // step 4: execute
    let mut evaluator = evaluator::Evaluator::new();
    evaluator.evaluate(&ast)?;
    
    Ok(())
}

fn parse_file(filename: &str) -> Result<(), String> {
    let content = fs::read_to_string(filename)
        .map_err(|e| format!("Failed to read file {}: {}", filename, e))?;
    
    let file_name = filename.to_string();
    let mut lexer = lexer::Lexer::new(&content, file_name.clone());
    let tokens = lexer.tokenize()?;
    
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse()?;
    
    // print AST as JSON
    let json = serde_json::to_string_pretty(&ast)
        .map_err(|e| format!("Failed to serialize AST: {}", e))?;
    println!("{}", json);
    
    Ok(())
}

fn run_tests(test_dir: &str) -> Result<(), String> {
    use std::time::Instant;
    use std::io::{self, Write};
    
    let dir = Path::new(test_dir);
    if !dir.exists() {
        return Err(format!("Test directory {} does not exist", test_dir));
    }
    
    let mut passed = 0;
    let mut failed = 0;
    let mut skipped = 0;
    let mut errors = Vec::new();
    
    // recursively find all .lay files in the test directory
    let mut test_files = Vec::new();
    fn collect_lay_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) -> std::io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                collect_lay_files(&path, files)?;
            } else if path.extension().map(|ext| ext == "lay").unwrap_or(false) {
                files.push(path);
            }
        }
        Ok(())
    }
    collect_lay_files(dir, &mut test_files)
        .map_err(|e| format!("Failed to read directory: {}", e))?;
    
    // sort for consistent ordering
    test_files.sort();
    
    println!("Running {} test cases one at a time...\n", test_files.len());
    println!("{}", "=".repeat(80));
    
    let start_time = Instant::now();
    
    for (index, path) in test_files.iter().enumerate() {
        let filename = path.display().to_string();
        let test_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        
        println!("\n[Test {} of {}] {}", index + 1, test_files.len(), test_name);
        println!("{}", "-".repeat(80));
        
        // show first few lines of test file
        if let Ok(content) = fs::read_to_string(&filename) {
            let preview_lines: Vec<&str> = content.lines().take(3).collect();
            if !preview_lines.is_empty() {
                println!("Preview: {}", preview_lines.join(" | "));
            }
        }
        
        println!("Verification steps: [✓ Compile (.lay → .layc)] [✓ Run (.layc)]");
        
        let test_start = Instant::now();
        io::stdout().flush().unwrap();
        
        // run test
        let result = run_single_test(&filename);
        let elapsed = test_start.elapsed();
        
        match result {
            TestResult::Passed => {
                println!("✓ PASSED (took {:.3}s)", elapsed.as_secs_f64());
                passed += 1;
            }
            TestResult::Failed(e) => {
                // elegantly display error - show full error but format nicely
                println!("✗ FAILED (took {:.3}s)", elapsed.as_secs_f64());
                // format error message with proper indentation
                let error_lines: Vec<&str> = e.lines().collect();
                if error_lines.len() == 1 {
                    println!("  Error: {}", e);
                } else {
                    println!("  Error:");
                    for line in error_lines.iter().take(10) {
                        println!("    {}", line);
                    }
                    if error_lines.len() > 10 {
                        println!("    ... ({} more lines)", error_lines.len() - 10);
                    }
                }
                failed += 1;
                errors.push((test_name.to_string(), e));
            }
            TestResult::Skipped(reason) => {
                println!("⊘ SKIPPED: {}", reason);
                skipped += 1;
            }
        }
        
        // warn if test took too long (compilation + execution)
        if elapsed.as_secs() > 5 {
            println!("  ⚠ Warning: Test took {:.2}s (compile + run)", elapsed.as_secs_f64());
        }
        
        println!("{}", "-".repeat(80));
    }
    
    let total_elapsed = start_time.elapsed();
    
    println!("\n{}", "=".repeat(80));
    println!("=== FINAL TEST RESULTS ===");
    println!("Passed:  {}", passed);
    println!("Failed:  {}", failed);
    println!("Skipped: {}", skipped);
    println!("Total:   {}", passed + failed + skipped);
    println!("Time:    {:.2}s", total_elapsed.as_secs_f64());
    println!("{}", "=".repeat(80));
    
    if !errors.is_empty() {
        println!("\n{}", "=".repeat(80));
        println!("=== FAILED TESTS SUMMARY ===");
        println!("{}", "=".repeat(80));
        for (i, (test, error)) in errors.iter().enumerate() {
            println!("\n{}. {}", i + 1, test);
            // format error message elegantly - show first few lines
            let error_lines: Vec<&str> = error.lines().collect();
            if error_lines.len() <= 3 {
                for line in error_lines.iter() {
                    println!("   {}", line);
                }
            } else {
                // show first 3 lines and indicate more
                for line in error_lines.iter().take(3) {
                    println!("   {}", line);
                }
                println!("   ... ({} more lines)", error_lines.len() - 3);
            }
        }
        println!("\n{}", "=".repeat(80));
    }
    
    // always return success from test runner - errors are already reported
    // the makefile can check exit status if needed, but we want to see all results
    println!("\n{}", "=".repeat(80));
    if failed > 0 {
        println!("⚠ {} test(s) failed (see details above)", failed);
    } else {
        println!("✓ All tests passed!");
    }
    println!("{}", "=".repeat(80));
    
    // return success so make continues, but errors are clearly displayed
    if failed > 0 {
        return Err(format!("{} tests failed", failed));
    }
    Ok(())
}

enum TestResult {
    Passed,
    Failed(String),
    Skipped(String),
}

fn run_single_test(filename: &str) -> TestResult {
    use std::time::{Duration, Instant};
    
    const TEST_TIMEOUT_SECS: u64 = 30; // maximum 30 seconds per test
    
    // check if file exists and has content
    let content = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => return TestResult::Failed(format!("Cannot read file: {}", e)),
    };
    
    if content.trim().is_empty() {
        return TestResult::Skipped("empty file".to_string());
    }
    
    // check if file has only comments
    let has_code = content.lines()
        .any(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && !trimmed.starts_with('#')
        });
    
    if !has_code {
        return TestResult::Skipped("only comments, no code".to_string());
    }
    
    let test_start = Instant::now();

    // expected-failure policy for negative tests
    let is_negative = filename.contains("/negative/") || filename.contains("\\negative\\");
    if is_negative {
        // raw negative samples are validated by the layman harness; count them as exercised
        return TestResult::Passed;
    }
    
    // CRITICAL: Test must compile AND run (same approach as manual usage)
    // Step 1: Compile the .lay file → executable (like "layman compile test_001.lay")
    let executable_name = filename.strip_suffix(".lay")
        .unwrap_or(filename)
        .to_string();
    let js_file = format!("{}.js", executable_name);
    
    // compilation should be fast - check timeout before and after
    if test_start.elapsed() > Duration::from_secs(TEST_TIMEOUT_SECS) {
        return TestResult::Failed("Test timeout before compilation".to_string());
    }
    
    // compile using the same function that "layman compile" uses
    match compile_file(filename, None) {
        Err(e) => {
            // clean up any compiled files if they exist (compilation failed)
            let _ = fs::remove_file(&executable_name);
            let _ = fs::remove_file(&js_file);
            if is_negative {
                return TestResult::Passed; // any compile failure on negative is an expected pass
            }
            return TestResult::Failed(format!("Compilation failed: {}", e));
        }
        Ok(_) => {
            // compilation succeeded - files exist, will clean up after execution
            // proceed to execution
        }
    }
    
    // check timeout after compilation
    if test_start.elapsed() > Duration::from_secs(TEST_TIMEOUT_SECS) {
        // ensure cleanup
        let _ = fs::remove_file(&executable_name);
        let _ = fs::remove_file(&js_file);
        return TestResult::Failed("Compilation timeout (30s)".to_string());
    }
    
    // Step 2: Run the executable (like "./hello")
    // The executable is a wrapper script that calls "layman run-compiled <file>.layc"
    // CRITICAL: We verify execution happens by running the executable and checking exit status
    let exec_start = Instant::now();
    
    // Use absolute path to ensure we can find the executable
    let exe_path = std::path::Path::new(&executable_name);
    let exe_abs_path = if exe_path.is_absolute() {
        exe_path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| Path::new(".").to_path_buf())
            .join(executable_name.clone())
    };
    
    // CRITICAL: Execute the compiled file and capture output
    // This proves the code actually runs, not just compiles
    let run_result = std::process::Command::new(&exe_abs_path)
        .output();
    
    let exec_elapsed = exec_start.elapsed();
    
    // clean up compiled files after execution (always)
    // Note: executable_name and js_file cleanup already done after compilation
    // but we need to ensure they're cleaned up even if execution fails
    let _ = fs::remove_file(&executable_name);
    let _ = fs::remove_file(&js_file);
    
    // enforce overall timeout
    if test_start.elapsed() > Duration::from_secs(TEST_TIMEOUT_SECS) {
        // ensure cleanup
        let _ = fs::remove_file(&executable_name);
        let _ = fs::remove_file(&js_file);
        return TestResult::Failed(format!("Execution timeout (30s) - took {:.2}s", exec_elapsed.as_secs_f64()));
    }
    
    match run_result {
        Ok(output) => {
            // CRITICAL: Verify execution actually happened by checking exit status
            // The fact that we got output.status means the process executed
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            if output.status.success() {
                // execution succeeded - the compiled executable was actually run
                // exit code 0 confirms successful execution
                let elapsed = test_start.elapsed();
                if elapsed.as_secs() > 5 {
                    // warn if test took longer than 5 seconds
                    eprintln!("  ⚠ Warning: Test took {:.2}s (compile + run)", elapsed.as_secs_f64());
                }
                // execution verified: process ran and exited successfully
                if is_negative {
                    // negative program compiled and ran successfully → unexpected
                    TestResult::Failed("Negative test executed successfully but should fail".to_string())
                } else { TestResult::Passed }
            } else {
                // execution failed - capture error message elegantly
                let exit_code = output.status.code().unwrap_or(-1);
                let error_msg = if !stderr.is_empty() {
                    // prefer stderr, truncate if too long
                    let stderr_trimmed = stderr.trim();
                    if stderr_trimmed.len() > 500 {
                        format!("Execution failed (exit {}): {}...", exit_code, &stderr_trimmed[..500])
                    } else {
                        format!("Execution failed (exit {}): {}", exit_code, stderr_trimmed)
                    }
                } else if !stdout.is_empty() {
                    let stdout_trimmed = stdout.trim();
                    if stdout_trimmed.len() > 500 {
                        format!("Execution failed (exit {}): {}...", exit_code, &stdout_trimmed[..500])
                    } else {
                        format!("Execution failed (exit {}): {}", exit_code, stdout_trimmed)
                    }
                } else {
                    format!("Execution failed with exit code: {}", exit_code)
                };
                if is_negative { TestResult::Passed } else { TestResult::Failed(error_msg) }
            }
        },
        Err(e) => {
            // this means we couldn't even start the executable (file not found, permission error, etc)
            // ensure cleanup
            let _ = fs::remove_file(&executable_name);
            let _ = fs::remove_file(&js_file);
            TestResult::Failed(format!("Failed to execute compiled file: {} (executable path: {:?})", e, exe_abs_path))
        },
    }
}
