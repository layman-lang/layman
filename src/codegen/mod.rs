// code generation - convert AST to target languages
// this is REAL compilation, not just AST serialization

use crate::ast::*;

pub struct CodeGenerator;

impl CodeGenerator {
    pub fn new() -> Self {
        Self
    }

    /// generate javascript code from AST
    pub fn generate_javascript(&self, ast: &Node) -> Result<String, String> {
        let mut output = String::new();
        
        // add runtime helper functions
        output.push_str("// Layman compiled to JavaScript\n");
        output.push_str("// Generated code - do not edit directly\n\n");
        
        output.push_str("// runtime helpers\n");
        output.push_str("function lay_print(value) { console.log(String(value)); }\n");
        output.push_str("function lay_concatenate(a, b) { return String(a) + String(b); }\n");
        output.push_str("function lay_convert_to_text(value) { return String(value); }\n\n");
        
        // generate code from program
        match ast {
            Node::Program(prog) => {
                for stmt in &prog.statements {
                    self.generate_statement_js(&mut output, stmt)?;
                }
            }
            _ => return Err("Expected Program node".to_string()),
        }
        
        Ok(output)
    }

    fn generate_statement_js(&self, output: &mut String, stmt: &Node) -> Result<(), String> {
        match stmt {
            Node::AssignStatement(assign) => {
                let var_name = self.sanitize_identifier(&assign.identifier);
                output.push_str(&format!("let {} = ", var_name));
                self.generate_expression_js(output, &assign.expression)?;
                output.push_str(";\n");
            }
            Node::DeclareStatement(decl) => {
                let var_name = self.sanitize_identifier(&decl.identifier);
                if let Some(init) = &decl.initial_value {
                    output.push_str(&format!("let {} = ", var_name));
                    self.generate_expression_js(output, init)?;
                    output.push_str(";\n");
                } else {
                    output.push_str(&format!("let {};\n", var_name));
                }
            }
            Node::CallExpression(call) => {
                // handle print statements (parsed as CallExpression directly)
                if call.function_name == "print" {
                    if let Some(first_arg) = call.arguments.first() {
                        output.push_str("lay_print(");
                        self.generate_expression_js(output, first_arg)?;
                        output.push_str(");\n");
                    }
                } else {
                    // other function calls
                    self.generate_expression_js(output, stmt)?;
                    output.push_str(";\n");
                }
            }
            Node::ExpressionStatement(expr_stmt) => {
                // check if this is a print statement
                if let Node::CallExpression(call) = &*expr_stmt.expression {
                    if call.function_name == "print" {
                        if let Some(first_arg) = call.arguments.first() {
                            output.push_str("lay_print(");
                            self.generate_expression_js(output, first_arg)?;
                            output.push_str(");\n");
                            return Ok(());
                        }
                    }
                }
                self.generate_expression_js(output, &expr_stmt.expression)?;
                output.push_str(";\n");
            }
            // print statements are handled as expression statements with special call
            Node::ReturnStatement(ret) => {
                output.push_str("return");
                if let Some(expr) = &ret.expression {
                    output.push_str(" ");
                    self.generate_expression_js(output, expr)?;
                }
                output.push_str(";\n");
            }
            Node::FunctionDeclaration(func) => {
                output.push_str(&format!("function {}(", self.sanitize_identifier(&func.name)));
                for (i, param) in func.parameters.iter().enumerate() {
                    if i > 0 {
                        output.push_str(", ");
                    }
                    output.push_str(&self.sanitize_identifier(&param.name));
                }
                output.push_str(") {\n");
                self.generate_statement_js(output, &func.body)?;
                output.push_str("}\n");
            }
            Node::ConditionalStatement(cond) => {
                output.push_str("if (");
                self.generate_expression_js(output, &cond.condition)?;
                output.push_str(") {\n");
                self.generate_statement_js(output, &cond.then_branch)?;
                if let Some(else_branch) = &cond.else_branch {
                    output.push_str("} else {\n");
                    self.generate_statement_js(output, else_branch)?;
                }
                output.push_str("}\n");
            }
            Node::LoopStatement(loop_stmt) => {
                match &loop_stmt.loop_type {
                    LoopType::ForEach => {
                        if let (Some(iter), Some(coll)) = (&loop_stmt.iterator, &loop_stmt.collection) {
                            output.push_str(&format!("for (let {} of ", self.sanitize_identifier(iter)));
                            self.generate_expression_js(output, coll)?;
                            output.push_str(") {\n");
                            self.generate_statement_js(output, &loop_stmt.body)?;
                            output.push_str("}\n");
                        }
                    }
                    LoopType::While => {
                        if let Some(condition) = &loop_stmt.condition {
                            output.push_str("while (");
                            self.generate_expression_js(output, condition)?;
                            output.push_str(") {\n");
                            self.generate_statement_js(output, &loop_stmt.body)?;
                            output.push_str("}\n");
                        }
                    }
                    LoopType::Repeat => {
                        // repeat...until not yet supported in JS generation
                        output.push_str("// repeat loop (not yet implemented in JS generation)\n");
                    }
                }
            }
            Node::ImportStatement(_) => {
                // imports are resolved and bundled before code generation
                // so we don't need to generate import statements here
                // output.push_str("// import statement (bundled at compile time)\n");
            }
            Node::ClassDeclaration(_) => {
                // classes not yet fully supported in JS output - skip silently
                // output.push_str("// class declaration (not yet fully implemented)\n");
            }
            Node::ThrowStatement(throw) => {
                output.push_str("throw ");
                self.generate_expression_js(output, &throw.expression)?;
                output.push_str(";\n");
            }
            Node::TryCatchStatement(try_catch) => {
                output.push_str("try {\n");
                self.generate_statement_js(output, &try_catch.try_block)?;
                output.push_str("} catch (");
                if let Some(error_var) = &try_catch.error_variable {
                    output.push_str(&self.sanitize_identifier(error_var));
                } else {
                    output.push_str("error");
                }
                output.push_str(") {\n");
                if let Some(catch_block) = &try_catch.catch_block {
                    self.generate_statement_js(output, catch_block)?;
                }
                output.push_str("}\n");
            }
            Node::Program(_) => {
                // nested programs shouldn't occur here
            }
            _ => {
                // skip other node types silently (they may be handled elsewhere)
            }
        }
        Ok(())
    }

