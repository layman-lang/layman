// static type checker for layman
// performs type inference and type checking on AST
// follows principles: readability, determinism, clear error messages

use crate::ast::*;
use crate::types::RuntimeType;
use crate::{lexer, parser};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs;

// type context tracks variable types in current scope
#[derive(Debug, Clone)]
pub struct TypeContext {
    variables: HashMap<String, (RuntimeType, bool)>, // (type, is_mutable)
    functions: HashMap<String, FunctionSignature>,
    classes: HashMap<String, ClassDefinition>,
    types: HashMap<String, TypeDefinition>,
    parent: Option<Box<TypeContext>>,
}

#[derive(Debug, Clone)]
pub struct TypeDefinition {
    pub name: String,
    pub variants: HashMap<String, Vec<(String, RuntimeType)>>, // variant_name -> fields
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub parameters: Vec<(String, RuntimeType)>,
    pub return_type: RuntimeType,
}

#[derive(Debug, Clone)]
pub struct ClassDefinition {
    pub name: String,
    pub parent: Option<String>,
    pub properties: HashMap<String, RuntimeType>,
    pub methods: HashMap<String, FunctionSignature>,
}

impl TypeContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            classes: HashMap::new(),
            types: HashMap::new(),
            parent: None,
        }
    }
    
    pub fn with_parent(parent: Box<TypeContext>) -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            classes: HashMap::new(),
            types: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn add_type(&mut self, def: TypeDefinition) {
        self.types.insert(def.name.clone(), def);
    }

    pub fn get_type_def(&self, name: &str) -> Option<&TypeDefinition> {
        if let Some(def) = self.types.get(name) {
            Some(def)
        } else if let Some(parent) = &self.parent {
            parent.get_type_def(name)
        } else {
            None
        }
    }
    
    pub fn is_variant(&self, name: &str) -> bool {
        // Check if 'name' is a known type (Variant type)
        self.get_type_def(name).is_some()
    }
    
    pub fn get_variable_type(&self, name: &str) -> Option<RuntimeType> {
        if let Some((ty, _)) = self.variables.get(name) {
            Some(ty.clone())
        } else if let Some(parent) = &self.parent {
            parent.get_variable_type(name)
        } else {
            None
        }
    }
    
    pub fn get_variable_mutability(&self, name: &str) -> Option<bool> {
        if let Some((_, is_mut)) = self.variables.get(name) {
            Some(*is_mut)
        } else if let Some(parent) = &self.parent {
            parent.get_variable_mutability(name)
        } else {
            None
        }
    }
    
    pub fn set_variable_type(&mut self, name: String, ty: RuntimeType, is_mutable: bool) {
        self.variables.insert(name, (ty, is_mutable));
    }
    
    pub fn get_function_signature(&self, name: &str) -> Option<FunctionSignature> {
        if let Some(sig) = self.functions.get(name) {
            Some(sig.clone())
        } else if let Some(parent) = &self.parent {
            parent.get_function_signature(name)
        } else {
            None
        }
    }
    
    pub fn set_function_signature(&mut self, sig: FunctionSignature) {
        self.functions.insert(sig.name.clone(), sig);
    }
    
    pub fn get_class(&self, name: &str) -> Option<&ClassDefinition> {
        if let Some(cls) = self.classes.get(name) {
            Some(cls)
        } else if let Some(parent) = &self.parent {
            parent.get_class(name)
        } else {
            None
        }
    }
    
    pub fn set_class(&mut self, def: ClassDefinition) {
        self.classes.insert(def.name.clone(), def);
    }
}

// type checking result
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TypeCheckResult {
    Success(RuntimeType),
    Error(String),
}

pub struct TypeChecker {
    context: TypeContext,
    errors: Vec<TypeError>,
}

