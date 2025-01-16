// import resolver - resolves and bundles imports into single AST
// when compiling hello.test.lay that imports hello.lay, bundles both into single executable

use crate::ast::*;
use crate::lexer;
use crate::parser;
use std::path::PathBuf;
use std::collections::HashSet;

pub struct ImportResolver {
    base_dir: PathBuf,
    visiting: HashSet<String>, // track modules currently being resolved (cycle detection)
    visited: std::collections::HashMap<String, Vec<String>>,  // track modules already resolved (deduplication) + exports
}

impl ImportResolver {
    pub fn new(base_dir: PathBuf) -> Self {
        Self {
            base_dir,
            visiting: HashSet::new(),
            visited: std::collections::HashMap::new(),
        }
    }
    
    /// resolve all imports in a program and bundle into single AST
    pub fn resolve_and_bundle(&mut self, ast: &Node) -> Result<Node, String> {
        match ast {
            Node::Program(prog) => {
                let mut bundled_statements = Vec::new();
                
                // separate imports from other statements
                let mut imports = Vec::new();
                let mut other_statements = Vec::new();
                
                for stmt in &prog.statements {
                    // println!("DEBUG: checking stmt kind");
                    match stmt {
                        Node::ImportStatement(import) => {
                            println!("DEBUG: Found ImportStatement for {}", import.module_name);
                            imports.push(import);
                        }
                        _ => {
                            other_statements.push(stmt.clone());
                        }
                    }
                }
                
                // resolve each import and bundle its statements
                for import in imports {
                    let (imported_ast, exports) = self.resolve_import(&import.module_name)?;
                    if let Node::Program(imported_prog) = imported_ast {
                        // add imported statements (excluding imports themselves to avoid cycles)
                        for stmt in imported_prog.statements {
                            if !matches!(stmt, Node::ImportStatement(_)) {
                                bundled_statements.push(stmt);
                            }
                        }
                        
                        // Handle alias: create a dictionary containing all exports
                        if let Some(alias) = &import.alias {
                            let mut dict_args = Vec::new();
                            for export_name in exports {
                                // key: string literal
                                dict_args.push(Node::LiteralExpression(LiteralExpression {
                                    location: import.location.clone(),
                                    value: LiteralValue::String(export_name.clone()),
                                }));
                                // value: variable expression
                                dict_args.push(Node::VariableExpression(VariableExpression {
                                    location: import.location.clone(),
                                    identifier: export_name,
                                }));
                            }
                            
                            // create_dictionary call
                            let create_dict_call = Node::CallExpression(CallExpression {
                                location: import.location.clone(),
                                function_name: "create_dictionary".to_string(),
                                arguments: dict_args,
                            });
                            
                            // define variable alias = ...
                            let alias_decl = Node::DeclareStatement(DeclareStatement {
                                location: import.location.clone(),
                                name: alias.clone(),
                                value: Box::new(create_dict_call),
                                type_annotation: None, // TODO: Dictionary type?
                                is_mutable: false,
                            });
                            
                            bundled_statements.push(alias_decl);
                        }
                    }
                }
                
                // add original non-import statements
                bundled_statements.extend(other_statements);
                
                Ok(Node::Program(Program {
                    location: prog.location.clone(),
                    statements: bundled_statements,
                }))
            }
            _ => Ok(ast.clone()),
        }
    }
    
    /// resolve a single import: find file, parse it, return AST and exports
    fn resolve_import(&mut self, module_name: &str) -> Result<(Node, Vec<String>), String> {
        // check for cycles
        if self.visiting.contains(module_name) {
            return Err(format!("Circular import detected: {}", module_name));
        }
        
        // if already visited, don't include it again (deduplication)
        if let Some(exports) = self.visited.get(module_name) {
            return Ok((Node::Program(Program {
                location: Location { file: module_name.to_string(), line: 0, column: 0, source: None },
                statements: Vec::new(),
            }), exports.clone()));
        }
        
        self.visiting.insert(module_name.to_string());
        
        // resolve file path
        let file_path = self.resolve_file_path(module_name)?;
        
        // read and parse file
        let content = std::fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read import {}: {}", module_name, e))?;
        
        let file_name = file_path.display().to_string();
        
        // lex
        let mut lexer = lexer::Lexer::new(&content, file_name.clone());
        let tokens = lexer.tokenize()?;
        
        // parse
        let mut parser = parser::Parser::new(tokens);
        let mut ast = parser.parse()?;
        
        // extract exports BEFORE recursive resolution (to avoid getting exports of sub-imports?)
        // actually, we want exports of THIS module.
        let exports = self.get_exports(&ast);
        
        // recursively resolve imports in the imported file
        ast = self.resolve_and_bundle(&ast)?;
        
        self.visiting.remove(module_name);
        self.visited.insert(module_name.to_string(), exports.clone());
        
        Ok((ast, exports))
    }
    
    /// resolve import path to actual file path
    fn resolve_file_path(&self, module_name: &str) -> Result<PathBuf, String> {
        // remove .lay extension if present
        let module = if module_name.ends_with(".lay") {
            &module_name[..module_name.len() - 4]
        } else {
            module_name
        };
        
        // try different path resolutions
        let mut candidates = vec![
            // relative to current file
            self.base_dir.join(format!("{}.lay", module)),
            // relative to current directory
            PathBuf::from(format!("{}.lay", module)),
            // relative with parent directory
            self.base_dir.parent()
                .map(|p| p.join(format!("{}.lay", module)))
                .unwrap_or_else(|| PathBuf::from(format!("{}.lay", module))),
            // standard library (std/) in project root
            // Assuming we are running from project root or base_dir is inside project
            // We'll try to find 'std' relative to where we are
            // standard library (std/) in project root
            PathBuf::from("std").join(format!("{}.lay", module)),
            // Also try relative to the executable/project root if base_dir is deep
            std::env::current_dir().unwrap_or_default().join("std").join(format!("{}.lay", module)),
        ];

        // Check for installed modules (e.g. "package/module" -> "modules/package/src/module.lay")
        if let Some((pkg, mod_path)) = module_name.split_once('/') {
             // Try modules/pkg/src/mod.lay
             candidates.push(
                 std::env::current_dir().unwrap_or_default()
                    .join("modules")
                    .join(pkg)
                    .join("src")
                    .join(format!("{}.lay", mod_path))
             );
        }
        
        // println!("DEBUG: Resolving module '{}' from base '{}'", module_name, self.base_dir.display());
        for candidate in &candidates {
            // println!("DEBUG: Checking candidate: {}", candidate.display());
            if candidate.exists() {
                return Ok(candidate.clone());
            }
        }
        
        Err(format!("Import not found: {} (searched relative to {})", module_name, self.base_dir.display()))
    }

    fn get_exports(&self, ast: &Node) -> Vec<String> {
        let mut exports = Vec::new();
        if let Node::Program(prog) = ast {
            for stmt in &prog.statements {
                match stmt {
                    Node::FunctionDeclaration(func) => exports.push(func.name.clone()),
                    Node::DeclareStatement(var) => exports.push(var.name.clone()),
                    Node::ClassDeclaration(class) => exports.push(class.name.clone()),
                    Node::TypeDeclaration(type_decl) => exports.push(type_decl.name.clone()),
                    _ => {}
                }
            }
        }
        exports
    }

}