    fn generate_expression_js(&self, output: &mut String, expr: &Node) -> Result<(), String> {
        match expr {
            Node::ExpressionStatement(expr_stmt) => {
                // unwrap expression statement - just generate the inner expression
                self.generate_expression_js(output, &expr_stmt.expression)?
            }
            Node::LiteralExpression(lit) => {
                match &lit.value {
                    LiteralValue::Number(n) => output.push_str(&n.to_string()),
                    LiteralValue::String(s) => {
                        output.push('"');
                        output.push_str(&self.escape_string(s));
                        output.push('"');
                    }
                    LiteralValue::Bool(b) => output.push_str(if *b { "true" } else { "false" }),
                    LiteralValue::Void => output.push_str("undefined"),
                }
            }
            Node::VariableExpression(var) => {
                output.push_str(&self.sanitize_identifier(&var.identifier));
            }
            Node::CallExpression(call) => {
                // handle special standard library functions
                match call.function_name.as_str() {
                    "concatenate" => {
                        output.push_str("lay_concatenate(");
                        if call.arguments.len() >= 2 {
                            self.generate_expression_js(output, &call.arguments[0])?;
                            output.push_str(", ");
                            self.generate_expression_js(output, &call.arguments[1])?;
                        }
                        output.push_str(")");
                    }
                    name if name.contains("concatenate") || name.contains("with") => {
                        // handle "concatenate X with Y" - check arguments
                        output.push_str("lay_concatenate(");
                        if call.arguments.len() >= 2 {
                            self.generate_expression_js(output, &call.arguments[0])?;
                            output.push_str(", ");
                            self.generate_expression_js(output, &call.arguments[1])?;
                        } else if call.arguments.len() == 1 {
                            // might be "concatenate X with Y" parsed differently
                            self.generate_expression_js(output, &call.arguments[0])?;
                            output.push_str(", \"\")");
                        } else {
                            output.push_str("\"\", \"\")");
                        }
                        output.push_str(")");
                    }
                    "convert" | "convert to text" => {
                        output.push_str("lay_convert_to_text(");
                        if !call.arguments.is_empty() {
                            self.generate_expression_js(output, &call.arguments[0])?;
                        }
                        output.push_str(")");
                    }
                    "expect" => {
                        // expect(actual, expected) - assertion
                        output.push_str("(function() { const actual = ");
                        if !call.arguments.is_empty() {
                            self.generate_expression_js(output, &call.arguments[0])?;
                        }
                        output.push_str("; const expected = ");
                        if call.arguments.len() >= 2 {
                            self.generate_expression_js(output, &call.arguments[1])?;
                        }
                        output.push_str("; if (String(actual) !== String(expected)) { throw new Error('expected ' + String(expected) + ' but got ' + String(actual)); } })()");
                    }
                    "test" => {
                        // test(name, body) - test case
                        output.push_str("(function() { const testName = ");
                        if !call.arguments.is_empty() {
                            self.generate_expression_js(output, &call.arguments[0])?;
                        }
                        output.push_str("; console.log('running test: ' + testName); try { ");
                        if call.arguments.len() >= 2 {
                            self.generate_statement_js(output, &call.arguments[1])?;
                        }
                        output.push_str(" console.log('test passed: ' + testName); } catch (e) { console.log('test failed: ' + testName + ' - ' + e.message); throw e; } })()");
                    }
                    "describe" => {
                        // describe(name, body) - test suite
                        output.push_str("(function() { const suiteName = ");
                        if !call.arguments.is_empty() {
                            self.generate_expression_js(output, &call.arguments[0])?;
                        }
                        output.push_str("; console.log('test suite: ' + suiteName); try { ");
                        if call.arguments.len() >= 2 {
                            self.generate_statement_js(output, &call.arguments[1])?;
                        }
                        output.push_str(" console.log('suite completed: ' + suiteName); } catch (e) { throw e; } })()");
                    }
                    _ => {
                        output.push_str(&self.sanitize_identifier(&call.function_name));
                        output.push_str("(");
                        for (i, arg) in call.arguments.iter().enumerate() {
                            if i > 0 {
                                output.push_str(", ");
                            }
                            self.generate_expression_js(output, arg)?;
                        }
                        output.push_str(")");
                    }
                }
            }
            Node::OperationExpression(op) => {
                match &op.operator {
                    Operator::Not => {
                        output.push_str("!");
                        if let Some(right) = &op.right {
                            self.generate_expression_js(output, right)?;
                        }
                    }
                    _ => {
                        self.generate_expression_js(output, &op.left)?;
                        output.push_str(match &op.operator {
                            Operator::Plus => " + ",
                            Operator::Minus => " - ",
                            Operator::Times => " * ",
                            Operator::DividedBy => " / ",
                            Operator::Equals => " === ",
                            Operator::LessThan => " < ",
                            Operator::GreaterThan => " > ",
                            Operator::And => " && ",
                            Operator::Or => " || ",
                            _ => return Err(format!("Unsupported operator in JS generation: {:?}", op.operator)),
                        });
                        if let Some(right) = &op.right {
                            self.generate_expression_js(output, right)?;
                        } else {
                            return Err("Binary operator missing right operand".to_string());
                        }
                    }
                }
            }
            Node::ObjectCreation(obj) => {
                // object creation - create a JS object literal
                output.push_str("({");
                for (i, prop) in obj.properties.iter().enumerate() {
                    if i > 0 {
                        output.push_str(", ");
                    }
                    output.push_str(&format!("{}: ", self.sanitize_identifier(&prop.name)));
                    self.generate_expression_js(output, &prop.value)?;
                }
                output.push_str("})");
            }
            Node::MethodCall(method) => {
                // method call - obj.method(args)
                self.generate_expression_js(output, &method.object)?;
                output.push_str(".");
                output.push_str(&self.sanitize_identifier(&method.method_name));
                output.push_str("(");
                for (i, arg) in method.arguments.iter().enumerate() {
                    if i > 0 {
                        output.push_str(", ");
                    }
                    self.generate_expression_js(output, arg)?;
                }
                output.push_str(")");
            }
            _ => {
                return Err(format!("Unsupported expression type in JS generation: {:?}", expr));
            }
        }
        Ok(())
    }

    fn sanitize_identifier(&self, ident: &str) -> String {
        // convert "user name" to "user_name" for JS compatibility
        ident.replace(" ", "_")
    }

    fn escape_string(&self, s: &str) -> String {
        s.replace("\\", "\\\\")
            .replace("\"", "\\\"")
            .replace("\n", "\\n")
            .replace("\r", "\\r")
            .replace("\t", "\\t")
    }
}