#[derive(Debug, Clone)]
pub struct TypeError {
    pub location: Location,
    pub message: String,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            context: TypeContext::new(),
            errors: Vec::new(),
        }
    }
    
    // main entry point: type check entire program
    pub fn check_program(&mut self, program: &Program) -> Result<(), Vec<TypeError>> {
        // register standard library functions
        self.register_stdlib();
        
        // type check all statements
        for stmt in &program.statements {
            self.check_statement(stmt)?;
        }
        
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }
    
    fn register_stdlib(&mut self) {
        // register print function
        self.context.set_function_signature(FunctionSignature {
            name: "print".to_string(),
            parameters: vec![("value".to_string(), RuntimeType::Void)], // accepts any type
            return_type: RuntimeType::Void,
        });
        
        self.context.set_function_signature(FunctionSignature {
            name: "printvalue".to_string(),
            parameters: vec![("value".to_string(), RuntimeType::Void)],
            return_type: RuntimeType::Void,
        });
        
        self.context.set_function_signature(FunctionSignature {
            name: "create_list".to_string(),
            parameters: vec![], // variable args
            return_type: RuntimeType::List(Box::new(RuntimeType::Void)),
        });

        self.context.set_function_signature(FunctionSignature {
            name: "len".to_string(),
            parameters: vec![("list".to_string(), RuntimeType::List(Box::new(RuntimeType::Any)))],
            return_type: RuntimeType::Number,
        });

        self.context.set_function_signature(FunctionSignature {
            name: "reverse_list".to_string(),
            parameters: vec![("list".to_string(), RuntimeType::List(Box::new(RuntimeType::Any)))],
            return_type: RuntimeType::List(Box::new(RuntimeType::Any)),
        });

        self.context.set_function_signature(FunctionSignature {
            name: "call_query".to_string(),
            parameters: vec![("query".to_string(), RuntimeType::String)],
            return_type: RuntimeType::Void,
        });

        self.context.set_function_signature(FunctionSignature {
            name: "expect_compilation_error".to_string(),
            parameters: vec![
                ("file".to_string(), RuntimeType::String),
                ("error".to_string(), RuntimeType::String)
            ],
            return_type: RuntimeType::Void,
        });

        self.context.set_function_signature(FunctionSignature {
            name: "create_dictionary".to_string(),
            parameters: vec![], // variable args
            return_type: RuntimeType::Dictionary {
                key: Box::new(RuntimeType::Void),
                value: Box::new(RuntimeType::Void),
            },
        });

        self.context.set_function_signature(FunctionSignature {
            name: "append".to_string(),
            parameters: vec![
                ("list".to_string(), RuntimeType::List(Box::new(RuntimeType::Any))),
                ("item".to_string(), RuntimeType::Any)
            ],
            return_type: RuntimeType::Void,
        });

        self.context.set_function_signature(FunctionSignature {
            name: "concatenate".to_string(),
            parameters: vec![
                ("a".to_string(), RuntimeType::String),
                ("b".to_string(), RuntimeType::String)
            ],
            return_type: RuntimeType::String,
        });

        self.context.set_function_signature(FunctionSignature {
            name: "__io_read_file".to_string(),
            parameters: vec![("path".to_string(), RuntimeType::String)],
            return_type: RuntimeType::String,
        });

        self.context.set_function_signature(FunctionSignature {
            name: "__io_write_file".to_string(),
            parameters: vec![
                ("path".to_string(), RuntimeType::String),
                ("content".to_string(), RuntimeType::String)
            ],
            return_type: RuntimeType::Void,
        });

        self.context.set_function_signature(FunctionSignature {
            name: "__str_split".to_string(),
            parameters: vec![
                ("text".to_string(), RuntimeType::String),
                ("delimiter".to_string(), RuntimeType::String)
            ],
            return_type: RuntimeType::List(Box::new(RuntimeType::String)),
        });

        self.context.set_function_signature(FunctionSignature {
            name: "__str_trim".to_string(),
            parameters: vec![("text".to_string(), RuntimeType::String)],
            return_type: RuntimeType::String,
        });
    }

    fn check_statement(&mut self, stmt: &Node) -> Result<RuntimeType, Vec<TypeError>> {
        match stmt {
            Node::Program(program) => {
                // For Program nodes (blocks), we need to check statements in order
                let mut last_type = RuntimeType::Void;
                for stmt in &program.statements {
                    match self.check_statement(stmt) {
                        Ok(t) => last_type = t,
                        Err(e) => return Err(e),
                    }
                }
                Ok(last_type)
            }

            
            Node::AssignStatement(assign) => {
                // check expression type
                let expr_type = self.check_expression(&assign.expression)?;
                
                // if variable already exists, check compatibility
            if let Some(existing_type) = self.context.get_variable_type(&assign.identifier) {
                // Check mutability
                if let Some(is_mut) = self.context.get_variable_mutability(&assign.identifier) {
                     if !is_mut && existing_type != RuntimeType::Void {
                         self.add_error(&assign.location, format!("Cannot reassign constant '{}'", assign.identifier));
                     }
                }

                if existing_type == RuntimeType::Void {
                    // upgrade placeholder to concrete type
                    let is_mut = self.context.get_variable_mutability(&assign.identifier).unwrap_or(true);
                    self.context.set_variable_type(assign.identifier.clone(), expr_type.clone(), is_mut);
                } else if !self.is_compatible(&expr_type, &existing_type) {
                        self.add_error(&assign.location, format!(
                            "cannot assign {} to variable '{}' which has type {}",
                            self.type_to_string(&expr_type),
                            assign.identifier,
                            self.type_to_string(&existing_type)
                        ));
                    }
                } else {
                    // new variable: infer type from expression
                    self.context.set_variable_type(assign.identifier.clone(), expr_type.clone(), assign.is_mutable);
                }
                
                Ok(expr_type)
            }
            
            Node::DeclareStatement(decl) => {
                let init_type = self.check_expression(&decl.value)?;
                
                let var_type = if let Some(annotated_type) = &decl.type_annotation {
                    // explicit type annotation
                    let annotated_runtime = self.ast_type_to_runtime_type(annotated_type);
                    
                    if !self.is_compatible(&init_type, &annotated_runtime) {
                        self.add_error(&decl.location, format!(
                            "initial value has type {} but variable '{}' is declared as {}",
                            self.type_to_string(&init_type),
                            decl.name,
                            self.type_to_string(&annotated_runtime)
                        ));
                    }
                    annotated_runtime
                } else {
                    // infer from initial value
                    init_type
                };
                
                // register variable in context
                self.context.set_variable_type(decl.name.clone(), var_type.clone(), decl.is_mutable);
                Ok(var_type)
            }
            
            Node::ConditionalStatement(cond) => {
                // condition must be boolean
                let cond_type = self.check_expression(&cond.condition)?;
                if !self.is_boolean_compatible(&cond_type) {
                    let cond_loc = cond.condition.location();
                    self.add_error(&cond_loc, format!(
                        "condition in if statement must be boolean, got {}",
                        self.type_to_string(&cond_type)
                    ));
                }
                
                self.check_statement(&cond.then_branch)?;
                if let Some(else_branch) = &cond.else_branch {
                    self.check_statement(else_branch)?;
                }
                Ok(RuntimeType::Void)
            }
            
            Node::LoopStatement(loop_stmt) => {
                match &loop_stmt.loop_type {
                    LoopType::ForEach => {
                        // TODO: check collection type and register iterator variable
                        if let Some(collection) = &loop_stmt.collection {
                            let col_type = self.check_expression(collection)?;
                            // infer iterator type from collection
                            let iter_type = match col_type {
                                RuntimeType::List(inner) => *inner,
                                RuntimeType::Dictionary { key: _, value: _ } => RuntimeType::String, // iterate keys?
                                _ => RuntimeType::Any,
                            };
                            
                            if let Some(iter_name) = &loop_stmt.iterator {
                                // register iterator in loop body scope?
                                // For now, register in current scope (simplified)
                                self.context.set_variable_type(iter_name.clone(), iter_type, false);
                            }
                        }
                        self.check_statement(&loop_stmt.body)?;
                        Ok(RuntimeType::Void)
                    }
                    
                    LoopType::While => {
                        if let Some(condition) = &loop_stmt.condition {
                            let cond_type = self.check_expression(condition)?;
                            if !self.is_boolean_compatible(&cond_type) {
                                let cond_loc = condition.location();
                                self.add_error(&cond_loc, format!(
                                    "while loop condition must be boolean, got {}",
                                    self.type_to_string(&cond_type)
                                ));
                            }
                        }
                        
                        self.check_statement(&loop_stmt.body)?;
                        Ok(RuntimeType::Void)
                    }
                    
                    LoopType::Repeat => {
                        self.check_statement(&loop_stmt.body)?;
                        Ok(RuntimeType::Void)
                    }
                }
            }
            
            Node::ReturnStatement(ret) => {
                if let Some(expr) = &ret.expression {
                    self.check_expression(expr)
                } else {
                    Ok(RuntimeType::Void)
                }
            }
            
            Node::FunctionDeclaration(func) => {
                // create new context for function body
                let mut func_context = TypeContext::with_parent(Box::new(self.context.clone()));
                
                // register parameters in function context
                let mut param_types = Vec::new();
                for param in &func.parameters {
                    let param_type = if let Some(annotated) = &param.type_annotation {
                        self.ast_type_to_runtime_type(annotated)
                    } else {
                        // parameter without type annotation - default to Any
                        RuntimeType::Any
                    };
                    param_types.push((param.name.clone(), param_type.clone()));
                    func_context.set_variable_type(param.name.clone(), param_type, false);
                }
                

                
                // check function body in its own context
                let old_context = std::mem::replace(&mut self.context, func_context);
                let body_type = self.check_statement(&func.body)?;
                self.context = old_context;
                
                // check return type matches annotation if present
                let expected_return = if let Some(annotated_return) = &func.return_type {
                    self.ast_type_to_runtime_type(annotated_return)
                } else {
                    // no annotation: infer from body
                    body_type.clone()
                };
                
                // register function in context
                self.context.set_function_signature(FunctionSignature {
                    name: func.name.clone(),
                    parameters: param_types,
                    return_type: expected_return,
                });
                
                Ok(RuntimeType::Void)
            }
            
            Node::ImportStatement(stmt) => {
                self.check_import(stmt)?;
                Ok(RuntimeType::Void)
            }
            
            Node::ExpressionStatement(expr_stmt) => {
                self.check_expression(&expr_stmt.expression)
            }
            
            Node::ClassDeclaration(class) => {
                // register class type
                let mut properties = HashMap::new();
                for prop in &class.properties {
                    let prop_type = self.ast_type_to_runtime_type(&prop.type_annotation);
                    properties.insert(prop.name.clone(), prop_type);
                }
                
                let mut methods = HashMap::new();
                for method in &class.methods {
                    let mut param_types = Vec::new();
                    for param in &method.parameters {
                        let param_type = if let Some(annotated) = &param.type_annotation {
                            self.ast_type_to_runtime_type(annotated)
                        } else {
                            RuntimeType::Void
                        };
                        param_types.push((param.name.clone(), param_type));
                    }
                    
                    let return_type = if let Some(annotated) = &method.return_type {
                        self.ast_type_to_runtime_type(annotated)
                    } else {
                        RuntimeType::Void // simplified: should infer from body
                    };
                    
                    methods.insert(method.name.clone(), FunctionSignature {
                        name: method.name.clone(),
                        parameters: param_types,
                        return_type,
                    });
                }
                
                self.context.set_class(ClassDefinition {
                    name: class.name.clone(),
                    parent: class.extends.clone(),
                    properties,
                    methods: methods.clone(),
                });

                // check method bodies
                for method in &class.methods {
                    // create new context for method body
                    let mut method_context = TypeContext::with_parent(Box::new(self.context.clone()));
                    
                    // register 'self'
                    method_context.set_variable_type("self".to_string(), RuntimeType::Object(class.name.clone()), false);
                    
                    // register parameters in method context
                    for param in &method.parameters {
                        let param_type = if param.name == "self" {
                            // self is always the class type
                            RuntimeType::Object(class.name.clone())
                        } else if let Some(annotated) = &param.type_annotation {
                            self.ast_type_to_runtime_type(annotated)
                        } else {
                            RuntimeType::Void
                        };
                        method_context.set_variable_type(param.name.clone(), param_type, false);
                    }
                    
                    // check method body
                    let old_context = std::mem::replace(&mut self.context, method_context);
                    self.check_statement(&method.body)?;
                    self.context = old_context;
                }
                
                Ok(RuntimeType::Void)
            }

            Node::TypeDeclaration(decl) => {
                // Register type definition
                let mut variants_map = HashMap::new();
                
                for variant in &decl.variants {
                    let mut param_types = Vec::new();
                    let mut field_types = Vec::new();
                    
                    for field in &variant.fields {
                        // field is (name, type_name)
                        // Use ClassType as a carrier for named types (including variants)
                        let type_node = crate::ast::Type::ClassType(field.1.clone());
                        let field_type = self.ast_type_to_runtime_type(&type_node);
                        param_types.push((field.0.clone(), field_type.clone()));
                        field_types.push((field.0.clone(), field_type));
                    }
                    
                    variants_map.insert(variant.name.clone(), field_types);
                    
                    // Register constructor function: VariantName(fields) -> TypeName
                    self.context.set_function_signature(FunctionSignature {
                        name: variant.name.clone(),
                        parameters: param_types,
                        return_type: RuntimeType::Variant(decl.name.clone()),
                    });
                }
                
                self.context.add_type(TypeDefinition {
                    name: decl.name.clone(),
                    variants: variants_map,
                });
                
                Ok(RuntimeType::Void)
            }

            Node::InspectStatement(stmt) => {
                let expr_type = self.check_expression(&stmt.expression)?;
                
                if let RuntimeType::Variant(type_name) = expr_type {
                    // Verify that type_name exists
                    if let Some(type_def) = self.context.get_type_def(&type_name).cloned() {
                        // Check for exhaustiveness (TODO: implement exhaustiveness check)
                        
                        for case in &stmt.cases {
                            // Create scope for case body
                            let mut case_context = TypeContext::with_parent(Box::new(self.context.clone()));
                            
                            // Bind fields from variant to variables in scope
                            if let Some(fields) = type_def.variants.get(&case.variant_name) {
                                for (field_name, field_type) in fields {
                                    case_context.set_variable_type(field_name.clone(), field_type.clone(), false);
                                }
                            } else {
                                self.add_error(&stmt.expression.location(), format!(
                                    "Variant '{}' not found in type '{}'",
                                    case.variant_name, type_name
                                ));
                            }
                            
                            let old_context = std::mem::replace(&mut self.context, case_context);
                            self.check_statement(&case.body)?;
                            self.context = old_context;
                        }
                    } else {
                        self.add_error(&stmt.expression.location(), format!(
                            "Unknown type '{}'", type_name
                        ));
                    }
                } else {
                    self.add_error(&stmt.expression.location(), format!(
                        "inspect statement requires a variant type, got {}",
                        self.type_to_string(&expr_type)
                    ));
                }
                Ok(RuntimeType::Void)
            }
            
            Node::StructDeclaration(struct_decl) => {
                // register struct type (treated as class without methods)
                let mut properties = HashMap::new();
                for prop in &struct_decl.properties {
                    let prop_type = self.ast_type_to_runtime_type(&prop.type_annotation);
                    properties.insert(prop.name.clone(), prop_type);
                }
                
                
                self.context.set_class(ClassDefinition {
                    name: struct_decl.name.clone(),
                    parent: None,
                    properties,
                    methods: HashMap::new(),
                });
                
                Ok(RuntimeType::Void)
            }
            
            Node::TryCatchStatement(try_catch) => {
                self.check_statement(&try_catch.try_block)?;
                
                if let Some(catch_block) = &try_catch.catch_block {
                    // create new context for catch block
                    let mut catch_context = TypeContext::with_parent(Box::new(self.context.clone()));
                    
                    // register error variable if present
                    if let Some(error_var) = &try_catch.error_variable {
                        // error is usually a string message
                        catch_context.set_variable_type(error_var.clone(), RuntimeType::String, false);
                    }
                    
                    let old_context = std::mem::replace(&mut self.context, catch_context);
                    self.check_statement(catch_block)?;
                    self.context = old_context;
                }
                
                Ok(RuntimeType::Void)
            }
            
            Node::SetStatement(set_stmt) => {
                // check object is an object
                let obj_type = self.check_expression(&set_stmt.object)?;
                
                if let RuntimeType::Object(class_name) = obj_type {
                    // check property exists and type matches
                    if let Some(prop_type) = self.find_property_in_class(&class_name, &set_stmt.property) {
                        let val_type = self.check_expression(&set_stmt.value)?;
                        if !self.is_compatible(&val_type, &prop_type) {
                            self.add_error(&set_stmt.location, format!(
                                "Cannot assign {} to property '{}' of type {}",
                                self.type_to_string(&val_type),
                                set_stmt.property,
                                self.type_to_string(&prop_type)
                            ));
                        }
                    } else {
                        self.add_error(&set_stmt.location, format!("Class '{}' has no property '{}'", class_name, set_stmt.property));
                    }
                } else if let RuntimeType::Dictionary { key: _, value } = obj_type {
                    let val_type = self.check_expression(&set_stmt.value)?;
                    if !self.is_compatible(&val_type, &value) {
                        self.add_error(&set_stmt.location, format!(
                            "Cannot assign {} to dictionary value of type {}",
                            self.type_to_string(&val_type),
                            self.type_to_string(&value)
                        ));
                    }
                } else if !matches!(obj_type, RuntimeType::Any) {
                    self.add_error(&set_stmt.location, format!("Cannot set property on non-object type {}", self.type_to_string(&obj_type)));
                }
                
                Ok(RuntimeType::Void)
            }
            
            Node::ThrowStatement(throw) => {
                self.check_expression(&throw.expression)?;
                Ok(RuntimeType::Void)
            }
            
            _ => Ok(RuntimeType::Void),
        }
    }
    
    fn check_expression(&mut self, expr: &Node) -> Result<RuntimeType, Vec<TypeError>> {
        match expr {
            Node::FunctionDeclaration(func) => {
                // create new context for function body
                let mut func_context = TypeContext::with_parent(Box::new(self.context.clone()));
                
                // register parameters in function context
                let mut param_types = Vec::new();
                for param in &func.parameters {
                    let param_type = if let Some(annotated) = &param.type_annotation {
                        self.ast_type_to_runtime_type(annotated)
                    } else {
                        RuntimeType::Any // Default to Any for lambdas
                    };
                    param_types.push(param_type.clone());
                    func_context.set_variable_type(param.name.clone(), param_type, false);
                }
                
                // check function body in its own context
                let old_context = std::mem::replace(&mut self.context, func_context);
                let body_type = self.check_statement(&func.body)?;
                self.context = old_context;
                
                // infer return type
                let return_type = if let Some(annotated_return) = &func.return_type {
                    self.ast_type_to_runtime_type(annotated_return)
                } else {
                    body_type
                };
                
                Ok(RuntimeType::Function {
                    parameters: param_types,
                    return_type: Box::new(return_type),
                })
            }

            Node::ObjectCreation(obj) => {
                // check class exists
                if self.context.get_class(&obj.class_name).is_none() {
                    return Err(vec![TypeError {
                        location: obj.location.clone(),
                        message: format!("Unknown class '{}'", obj.class_name),
                    }]);
                }
                
                // TODO: validate properties match class definition
                Ok(RuntimeType::Object(obj.class_name.clone()))
            }
            
            Node::MethodCall(method_call) => {
                // check object type and method exists
                let obj_type = self.check_expression(&method_call.object)?;
                
                if let RuntimeType::Object(class_name) = obj_type {
                    if let Some(method_sig) = self.find_method_in_class(&class_name, &method_call.method_name) {
                        // TODO: validate arguments
                        return Ok(method_sig.return_type.clone());
                    } else {
                        return Err(vec![TypeError {
                            location: method_call.location.clone(),
                            message: format!("Class '{}' has no method '{}'", class_name, method_call.method_name),
                        }]);
                    }
                } else if let RuntimeType::List(elem_type) = obj_type {
                    match method_call.method_name.as_str() {
                        "add" => {
                            // add(item)
                            // TODO: check argument count and type
                            return Ok(RuntimeType::Void);
                        }
                        "remove" => {
                            // remove(index)
                            return Ok(RuntimeType::Void);
                        }
                        "size" => {
                            return Ok(RuntimeType::Number);
                        }
                        _ => {
                            return Err(vec![TypeError {
                                location: method_call.location.clone(),
                                message: format!("List has no method '{}'", method_call.method_name),
                            }]);
                        }
                    }
                }
                // For now, allow dynamic dispatch on non-objects? No, that's unsafe.
                // But we might have Any type.
                if matches!(obj_type, RuntimeType::Any) {
                    return Ok(RuntimeType::Any);
                }
                
                Err(vec![TypeError {
                    location: method_call.location.clone(),
                    message: format!("Cannot call method '{}' on type {:?}", method_call.method_name, obj_type),
                }])
            }
            Node::LiteralExpression(lit) => {
                Ok(match &lit.value {
                    LiteralValue::Number(_) => RuntimeType::Number,
                    LiteralValue::String(_) => RuntimeType::String,
                    LiteralValue::Bool(_) => RuntimeType::Bool,
                    LiteralValue::Void => RuntimeType::Void,
                    LiteralValue::Nothing => RuntimeType::Nothing,
                })
            }
            
            Node::VariableExpression(var) => {
                if let Some(var_type) = self.context.get_variable_type(&var.identifier) {
                    Ok(var_type)
                } else if let Some(sig) = self.context.get_function_signature(&var.identifier) {
                    // It's a function used as a value
                    Ok(RuntimeType::Function {
                        parameters: sig.parameters.iter().map(|(_, t)| t.clone()).collect(),
                        return_type: Box::new(sig.return_type.clone()),
                    })
                } else {
                    // Return error instead of Void
                    Err(vec![TypeError {
                        location: var.location.clone(),
                        message: format!("Variable '{}' not found", var.identifier),
                    }])
                }
            }
            
            Node::CallExpression(call) => {
                // special handling for built-in list creation
                // special handling for built-in list creation
                if call.function_name == "create_list" {
                    // infer list type from first element
                    let elem_type = if !call.arguments.is_empty() {
                        let t = self.check_expression(&call.arguments[0])?;
                        t
                    } else {
                        RuntimeType::Void // empty list (generic)
                    };
                    
                    // verify all elements have compatible types
                    for arg in &call.arguments {
                        let arg_type = self.check_expression(arg)?;
                        if !self.is_compatible(&arg_type, &elem_type) && !self.is_compatible(&elem_type, &arg_type) {
                             // warn or error? For now, allow mixed types (Void)
                        }
                    }
                    
                    return Ok(RuntimeType::List(Box::new(elem_type)));
                }
                
                if call.function_name == "create_dictionary" {
                    // infer value type from arguments (indices 1, 3, 5...)
                    let mut value_type = RuntimeType::Any;
                    
                    if call.arguments.len() >= 2 {
                        // Start with first value's type
                        if let Ok(t) = self.check_expression(&call.arguments[1]) {
                            value_type = t;
                        }
                        
                        // Check other values and find common type
                        let mut i = 3;
                        while i < call.arguments.len() {
                            if let Ok(next_val_type) = self.check_expression(&call.arguments[i]) {
                                if !self.is_compatible(&next_val_type, &value_type) {
                                    if self.is_compatible(&value_type, &next_val_type) {
                                        value_type = next_val_type; // Upgrade to more general type
                                    } else {
                                        value_type = RuntimeType::Any; // Fallback to Any
                                    }
                                }
                            }
                            i += 2;
                        }
                    }

                    return Ok(RuntimeType::Dictionary {
                        key: Box::new(RuntimeType::String),
                        value: Box::new(value_type)
                    });
                }

                // check for dot notation in function name (e.g. mu.add)
                if call.function_name.contains('.') {
                    let parts: Vec<&str> = call.function_name.split('.').collect();
                    if parts.len() == 2 {
                        let obj_name = parts[0];
                        let _method_name = parts[1];
                        
                        // println!("DEBUG: Checking dot call: {} on {}", call.function_name, obj_name);
                        if let Some(obj_type) = self.context.get_variable_type(obj_name) {
                            if matches!(obj_type, RuntimeType::Any) {
                                // Dynamic dispatch on Any - allow it
                                // Validate arguments (check them but ignore types)
                                for arg in &call.arguments {
                                    self.check_expression(arg)?;
                                }
                                return Ok(RuntimeType::Any);
                            } else if let RuntimeType::Dictionary { key: _, value } = obj_type {
                                // Allow method calls on dictionaries (e.g. imports)
                                // The value type is likely Any or Function
                                for arg in &call.arguments {
                                    self.check_expression(arg)?;
                                }
                                return Ok(*value.clone());
                            }
                        } else {
                             // println!("DEBUG: Object {} not found in context", obj_name);
                        }
                    }
                }

                // check function exists
                if let Some(sig) = self.context.get_function_signature(&call.function_name) {
                    // check argument count matches
                    if call.arguments.len() != sig.parameters.len() {
                        self.add_error(&call.location, format!(
                            "function '{}' expects {} arguments but got {}",
                            call.function_name,
                            sig.parameters.len(),
                            call.arguments.len()
                        ));
                    }
                    
                    // check argument types
                    for (i, arg) in call.arguments.iter().enumerate() {
                        let arg_type = self.check_expression(arg)?;
                        if i < sig.parameters.len() {
                            let expected_type = &sig.parameters[i].1;
                            if !self.is_compatible(&arg_type, expected_type) {
                                self.add_error(&arg.location(), format!(
                                    "Argument {} of '{}' expects {}, got {}",
                                    i + 1,
                                    call.function_name,
                                    self.type_to_string(expected_type),
                                    self.type_to_string(&arg_type)
                                ));
                            }
                        }
                    }
                    
                    Ok(sig.return_type.clone())
                } else if let Some(var_type) = self.context.get_variable_type(&call.function_name) {
                    // Check if variable is a function
                    if let RuntimeType::Function { parameters, return_type } = var_type {
                        // check argument count matches
                        if call.arguments.len() != parameters.len() {
                            self.add_error(&call.location, format!(
                                "function variable '{}' expects {} arguments but got {}",
                                call.function_name,
                                parameters.len(),
                                call.arguments.len()
                            ));
                        }
                        
                        // check argument types
                        for (i, arg) in call.arguments.iter().enumerate() {
                            let arg_type = self.check_expression(arg)?;
                            if i < parameters.len() {
                                let expected_type = &parameters[i];
                                if !self.is_compatible(&arg_type, expected_type) {
                                    self.add_error(&arg.location(), format!(
                                        "Argument {} of '{}' expects {}, got {}",
                                        i + 1,
                                        call.function_name,
                                        self.type_to_string(expected_type),
                                        self.type_to_string(&arg_type)
                                    ));
                                }
                            }
                        }
                        
                        Ok(*return_type)
                    } else if matches!(var_type, RuntimeType::Any) {
                         // Allow calling Any
                         Ok(RuntimeType::Any)
                    } else {
                        Err(vec![TypeError {
                            location: call.location.clone(),
                            message: format!("Variable '{}' is not a function (type: {})", call.function_name, self.type_to_string(&var_type)),
                        }])
                    }
                } else {
                    Err(vec![TypeError {
                        location: call.location.clone(),
                        message: format!("Unknown function or variable '{}'", call.function_name),
                    }])
                }
            }
            
            Node::OperationExpression(op) => {
                self.check_operation(op)
            }
            
            Node::ConditionalExpression(cond) => {
                let cond_type = self.check_expression(&cond.condition)?;
                if !self.is_boolean_compatible(&cond_type) {
                    self.add_error(&cond.condition.location(), format!(
                        "condition must be boolean, got {}",
                        self.type_to_string(&cond_type)
                    ));
                }
                
                let then_type = self.check_expression(&cond.then_expr)?;
                let else_type = self.check_expression(&cond.else_expr)?;
                
                if self.is_compatible(&then_type, &else_type) {
                    Ok(then_type)
                } else if self.is_compatible(&else_type, &then_type) {
                    Ok(else_type)
                } else {
                    self.add_error(&cond.location, format!(
                        "if branches have incompatible types: {} and {}",
                        self.type_to_string(&then_type),
                        self.type_to_string(&else_type)
                    ));
                    Ok(RuntimeType::Void)
                }
            }
            
            Node::AccessExpression(access) => {
                let obj_type = self.check_expression(&access.object)?;
                
                if let RuntimeType::Object(class_name) = &obj_type {
                    if let Some(prop_type) = self.find_property_in_class(class_name, &access.property) {
                        return Ok(prop_type);
                    } else {
                        return Err(vec![TypeError {
                            location: access.location.clone(),
                            message: format!("Class '{}' has no property '{}'", class_name, access.property),
                        }]);
                    }
                }
                
                if let RuntimeType::List(_) = &obj_type {
                     if matches!(access.property.as_str(), "size" | "length" | "count") {
                         return Ok(RuntimeType::Number);
                     } else {
                         return Err(vec![TypeError {
                             location: access.location.clone(),
                             message: format!("List has no property '{}'", access.property),
                         }]);
                     }
                }

                // Allow property access on Dictionary (treat as string key lookup)
                if let RuntimeType::Dictionary { key, value } = &obj_type {
                    if matches!(access.property.as_str(), "size" | "length" | "count") {
                         return Ok(RuntimeType::Number);
                    }
                    if self.is_compatible(&RuntimeType::String, key) {
                        return Ok(*value.clone());
                    }
                }
                
                // Allow access on Any type
                if matches!(obj_type, RuntimeType::Any) {
                    return Ok(RuntimeType::Any);
                }
                
                Err(vec![TypeError {
                    location: access.location.clone(),
                    message: format!("Cannot access property '{}' on type {:?}", access.property, obj_type),
                }])
            }
            
            Node::IndexExpression(index) => {
                let obj_type = self.check_expression(&index.object)?;
                let index_type = self.check_expression(&index.index)?;
                
                match obj_type {
                    RuntimeType::List(elem_type) => {
                        if !matches!(index_type, RuntimeType::Number) {
                            self.add_error(&index.index.location(), format!(
                                "List index must be a number, got {}",
                                self.type_to_string(&index_type)
                            ));
                        }
                        Ok(*elem_type)
                    }
                    RuntimeType::Dictionary { key: key_type, value: val_type } => {
                        if !self.is_compatible(&index_type, &key_type) {
                            self.add_error(&index.index.location(), format!(
                                "Dictionary key must be {}, got {}",
                                self.type_to_string(&key_type),
                                self.type_to_string(&index_type)
                            ));
                        }
                        Ok(*val_type)
                    }
                    RuntimeType::Any => Ok(RuntimeType::Any),
                    _ => {
                        Err(vec![TypeError {
                            location: index.location.clone(),
                            message: format!("Type {:?} is not indexable", obj_type),
                        }])
                    }
                }
            }
            
            _ => Ok(RuntimeType::Void),
        }
    }
    
    fn check_operation(&mut self, op: &OperationExpression) -> Result<RuntimeType, Vec<TypeError>> {
        let left_type = self.check_expression(&op.left)?;
        
        // handle unary operators
        if matches!(op.operator, Operator::Not) {
            if !self.is_boolean_compatible(&left_type) {
                let left_loc = op.left.location();
                self.add_error(&left_loc, format!(
                    "not operator requires boolean operand, got {}",
                    self.type_to_string(&left_type)
                ));
            }
            return Ok(RuntimeType::Bool);
        }

        if matches!(op.operator, Operator::Exists) {
            return Ok(RuntimeType::Bool);
        }
        
        // handle binary operators
        if let Some(right) = &op.right {
            let right_type = self.check_expression(right)?;
            
            match op.operator {
                Operator::Plus => {
                    // number + number = number
                    // text + text = text
                    // number + text = text (coercion)
                    if matches!(left_type, RuntimeType::Any) || matches!(right_type, RuntimeType::Any) {
                        Ok(RuntimeType::Any)
                    } else if matches!(left_type, RuntimeType::Number) && matches!(right_type, RuntimeType::Number) {
                        Ok(RuntimeType::Number)
                    } else if matches!(left_type, RuntimeType::String) || matches!(right_type, RuntimeType::String) {
                        Ok(RuntimeType::String)
                    } else {
                        self.add_error(&op.location, format!(
                            "plus operator not applicable to {} and {}",
                            self.type_to_string(&left_type),
                            self.type_to_string(&right_type)
                        ));
                        Ok(RuntimeType::Void)
                    }
                }
                
                Operator::Minus | Operator::Times | Operator::DividedBy | Operator::Modulo => {
                    // arithmetic operations require numbers
                    if !self.is_compatible(&left_type, &RuntimeType::Number) {
                        let left_loc = op.left.location();
                self.add_error(&left_loc, format!(
                            "{} operator requires number on left, got {}",
                            self.op_to_string(&op.operator),
                            self.type_to_string(&left_type)
                        ));
                    }
                    if !self.is_compatible(&right_type, &RuntimeType::Number) {
                        let right_loc = right.location();
                        self.add_error(&right_loc, format!(
                            "{} operator requires number on right, got {}",
                            self.op_to_string(&op.operator),
                            self.type_to_string(&right_type)
                        ));
                    }
                    Ok(RuntimeType::Number)
                }
                
                Operator::Equals | Operator::NotEquals => {
                    // comparison returns boolean
                    // types should be compatible
                    if !self.is_compatible(&left_type, &right_type) {
                        self.add_error(&op.location, format!(
                            "cannot compare {} with {}",
                            self.type_to_string(&left_type),
                            self.type_to_string(&right_type)
                        ));
                    }
                    Ok(RuntimeType::Bool)
                }
                
                Operator::GreaterThan | Operator::LessThan | Operator::GreaterThanOrEqual | Operator::LessThanOrEqual => {
                    // comparison operators require numbers
                    if !self.is_compatible(&left_type, &RuntimeType::Number) {
                        let left_loc = op.left.location();
                        self.add_error(&left_loc, format!(
                            "comparison operator requires number on left, got {}",
                            self.type_to_string(&left_type)
                        ));
                    }
                    if !self.is_compatible(&right_type, &RuntimeType::Number) {
                        let right_loc = right.location();
                        self.add_error(&right_loc, format!(
                            "comparison operator requires number on right, got {}",
                            self.type_to_string(&right_type)
                        ));
                    }
                    Ok(RuntimeType::Bool)
                }
                
                Operator::And | Operator::Or => {
                    // logical operators return boolean
                    // operands should be boolean-compatible
                    if !self.is_boolean_compatible(&left_type) {
                        let left_loc = op.left.location();
                self.add_error(&left_loc, format!(
                            "logical operator requires boolean operand, got {}",
                            self.type_to_string(&left_type)
                        ));
                    }
                    if !self.is_boolean_compatible(&right_type) {
                        let right_loc = right.location();
                        self.add_error(&right_loc, format!(
                            "logical operator requires boolean operand, got {}",
                            self.type_to_string(&right_type)
                        ));
                    }
                    Ok(RuntimeType::Bool)
                }
                
                _ => Ok(RuntimeType::Void),
            }
        } else {
            self.add_error(&op.location, "binary operator missing right operand".to_string());
            Ok(RuntimeType::Void)
        }
    }
    
    // helper methods
    fn is_compatible(&self, from: &RuntimeType, to: &RuntimeType) -> bool {
        match (from, to) {
            // same types are compatible
            (a, b) if a == b => true,
            
            // nothing is compatible with everything (top type)
            (_, RuntimeType::Void) => true,
            (RuntimeType::Void, _) => true,
            
            // number can be coerced to text in some contexts
            (RuntimeType::Number, RuntimeType::String) => true,
            (RuntimeType::String, RuntimeType::Number) => false,
            
            // lists are compatible if element types are compatible
            (RuntimeType::List(a_inner), RuntimeType::List(b_inner)) => {
                self.is_compatible(a_inner, b_inner)
            }
            
            // dictionaries are compatible if key and value types are compatible
            (RuntimeType::Dictionary { key: k1, value: v1 }, RuntimeType::Dictionary { key: k2, value: v2 }) => {
                self.is_compatible(k1, k2) && self.is_compatible(v1, v2)
            }
            
            // objects are compatible if same class or inheritance
            (RuntimeType::Object(a_class), RuntimeType::Object(b_class)) => {
                a_class == b_class // TODO: check inheritance
            }
            
            (RuntimeType::Future(a_inner), RuntimeType::Future(b_inner)) => {
                self.is_compatible(a_inner, b_inner)
            }
            
            // maybe types
            // maybe types
            (RuntimeType::Nothing, RuntimeType::Maybe(_)) => true,
            (RuntimeType::Maybe(inner_from), RuntimeType::Maybe(inner_to)) => self.is_compatible(inner_from, inner_to),
            (t, RuntimeType::Maybe(inner)) => self.is_compatible(t, inner),
            
            // Any is compatible with everything (dynamic typing)
            (RuntimeType::Any, _) => true,
            (_, RuntimeType::Any) => true,
            
            _ => false,
        }
    }
    
    fn is_boolean_compatible(&self, _ty: &RuntimeType) -> bool {
        // All types have truthy/falsy semantics in Layman
        true
    }
    
    fn ast_type_to_runtime_type(&self, ast_type: &Type) -> RuntimeType {
        match ast_type {
            Type::BasicType(basic) => match basic {
                BasicType::Number => RuntimeType::Number,
                BasicType::String => RuntimeType::String,
                BasicType::Bool => RuntimeType::Bool,
                BasicType::Void => RuntimeType::Void,
                BasicType::Any => RuntimeType::Any,
            },
            Type::CompositeType(composite) => match composite {
                CompositeType::List(inner) => {
                    RuntimeType::List(Box::new(self.ast_type_to_runtime_type(inner)))
                }
                CompositeType::DictionaryType(key, value) => {
                    RuntimeType::Dictionary {
                        key: Box::new(self.ast_type_to_runtime_type(key)),
                        value: Box::new(self.ast_type_to_runtime_type(value)),
                    }
                }
                CompositeType::Maybe(inner) => {
                    RuntimeType::Maybe(Box::new(self.ast_type_to_runtime_type(inner)))
                }
                _ => RuntimeType::Void,
            },
            Type::ClassType(class_name) => {
                match class_name.as_str() {
                    "Number" => RuntimeType::Number,
                    "String" | "Text" => RuntimeType::String,
                    "Bool" | "Boolean" => RuntimeType::Bool,
                    "Void" | "Nothing" => RuntimeType::Void,
                    "Any" => RuntimeType::Any,
                    _ => {
                        if self.context.is_variant(class_name) {
                            RuntimeType::Variant(class_name.clone())
                        } else {
                            RuntimeType::Object(class_name.clone())
                        }
                    }
                }
            },
            _ => RuntimeType::Void,
        }
    }
    
    fn type_to_string(&self, ty: &RuntimeType) -> String {
        match ty {
            RuntimeType::Number => "number".to_string(),
            RuntimeType::String => "text".to_string(),
            RuntimeType::Bool => "boolean".to_string(),
            RuntimeType::Void => "nothing".to_string(),
            RuntimeType::List(inner) => format!("list of {}", self.type_to_string(inner)),
            RuntimeType::Dictionary { key, value } => {
                format!("dictionary from {} to {}", self.type_to_string(key), self.type_to_string(value))
            }
            RuntimeType::Function { parameters, return_type } => {
                let params_str = parameters.iter().map(|p| self.type_to_string(p)).collect::<Vec<_>>().join(", ");
                format!("function({}) -> {}", params_str, self.type_to_string(return_type))
            },
            RuntimeType::Class(class_name) => format!("class {}", class_name),
            RuntimeType::Object(class_name) => format!("{} object", class_name),
            RuntimeType::Future(inner) => format!("future of {}", self.type_to_string(inner)),
            RuntimeType::Variant(name) => name.clone(),
            RuntimeType::Maybe(inner) => format!("maybe {}", self.type_to_string(inner)),
            RuntimeType::Nothing => "nothing".to_string(),
            RuntimeType::Any => "any".to_string(),
        }
    }
    
    fn op_to_string(&self, op: &Operator) -> String {
        match op {
            Operator::Plus => "plus",
            Operator::Minus => "minus",
            Operator::Times => "times",
            Operator::DividedBy => "divided by",
            Operator::Modulo => "modulo",
            Operator::Equals => "equals",
            Operator::NotEquals => "not equals",
            Operator::GreaterThan => "greater than",
            Operator::LessThan => "less than",
            Operator::GreaterThanOrEqual => "greater than or equal",
            Operator::LessThanOrEqual => "less than or equal",
            Operator::And => "and",
            Operator::Or => "or",
            Operator::Not => "not",
            Operator::Exists => "exists",
        }
        .to_string()
    }
    
    fn add_error(&mut self, location: &Location, message: String) {
        self.errors.push(TypeError {
            location: location.clone(),
            message,
        });
    }

    fn find_property_in_class(&self, class_name: &str, prop_name: &str) -> Option<RuntimeType> {
        let mut current_class_name = Some(class_name.to_string());
        
        while let Some(name) = current_class_name {
            if let Some(class_def) = self.context.get_class(&name) {
                if let Some(prop_type) = class_def.properties.get(prop_name) {
                    return Some(prop_type.clone());
                }
                current_class_name = class_def.parent.clone();
            } else {
                break;
            }
        }
        None
    }

    fn find_method_in_class(&self, class_name: &str, method_name: &str) -> Option<FunctionSignature> {
        let mut current_class_name = Some(class_name.to_string());
        
        while let Some(name) = current_class_name {
            if let Some(class_def) = self.context.get_class(&name) {
                if let Some(method_sig) = class_def.methods.get(method_name) {
                    return Some(method_sig.clone());
                }
                current_class_name = class_def.parent.clone();
            } else {
                break;
            }
        }
        None
    }
    fn check_import(&mut self, stmt: &ImportStatement) -> Result<(), Vec<TypeError>> {
        // Resolve path relative to current file
        let current_file_path = Path::new(&stmt.location.file);
        let parent_dir = current_file_path.parent().unwrap_or(Path::new("."));
        
        let module_key = if stmt.is_file_path {
            stmt.module_name.clone()
        } else {
            format!("{}.lay", stmt.module_name)
        };
        
        let candidates = vec![
            parent_dir.join(&module_key),
            PathBuf::from("std").join(&module_key),
            std::env::current_dir().unwrap_or_default().join("std").join(&module_key),
        ];
        
        let mut path = PathBuf::new();
        let mut found = false;
        
        for candidate in candidates {
            if candidate.exists() {
                path = candidate;
                found = true;
                break;
            }
        }
        
        if !found {
            // If file doesn't exist, we can't check it.
            return Err(vec![TypeError {
                location: stmt.location.clone(),
                message: format!("Module not found: {}", module_key),
            }]);
        }
        
        // Read and parse
        let source = fs::read_to_string(path)
            .map_err(|e| vec![TypeError {
                location: stmt.location.clone(),
                message: format!("Failed to read module: {}", e),
            }])?;
            
        let mut lexer = lexer::Lexer::new(&source, module_key.clone());
        let tokens = lexer.tokenize()
            .map_err(|e| vec![TypeError {
                location: stmt.location.clone(),
                message: format!("Lexer error: {}", e),
            }])?;
            
        let mut parser = parser::Parser::new(tokens);
        let ast = parser.parse()
            .map_err(|e| vec![TypeError {
                location: stmt.location.clone(),
                message: format!("Parser error: {}", e),
            }])?;
            
        // Check module in new context
        let mut module_checker = TypeChecker::new();
        
        if let Node::Program(prog) = ast {
            if let Err(errors) = module_checker.check_program(&prog) {
                // If module has errors, fail
                return Err(errors);
            }
            
            // Extract exports from module_checker.context
            // Bind to current context based on import stmt
            
            if !stmt.specific_imports.is_empty() {
                for name in &stmt.specific_imports {
                    if let Some(ty) = module_checker.context.get_variable_type(name) {
                        self.context.set_variable_type(name.clone(), ty, false);
                    } else if let Some(sig) = module_checker.context.get_function_signature(name) {
                         // Function signature needs to be registered
                         self.context.set_function_signature(sig);
                    } else {
                        // Warn or error?
                        // For now, assume Any/Void to avoid blocking
                        self.context.set_variable_type(name.clone(), RuntimeType::Any, false);
                    }
                }
            } else if let Some(alias) = &stmt.alias {
                // import ... as alias
                self.context.set_variable_type(alias.clone(), RuntimeType::Any, false);
            } else {
                // import ... (no alias)
                 let var_name = if stmt.is_file_path {
                    Path::new(&stmt.module_name).file_stem().unwrap().to_string_lossy().to_string()
                } else {
                    stmt.module_name.clone()
                };
                self.context.set_variable_type(var_name, RuntimeType::Any, false);
            }
        }
        
        Ok(())
    }
}

// helper trait for getting location from nodes
trait LocationExt {
    fn location(&self) -> Location;
}

impl LocationExt for Node {
    fn location(&self) -> Location {
        match self {
            Node::LiteralExpression(lit) => lit.location.clone(),
            Node::VariableExpression(var) => var.location.clone(),
            Node::CallExpression(call) => call.location.clone(),
            Node::OperationExpression(op) => op.location.clone(),
            Node::ConditionalExpression(cond) => cond.location.clone(),
            Node::AccessExpression(acc) => acc.location.clone(),
            Node::IndexExpression(idx) => idx.location.clone(),
            _ => Location {
                file: String::new(),
                line: 0,
                column: 0,
                source: None,
            },
        }
    }


}

