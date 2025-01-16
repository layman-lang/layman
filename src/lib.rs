// Library entry point for layman compiler
// Exposes core functionality for use in WASM and other contexts

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod evaluator;
pub mod types;
pub mod typechecker;
pub mod resolver;
pub mod stdlib;
pub mod utils;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Compile and run Layman code, returning output as string
pub fn compile_and_run(code: &str, files: Option<&HashMap<String, String>>) -> Result<String, String> {
    use evaluator::Evaluator;
    use lexer::Lexer;
    use parser::Parser;
    use resolver::ImportResolver;
    use typechecker::TypeChecker;
    use std::path::PathBuf;
    
    // capture stdout
    let output = Arc::new(Mutex::new(String::new()));
    let output_clone = output.clone();
    
    // override print to capture output
    // we'll need to modify the evaluator to use a callback or capture mechanism
    // for now, let's use a simpler approach with a custom print handler
    
    // step 1: lex
    let mut lexer = Lexer::new(code, "main.lay".to_string());
    let tokens = lexer.tokenize()
        .map_err(|e| format!("Lexer error: {}", e))?;
    
    // step 2: parse
    let mut parser = Parser::new(tokens);
    let mut ast = parser.parse()
        .map_err(|e| format!("Parser error: {}", e))?;
    
    // step 3: resolve imports (if files provided)
    if let Some(file_map) = files {
        // create a virtual file system resolver
        let base_dir = PathBuf::from(".");
        let mut resolver = ImportResolver::new(base_dir);
        
        // for now, we'll handle imports differently in WASM
        // the resolver expects file system access which we don't have in WASM
        // we'll need to modify the resolver or handle imports in the files map
    }
    
    // step 4: type check
    let mut typechecker = TypeChecker::new();
    if let ast::Node::Program(ref prog) = ast {
        if let Err(type_errors) = typechecker.check_program(prog) {
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
    }
    
    // step 5: evaluate
    let mut evaluator = Evaluator::new();
    
    // we need to capture print output
    // the evaluator uses println! which we can't capture easily
    // we'll need to modify the evaluator to use a callback or output buffer
    // for now, let's create a simple wrapper that captures stdout
    
    // evaluate the program
    if let ast::Node::Program(ref prog) = ast {
        for stmt in &prog.statements {
            evaluator.evaluate(stmt)
                .map_err(|e| format!("Runtime error: {}", e))?;
        }
    }
    
    // for now, return empty output since we can't capture println!
    // we'll need to modify the evaluator to support output capture
    Ok(String::new())
}

