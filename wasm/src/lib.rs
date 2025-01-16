// WASM bindings for Layman compiler
use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Import from parent crate - use the lib name directly
use layman_lib::lexer;
use layman_lib::parser;
use layman_lib::evaluator;
use layman_lib::typechecker;
use layman_lib::resolver;
use layman_lib::ast;

// global output buffer for capturing print statements
static OUTPUT: Mutex<String> = Mutex::new(String::new());

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn compile_and_run(code: &str) -> Result<String, String> {
    // clear output buffer
    {
        let mut output = OUTPUT.lock().unwrap();
        output.clear();
    }
    
    // step 1: lex
    let mut lexer = lexer::Lexer::new(code, "main.lay".to_string());
    let tokens = lexer.tokenize()
        .map_err(|e| format!("Lexer error: {}", e))?;
    
    // step 2: parse
    let mut parser = parser::Parser::new(tokens);
    let mut ast = parser.parse()
        .map_err(|e| format!("Parser error: {}", e))?;
    
    // step 3: type check
    let mut typechecker = typechecker::TypeChecker::new();
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
    
    // step 4: evaluate with output capture
    let output_arc: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let output_clone = output_arc.clone();
    let mut eval = evaluator::Evaluator::with_output_callback(move |s: &str| {
        let mut output = output_clone.lock().unwrap();
        output.push_str(s);
        output.push('\n');
    });
    
    if let ast::Node::Program(ref prog) = ast {
        for stmt in &prog.statements {
            match eval.evaluate(stmt) {
                Ok(_) => {},
                Err(e) => {
                    return Err(format!("Runtime error: {}", e));
                }
            }
        }
    }
    
    // get captured output
    let output = output_arc.lock().unwrap().clone();
    Ok(output)
}

#[wasm_bindgen]
pub fn compile_and_run_with_files(code: &str, files_json: &str) -> Result<String, String> {
    // parse files from JSON
    let files: HashMap<String, String> = serde_json::from_str(files_json)
        .map_err(|e| format!("Failed to parse files JSON: {}", e))?;
    
    // for now, just use the main code
    // TODO: handle imports from files map
    compile_and_run(code)
}

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}
