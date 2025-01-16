// parser - builds AST from tokens
// parses pure english syntax into structured AST

use crate::ast::*;
use crate::lexer::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    
    pub fn parse(&mut self) -> Result<Node, String> {
        let mut statements = Vec::new();
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 100000; // prevent infinite loops
        
        while !self.is_at_end() {
            iterations += 1;
            if iterations > MAX_ITERATIONS {
                return Err("Parser exceeded maximum iterations - possible infinite loop".to_string());
            }
            
            if self.check(&TokenKind::Eof) {
                break;
            }
            
            // skip newlines at start
            self.skip_newlines();
            
            if self.is_at_end() || self.check(&TokenKind::Eof) {
                break;
            }
            
            let position_before = self.current;
            
            match self.parse_statement() {
                Ok(stmt) => {
                    // only add non-empty statements
                    if !matches!(&stmt, Node::ExpressionStatement(es) if 
                        matches!(*es.expression, Node::LiteralExpression(LiteralExpression { value: LiteralValue::Void, .. }))) {
                        statements.push(stmt);
                    }
                    
                    // ensure we advanced - if not, break to prevent infinite loop
                    if self.current == position_before && !self.is_at_end() {
                        // didn't advance, skip current token to prevent hang
                        if self.current < self.tokens.len() {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                Err(e) => {
                    // for now, continue on error to allow partial parsing
                    // ensure we advance to prevent infinite loop
                    if self.current == position_before && !self.is_at_end() {
                        if self.current < self.tokens.len() {
                            self.advance();
                        }
                    }
                    // break on parse error to avoid hanging
                    return Err(e);
                }
            }
        }
        
        // wrap in Program
        Ok(Node::Program(Program {
            location: if statements.is_empty() {
                self.current_token().location.clone()
            } else {
                // get location from first statement
                match &statements[0] {
                    Node::AssignStatement(a) => a.location.clone(),
                    Node::DeclareStatement(d) => d.location.clone(),
                    Node::ExpressionStatement(e) => e.location.clone(),
                    Node::FunctionDeclaration(f) => f.location.clone(),
                    Node::ClassDeclaration(c) => c.location.clone(),
                    Node::ReturnStatement(r) => r.location.clone(),
                    Node::ConditionalStatement(i) => i.location.clone(),
                    Node::LoopStatement(l) => l.location.clone(),
                    Node::ImportStatement(i) => i.location.clone(),
                    Node::TryCatchStatement(t) => t.location.clone(),
                    Node::ObjectCreation(o) => o.location.clone(),
                    Node::MethodCall(m) => m.location.clone(),
                    Node::UsingStatement(u) => u.location.clone(),
                    Node::IndexSetStatement(s) => s.location.clone(),
                    Node::SetStatement(s) => s.location.clone(),
                    Node::ThrowStatement(t) => t.location.clone(),
                    _ => self.current_token().location.clone(),
                }
            },
            statements,
        }))
    }
    
    fn parse_statement(&mut self) -> Result<Node, String> {
        // skip newlines
        self.skip_newlines();
        

        
        if self.is_at_end() || self.check(&TokenKind::Eof) {
            return Err("Unexpected end of input".to_string());
        }
        
        // CRITICAL: check for "define" and "import" FIRST, before checking for "the variable"
        // this prevents trying to parse top-level declarations as assignments
        // IMPORTANT: If we're at column 0, this MUST be a top-level declaration
        // In the main parser loop, we should parse it normally
        // In function body loops, the caller should check for column 0 before calling parse_statement()
        // But if parse_statement() is called with "define" at column 0, we should parse it (main parser context)
        if self.current_token().location.column == 0 {
            if self.check(&TokenKind::Define) {
                // This is a top-level declaration - parse it normally
                // The function body parsing loop should have broken before calling parse_statement(),
                // but if it didn't, we'll parse it here (which is fine for main parser)
                self.advance(); // consume "define"
                return self.parse_declaration_or_function();
            }
            if self.check(&TokenKind::Import) {
                // This is a top-level import - parse it normally
                self.advance(); // consume "import"
                return self.parse_import();
            }
            if self.check(&TokenKind::Struct) {
                // This is a top-level struct declaration
                let location = self.current_token().location.clone();
                self.advance(); // consume "struct"
                return self.parse_struct_declaration(location);
            }
        }
        
        // CRITICAL: Check for "return" BEFORE checking for "the variable"
        // This prevents trying to parse "return" as part of an assignment
        // return: "return X"
        if self.match_token(&[TokenKind::Return]) {
            return self.parse_return();
        }
        
        // CRITICAL: Check for "print" BEFORE checking for "the variable"
        // This prevents trying to parse "print the variable X" as an assignment
        // print statement: "print X" or "print the variable X"
        if let TokenKind::Identifier(ref s) = &self.current_token().kind {
            if s.to_lowercase().trim() == "print" {
                self.advance();
                return self.parse_print_statement();
            }
        }
        
        // ignore common pseudocode markers as no-ops
        if let TokenKind::Identifier(ref s) = &self.current_token().kind {
            let low = s.to_lowercase();
            if Self::is_noop_marker(&low) {
                // consume tokens until newline or eof
                while !self.is_at_end() && !self.check(&TokenKind::Newline) {
                    self.advance();
                }
                // produce an empty expression statement (filtered by parse loop)
                return Ok(Node::ExpressionStatement(ExpressionStatement {
                    location: self.current_token().location.clone(),
                    expression: Box::new(Node::LiteralExpression(LiteralExpression {
                        location: self.current_token().location.clone(),
                        value: LiteralValue::Void,
                    })),
                }));
            }
        }

        // check for unexpected "end" tokens
        if let TokenKind::Identifier(ref s) = &self.current_token().kind {
            let low = s.to_lowercase();
            if low == "end" || low == "endif" || low == "endfor" || low == "endfunction" || low == "done" {
                return Err(format!("Unexpected '{}'. Unmatched closing block or redundant end token.", s));
            }
        }

        // assignment: "the variable X is Y" (or "equals Y" for backward compatibility)
        if self.check(&TokenKind::The) {
            if let Some(kind) = self.peek_kind_skip_newlines(1) {
                if matches!(kind, TokenKind::Variable | TokenKind::Constant) {
                    // Before parsing assignment, check if the next meaningful token after "the variable/constant" is "return", "define", or "import"
                    // If so, this is not an assignment - it's likely a statement boundary
                    let mut peek_pos = self.current + 2; // after "the" and "variable/constant"
                    while peek_pos < self.tokens.len() {
                        let token = &self.tokens[peek_pos];
                        if matches!(token.kind, TokenKind::Newline) {
                            peek_pos += 1;
                            continue;
                        }
                        // Found non-newline token - check if it's a statement keyword
                        if matches!(token.kind, TokenKind::Return | TokenKind::Define | TokenKind::Import) {
                            // This is not an assignment - it's a statement boundary
                            // Skip this line and return void
                            while !self.is_at_end() && !self.check(&TokenKind::Newline) { self.advance(); }
                            return Ok(Node::ExpressionStatement(ExpressionStatement {
                                location: self.current_token().location.clone(),
                                expression: Box::new(Node::LiteralExpression(LiteralExpression { location: self.current_token().location.clone(), value: LiteralValue::Void })),
                            }));
                        }
                        // Found non-newline token that's not a statement keyword - stop peeking
                        break;
                    }
                    // consume 'the'
                    self.advance();
                    return self.parse_variable_declaration();
                } else {
                    // not a code assignment, skip line
                    while !self.is_at_end() && !self.check(&TokenKind::Newline) { self.advance(); }
                    return Ok(Node::ExpressionStatement(ExpressionStatement {
                        location: self.current_token().location.clone(),
                        expression: Box::new(Node::LiteralExpression(LiteralExpression { location: self.current_token().location.clone(), value: LiteralValue::Void })),
                    }));
                }
            }
        }
        
        // declaration: "define variable X as type" or "let X be type"
        if self.match_token(&[TokenKind::Define]) {
            return self.parse_declaration_or_function();
        }
        
        // explicit check for "call" statement
        if self.check(&TokenKind::Call) {
            return self.parse_call_statement();
        }
        
        // import: "import the module X"
        // import: "import ..." or "from ..."
        if self.match_token(&[TokenKind::Import, TokenKind::From]) {
            return self.parse_import();
        }
        
        // conditional: "if X then Y otherwise Z"
        if self.check(&TokenKind::If) {
            return self.parse_conditional();
        }
        
        // inspect: "inspect X case Y do Z"
        if self.check(&TokenKind::Inspect) {
            return self.parse_inspect_statement();
        }
        
        // loop: "for each X in Y do Z"
        if self.check(&TokenKind::For) {
            if let Some(kind) = self.peek_kind_skip_newlines(1) {
                if matches!(kind, TokenKind::Each) {
                    self.advance();
                    self.consume(&TokenKind::Each, "Expected 'each'")?;
                    return self.parse_for_loop();
                }
            }
            // not a real loop, skip line
            while !self.is_at_end() && !self.check(&TokenKind::Newline) { self.advance(); }
            return Ok(Node::ExpressionStatement(ExpressionStatement { location: self.current_token().location.clone(), expression: Box::new(Node::LiteralExpression(LiteralExpression { location: self.current_token().location.clone(), value: LiteralValue::Void })) }));
        }
        
        // while loop: "while X do Y"
        if self.check(&TokenKind::While) {
            return self.parse_while_loop();
        }
        
        // function call statement
        if self.check(&TokenKind::Call) {
            return self.parse_call_statement();
        }
        
        // throw statement: "throw <expression>"
        if self.match_token(&[TokenKind::Throw]) {
            return self.parse_throw_statement();
        }
        
        if self.check(&TokenKind::Using) {
            self.advance();
            return self.parse_using_statement();
        }
        
        if self.match_token(&[TokenKind::Run]) {
            return self.parse_run_concurrently_statement();
        }
        
        // try-catch statement: "try ... catch error ... end try"
        if self.match_token(&[TokenKind::Try]) {
            return self.parse_try_catch_statement();
        }
        
        // describe statement: "describe 'name' ... end describe"
        if self.match_token(&[TokenKind::Describe]) {
            return self.parse_describe_statement();
        }
        
        // test statement: "test 'name' ... end test"
        if self.match_token(&[TokenKind::Test]) {
            return self.parse_test_statement();
        }
        
        // expect statement: "expect actual is expected"
        if self.match_token(&[TokenKind::Expect]) {
            return self.parse_expect_statement();
        }
        
        // set statement: "set property in object to value"
        if let TokenKind::Identifier(ref s) = &self.current_token().kind {
            if s.to_lowercase() == "set" {
                return self.parse_set_statement();
            }
        }
        
        // print statement is already handled earlier (before "the variable" check)
        // check for short-form assignment: "variable is value"
        if let TokenKind::Identifier(ref _s) = self.current_token().kind {
            // peek to see if next token is "is" or "equals"
            if let Some(next_kind) = self.peek_kind_skip_newlines(1) {
                if matches!(next_kind, TokenKind::Is | TokenKind::Equals) {
                    return self.parse_assignment();
                }
            }
        }


        
        // try to parse as expression statement or assignment
        let start_location = self.current_token().location.clone();
        
        let result = match self.parse_expression() {
            Ok(expr) => {
                // check for assignment: "expr = value"
                if self.match_token(&[TokenKind::Equals]) {
                    let value = self.parse_expression()?;
                    
                    if let Node::IndexExpression(idx) = expr {
                        Ok(Node::IndexSetStatement(IndexSetStatement {
                            location: start_location,
                            object: idx.object,
                            index: idx.index,
                            value: Box::new(value),
                        }))
                    } else {
                        Err("Invalid assignment target".to_string())
                    }
                } else {
                    // Check if the expression was parsed as an equality check (e.g. "a[i] = b")
                    // which the expression parser treats as an OperationExpression with Operator::Equals
                    if let Node::OperationExpression(op) = &expr {
                        if op.operator == Operator::Equals {
                            if let Node::IndexExpression(idx) = &*op.left {
                                return Ok(Node::IndexSetStatement(IndexSetStatement {
                                    location: op.location.clone(),
                                    object: idx.object.clone(),
                                    index: idx.index.clone(),
                                    value: op.right.clone().unwrap(),
                                }));
                            }
                        }
                    }
                    
                    if self.check(&TokenKind::Newline) { self.advance(); }
                    
                    Ok(Node::ExpressionStatement(ExpressionStatement {
                        location: start_location,
                        expression: Box::new(expr),
                    }))
                }
            },
            Err(e) => {
                // ignore error and skip line (fallback)
                while !self.is_at_end() && !self.check(&TokenKind::Newline) { self.advance(); }
                Ok(Node::ExpressionStatement(ExpressionStatement {
                    location: self.current_token().location.clone(),
                    expression: Box::new(Node::LiteralExpression(LiteralExpression { location: self.current_token().location.clone(), value: LiteralValue::Void })),
                }))
            }
        };
        
        result
    }
    
    fn parse_assignment(&mut self) -> Result<Node, String> {
        let location = self.previous().location.clone();
        
        // CRITICAL: Check for "define", "import", or "return" BEFORE doing anything else
        // This should never happen, but if it does, we need to fail immediately
        // to prevent trying to parse these as identifiers or assignments
        if matches!(self.current_token().kind, TokenKind::Define | TokenKind::Import | TokenKind::Return) {
            return Err(format!("Expected 'is' or 'equals' for assignment, got: {:?} at {:?}", 
                self.current_token().kind, self.current_token().location));
        }
        
        // consume "variable" or "constant" if present
        let mut is_mutable = true;
        if self.check(&TokenKind::Variable) {
            self.advance();
            is_mutable = true;
        } else if self.check(&TokenKind::Constant) {
            self.advance();
            is_mutable = false;
        }        
        // Additional check: if current token is Define, Import, or Return (regardless of column), this is wrong
        if matches!(self.current_token().kind, TokenKind::Define | TokenKind::Import | TokenKind::Return) {
            return Err(format!("Expected 'is' or 'equals' for assignment, got: {:?} at {:?}", 
                self.current_token().kind, self.current_token().location));
        }
        
        let identifier = self.parse_identifier()?;
        
        // CRITICAL: After parsing identifier, check if next token is "return", "define", or "import"
        // If so, this is not an assignment - it's likely a statement boundary
        // This prevents trying to parse "the variable X return" or "the variable X define" as assignments
        self.skip_newlines();
        if matches!(self.current_token().kind, TokenKind::Return | TokenKind::Define | TokenKind::Import) {
            return Err(format!("Expected 'is' or 'equals' for assignment, got: {:?} at {:?}", 
                self.current_token().kind, self.current_token().location));
        }
        
        // handle optional "of type <Type>"
        // handle optional "of type <Type>"
        let type_annotation = if matches!(self.current_token().kind, TokenKind::Of) {
            if self.peek_kind_skip_newlines(1).and_then(|k| {
                if let TokenKind::Identifier(ref next) = k { Some(next == "type") } else { None }
            }).unwrap_or(false) {
                // match "of type"
                self.advance(); // consume "of"
                self.advance(); // consume "type"
                // parse type - parse_type() calls skip_newlines() at the start and parses the type name
                Some(self.parse_type()?)
            } else {
                None
            }
        } else {
            None
        };
        
        // context-aware "is" for assignment: "the variable x is value"
        // check if next tokens are "is a new" (object creation) - this is allowed in assignments!
        // allow newlines between type and 'is'
        // Note: parse_type() already called skip_newlines() at the start, but we need to skip again
        // in case there are newlines between the type and "is"
        self.skip_newlines();
        
        // debug: check what token we're at
        if !self.check(&TokenKind::Is) && !self.check(&TokenKind::Equals) {
            // if we don't see "is" or "equals", this is an error
            // but first check if we're at end of input
            if self.is_at_end() || self.check(&TokenKind::Eof) {
                return Err("Expected 'is' or 'equals' for assignment, but reached end of input".to_string());
            }
            // return error with current token info for debugging
            return Err(format!("Expected 'is' or 'equals' for assignment, got: {:?} at {:?}", 
                self.current_token().kind, self.current_token().location));
        }
        
        if self.check(&TokenKind::Is) {
            if let Some(kind_ahead) = self.peek_kind_skip_newlines(1) {
                if matches!(kind_ahead, TokenKind::A) {
                    if let Some(kind_ahead2) = self.peek_kind_skip_newlines(2) {
                        if matches!(kind_ahead2, TokenKind::New) {
                            // object creation syntax - parse it directly
                            self.advance(); // consume "is"
                            self.advance(); // consume "a"
                            self.advance(); // consume "new"
                            
                            let class_name = self.parse_identifier()?;
                            
                            // parse property assignments after "with"
                            let mut properties = Vec::new();
                            if self.match_token(&[TokenKind::With]) {
                                self.skip_newlines();
                                
                                loop {
                                    let prop_location = self.current_token().location.clone();
                                    let prop_name = self.parse_identifier()?;
                                    
                                    // skip "which is" or "as"
                                    if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                                        if s.to_lowercase() == "which" {
                                            self.advance();
                                            if let TokenKind::Identifier(ref s2) = &self.current_token().kind {
                                                if s2.to_lowercase() == "is" {
                                                    self.advance();
                                                }
                                            } else if matches!(&self.current_token().kind, TokenKind::Is) {
                                                self.advance();
                                            }
                                        } else if s.to_lowercase() == "as" {
                                            self.advance();
                                        }
                                    } else if matches!(&self.current_token().kind, TokenKind::Is) {
                                        self.advance();
                                    } else if self.current_token().kind == TokenKind::As {
                                        self.advance();
                                    }
                                    
                                    let prop_value = Box::new(self.parse_expression()?);
                                    
                                    properties.push(PropertyAssignment {
                                        location: prop_location,
                                        name: prop_name,
                                        value: prop_value,
                                    });
                                    
                                    self.skip_newlines();
                                    
                                    // check for comma or "and"
                                    if self.match_token(&[TokenKind::Comma, TokenKind::And]) {
                                        self.skip_newlines();
                                    }
                                    
                                    if self.is_at_end() || self.check(&TokenKind::Eof) {
                                        break;
                                    }
                                    // check if next token is a keyword that indicates end of object creation
                                    // check for "the" (start of variable declaration/assignment)
                                    if matches!(&self.current_token().kind, TokenKind::The) {
                                        break;
                                    }
                                    // check for other keywords by identifier name
                                    if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                                        let low = s.to_lowercase();
                                        if matches!(low.as_str(), "print" | "if" | "while" | "for" | "return" | "define" | "import") {
                                            break;
                                        }
                                    }
                                    // check for other token kinds that indicate end
                                    if matches!(&self.current_token().kind, 
                                        TokenKind::If | TokenKind::While | TokenKind::For |
                                        TokenKind::Return | TokenKind::Define | TokenKind::Import) {
                                        break;
                                    }
                                    // check if it's an identifier that could be a property
                                    if !matches!(&self.current_token().kind, TokenKind::Identifier(_) | TokenKind::TypeIdentifier(_)) {
                                        break;
                                    }
                                    // check if the identifier is a keyword (like "print", "if", etc.)
                                    if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                                        let low = s.to_lowercase();
                                        if matches!(low.as_str(), "print" | "if" | "while" | "for" | "return" | "define" | "import") {
                                            break;
                                        }
                                    }
                                }
                            }
                            
                            let expression = Node::ObjectCreation(ObjectCreation {
                                location: self.current_token().location.clone(),
                                class_name,
                                properties,
                            });
                            
                            if let Some(annotation) = type_annotation {
                                return Ok(Node::DeclareStatement(DeclareStatement {
                                    location,
                                    name: identifier,
                                    type_annotation: Some(annotation),
                                    value: Box::new(expression),
                                    is_mutable,
                                }));
                            } else {
                                return Ok(Node::AssignStatement(AssignStatement {
                                    location,
                                    identifier,
                                    expression: Box::new(expression),
                                    is_mutable,
                                }));
                            }
                        }
                    }
                }
            }
            // it's assignment: "is <value>"
            self.advance(); // consume "is"
        } else if self.match_token(&[TokenKind::Equals]) {
            // backward compatibility: "equals" also works
        } else {
            return Err("Expected 'is' or 'equals' for assignment".to_string());
        }
        
        let expression = self.parse_expression()?;
        
        if let Some(annotation) = type_annotation {
            Ok(Node::DeclareStatement(DeclareStatement {
                location,
                name: identifier,
                type_annotation: Some(annotation),
                value: Box::new(expression),
                is_mutable,
            }))
        } else {
            Ok(Node::AssignStatement(AssignStatement {
                location,
                identifier,
                expression: Box::new(expression),
                is_mutable,
            }))
        }
    }
    
    fn parse_set_statement(&mut self) -> Result<Node, String> {
        let location = self.current_token().location.clone();
        self.advance(); // consume "set"
        
        let property = self.parse_identifier()?;
        
        // check for "in" or "of" for property assignment, OR "to"/"is" for variable assignment
        self.skip_newlines();
        
        let is_property_set = if matches!(self.current_token().kind, TokenKind::In | TokenKind::Of) {
            self.advance();
            true
        } else if let TokenKind::Identifier(ref s) = &self.current_token().kind {
            if s.to_lowercase() == "in" || s.to_lowercase() == "of" {
                self.advance();
                true
            } else {
                false
            }
        } else {
            false
        };

        if is_property_set {
             // ... existing property set logic ...
             // But wait, I need to restructure because "object" parsing follows "in"
             // and "value" parsing follows "to".
        }
        
        // Let's rewrite the flow.
        if is_property_set {
             let object = self.parse_expression()?;
             
             self.skip_newlines();
             if !self.match_token(&[TokenKind::To, TokenKind::Is, TokenKind::Equals]) {
                 return Err(format!("Expected 'to' or 'is' after object, got: {:?}", self.current_token()));
             }
             
             let value = self.parse_expression()?;
             
             return Ok(Node::SetStatement(SetStatement {
                 location,
                 object: Box::new(object),
                 property,
                 value: Box::new(value),
             }));
        } else if self.match_token(&[TokenKind::To, TokenKind::Is, TokenKind::Equals]) {
             // Variable assignment: set x to y
             let value = self.parse_expression()?;
             
             return Ok(Node::AssignStatement(AssignStatement {
                 location,
                 identifier: property,
                 expression: Box::new(value),
                 is_mutable: true,
             }));
        } else {
             return Err(format!("Expected 'in' (for property) or 'to' (for variable) after '{}', got: {:?}", property, self.current_token()));
        }
        

    }
    
    fn parse_function_declaration(&mut self, location: Location, is_async: bool) -> Result<Node, String> {
        let name = self.parse_identifier()?;
        
        let mut parameters = Vec::new();
        
        // check for optional "that"
        self.match_token(&[TokenKind::That]);
        
        if self.match_token(&[TokenKind::Takes]) {
                loop {
                    let param_name = self.parse_identifier()?;
                    let param_type = if self.match_token(&[TokenKind::As]) {
                        Some(self.parse_type()?)
                    } else {
                        None
                    };
                    
                    parameters.push(Parameter {
                        name: param_name,
                        type_annotation: param_type,
                    });
                    
                    // peek ahead to check if next "and" is followed by "returns"
                    // if so, don't consume it here - let the return_type parsing handle it
                    let peek_and = self.check(&TokenKind::And);
                    let peek_returns = self.peek_kind_skip_newlines(1);
                    if peek_and && matches!(peek_returns, Some(TokenKind::Returns)) {
                        // this "and" is part of "and returns", not a parameter separator
                        break;
                    }
                    
                    if !self.match_token(&[TokenKind::And]) && !self.match_token(&[TokenKind::Comma]) {
                        break;
                    }


                }
            }

        
        let return_type = if self.match_token(&[TokenKind::And]) {
            if self.match_token(&[TokenKind::Returns]) {
                // after matching "returns", the next token should be the type name
                self.skip_newlines();
                Some(self.parse_type()?)
            } else {
                None
            }
        } else if self.match_token(&[TokenKind::Returns]) {
            self.skip_newlines();
            Some(self.parse_type()?)
        } else {
            None
        };
        
        // parse body - can be multiple statements or a single expression
        self.skip_newlines();
        let body = {
            // parse multiple statements until we hit a new top-level declaration or end
            let mut statements = Vec::new();
            let start_indent = self.current_token().location.column;
            
            loop {
                // check if we've hit end of file first
                if self.is_at_end() || self.check(&TokenKind::Eof) {
                    break;
                }
                
                // CRITICAL: Check for top-level declarations BEFORE skip_newlines()
                // Peek ahead through newlines to see if next token is a top-level declaration
                let mut found_top_level = false;
                let mut peek_pos = self.current;
                while peek_pos < self.tokens.len() {
                    let token = &self.tokens[peek_pos];
                    if matches!(token.kind, TokenKind::Newline) {
                        peek_pos += 1;
                        continue;
                    }
                    // Found non-newline token - check if it's a top-level declaration
                    if token.location.column == 0 {
                        if matches!(token.kind, TokenKind::Define | TokenKind::Import) {
                            // Top-level declaration found - mark and break
                            found_top_level = true;
                            break;
                        }
                        if matches!(token.kind, TokenKind::The) {
                            // Check if next token is "variable" (top-level variable declaration)
                            if peek_pos + 1 < self.tokens.len() {
                                if matches!(self.tokens[peek_pos + 1].kind, TokenKind::Variable) {
                                    // Top-level variable declaration - mark and break
                                    found_top_level = true;
                                    break;
                                }
                            }
                        }
                    }
                    // Found non-newline token that's not a top-level declaration - stop peeking
                    break;
                }
                
                // If we found a top-level declaration while peeking, break from outer loop immediately
                if found_top_level {
                    break;
                }
                
                // skip newlines first to get to the next meaningful token
                self.skip_newlines();
                
                // check current token's column and type BEFORE trying to parse
                let current_indent = self.current_token().location.column;
                
                // Check for "end function" explicitly
                if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                    if s.to_lowercase() == "end" {
                        if let Some(TokenKind::Function) = self.peek_kind_skip_newlines(1) {
                            self.advance(); // consume "end"
                            self.advance(); // consume "function"
                            break;
                        }
                    }
                }

                // if we're at less indentation than start, we've left the function body
                if current_indent < start_indent {
                    // back to less indentation, end of function body
                    break;
                }
                
                // CRITICAL: check if we've hit a new top-level declaration at column 0
                if current_indent == 0 {
                    if self.check(&TokenKind::Define) {
                        break;
                    }
                    if self.check(&TokenKind::Import) {
                        break;
                    }
                    if self.check(&TokenKind::The) {
                        if let Some(kind) = self.peek_kind_skip_newlines(1) {
                            if matches!(kind, TokenKind::Variable) {
                                break;
                            }
                        }
                    }
                }
                
                match self.parse_statement() {
                    Ok(stmt) => {
                        statements.push(stmt);
                        
                        // After parsing a statement, check if we've advanced to a top-level declaration
                        if !self.is_at_end() {
                            let col_after = self.current_token().location.column;
                            let kind_after = &self.current_token().kind;
                            if col_after == 0 {
                                if matches!(kind_after, TokenKind::Define | TokenKind::Import) {
                                    break;
                                }
                            }
                        }
                    }
                    Err(_e) => {
                        // if we get an error and we're at column 0 with "define", definitely break
                        if current_indent == 0 && self.check(&TokenKind::Define) {
                            break;
                        }
                        // can't parse more, stop
                        break;
                    }
                }
            }
            
            // wrap statements in a Program node
            if statements.is_empty() {
                // empty body - return void
                Box::new(Node::LiteralExpression(LiteralExpression {
                    location: self.current_token().location.clone(),
                    value: LiteralValue::Void,
                }))
            } else if statements.len() == 1 {
                // single statement
                Box::new(statements.remove(0))
            } else {
                // multiple statements - wrap in Program
                // get location from first statement
                let first_location = match &statements[0] {
                    Node::AssignStatement(a) => a.location.clone(),
                    Node::DeclareStatement(d) => d.location.clone(),
                    Node::ExpressionStatement(e) => e.location.clone(),
                    Node::ConditionalStatement(c) => c.location.clone(),
                    Node::LoopStatement(l) => l.location.clone(),
                    Node::ReturnStatement(r) => r.location.clone(),
                    Node::UsingStatement(u) => u.location.clone(),
                    Node::IndexSetStatement(s) => s.location.clone(),
                    Node::SetStatement(s) => s.location.clone(),
                    Node::ThrowStatement(t) => t.location.clone(),
                    _ => self.current_token().location.clone(),
                };
                Box::new(Node::Program(Program {
                    location: first_location,
                    statements,
                }))
            }
        };
        
        Ok(Node::FunctionDeclaration(FunctionDeclaration {
            location,
            name,
            parameters,
            return_type,
            body,
            is_async,
        }))
    }

    fn parse_declaration_or_function(&mut self) -> Result<Node, String> {
        // NOTE: "define" has already been consumed by the caller
        let location = self.previous().location.clone();
        
        // check for "class" after "define"
        if let TokenKind::Identifier(ref s) = &self.current_token().kind {
            if s.to_lowercase() == "class" {
                // consume "class"
                self.advance();
                return self.parse_class_declaration(location);
            }
        }
        
        // check for "struct" after "define"
        if self.match_token(&[TokenKind::Struct]) {
            return self.parse_struct_declaration(location);
        }

        // check for "type" after "define" (for sum types)
        if let TokenKind::Identifier(ref s) = &self.current_token().kind {
            if s.to_lowercase() == "type" {
                self.advance();
                return self.parse_type_declaration(location);
            }
        }
        
        // check for "background" or "asynchronous"
        let is_async = if self.match_token(&[TokenKind::Background]) {
            true
        } else {
            false
        };
        
        if self.match_token(&[TokenKind::Function]) {
            // "function" token already consumed, parse function declaration
            return self.parse_function_declaration(location, is_async);
        } else {
            if is_async {
                return Err("Expected 'function' after 'background'".to_string());
            }
            self.parse_variable_declaration()
        }
    }
    
    fn parse_type_declaration(&mut self, location: Location) -> Result<Node, String> {
        // define type <Name> as either <Variant1> or <Variant2> ...
        let name = self.parse_identifier()?;
        
        self.consume(&TokenKind::As, "Expected 'as' after type name")?;
        self.consume(&TokenKind::Either, "Expected 'either' after 'as'")?;
        
        let mut variants = Vec::new();
        
        // Parse first variant
        variants.push(self.parse_variant()?);
        
        // Parse subsequent variants
        while self.match_token(&[TokenKind::Or]) {
            variants.push(self.parse_variant()?);
        }
        
        Ok(Node::TypeDeclaration(TypeDeclaration {
            location,
            name,
            variants,
        }))
    }
    
    fn parse_variant(&mut self) -> Result<Variant, String> {
        let name = self.parse_identifier()?;
        let mut fields = Vec::new();
        
        // Check for fields: "with <field> of type <Type> and ..."
        if self.match_token(&[TokenKind::With]) {
            loop {
                let field_name = self.parse_identifier()?;
                
                // "of type" or just "of"
                if self.match_token(&[TokenKind::Of]) {
                    if let TokenKind::Identifier(ref s) = self.current_token().kind {
                        if s.to_lowercase() == "type" {
                            self.advance();
                        }
                    }
                }
                
                let field_type = if matches!(self.current_token().kind, TokenKind::TypeIdentifier(_)) {
                    // Simple type
                     if let TokenKind::TypeIdentifier(t) = &self.current_token().kind {
                        let t = t.clone();
                        self.advance();
                        t
                    } else {
                        return Err("Expected type identifier".to_string());
                    }
                } else {
                    // Try to parse type
                    // Simplified: just expect TypeIdentifier for now
                     if let TokenKind::TypeIdentifier(t) = &self.current_token().kind {
                        let t = t.clone();
                        self.advance();
                        t
                    } else {
                        // Fallback to "Any" or error?
                        // For now, require explicit type
                        return Err(format!("Expected type for field '{}', got {:?}", field_name, self.current_token().kind));
                    }
                };
                
                fields.push((field_name, field_type));
                
                if !self.match_token(&[TokenKind::And]) {
                    break;
                }
            }
        }
        
        Ok(Variant { name, fields })
    }

    fn parse_struct_declaration(&mut self, location: Location) -> Result<Node, String> {
        let name = self.parse_identifier()?;
        
        let mut properties = Vec::new();
        
        if self.match_token(&[TokenKind::With]) {
            self.skip_newlines();
            
            loop {
                let prop_loc = self.current_token().location.clone();
                let prop_name = self.parse_identifier()?;
                
                self.consume(&TokenKind::As, "Expected 'as' after property name")?;
                let prop_type = self.parse_type()?;
                
                properties.push(PropertyDeclaration {
                    location: prop_loc,
                    name: prop_name,
                    type_annotation: prop_type,
                });
                
                // check for comma or "and"
                if self.match_token(&[TokenKind::Comma, TokenKind::And]) {
                    self.skip_newlines();
                    continue;
                } else {
                    break;
                }
            }
        }
        
        Ok(Node::StructDeclaration(StructDeclaration {
            location,
            name,
            properties,
        }))
    }
    
    fn parse_variable_declaration(&mut self) -> Result<Node, String> {
        let location = self.previous().location.clone();
        
        let is_mutable = if self.match_token(&[TokenKind::Variable]) {
            true
        } else if self.match_token(&[TokenKind::Constant]) {
            false
        } else {
            return Err("Expected 'variable' or 'constant'".to_string());
        };
        let identifier = self.parse_identifier()?;
        
        let type_annotation = if self.match_token(&[TokenKind::As]) {
            Some(self.parse_type()?)
        } else {
            if matches!(self.current_token().kind, TokenKind::Of) {
                if self.peek_kind_skip_newlines(1).and_then(|k| {
                    if let TokenKind::Identifier(ref next) = k { Some(next == "type") } else { None }
                }).unwrap_or(false) {
                    // match "of type"
                    self.advance(); // consume "of"
                    self.advance(); // consume "type"
                    Some(self.parse_type()?)
                } else {
                    None
                }
            } else {
                None
            }
        };

        
        // context-aware "is" for initial value: "define variable x as Number is value"
        // or backward compatibility: "equals value"
        let initial_value = if self.check(&TokenKind::Is) {
            // check if it's "is a new" (object creation) - but in declaration, it's just assignment
            if let Some(kind_ahead) = self.peek_kind_skip_newlines(1) {
                if matches!(kind_ahead, TokenKind::A) {
                    if let Some(kind_ahead2) = self.peek_kind_skip_newlines(2) {
                        if matches!(kind_ahead2, TokenKind::New) {
                            // object creation in declaration - parse as expression
                            self.advance(); // consume "is"
                            Some(Box::new(self.parse_expression()?))
                        } else {
                            // just "is a" - treat as assignment
                            self.advance(); // consume "is"
                            Some(Box::new(self.parse_expression()?))
                        }
                    } else {
                        // just "is a" - treat as assignment
                        self.advance(); // consume "is"
                        Some(Box::new(self.parse_expression()?))
                    }
                } else {
                    // just "is" - simple assignment
                    self.advance(); // consume "is"
                    Some(Box::new(self.parse_expression()?))
                }
            } else {
                // just "is" - simple assignment
                self.advance(); // consume "is"
                Some(Box::new(self.parse_expression()?))
            }
        } else if self.match_token(&[TokenKind::Equals]) {
            // backward compatibility
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };
        
        let value = if let Some(v) = initial_value {
            v
        } else {
            // Allow uninitialized if type annotation is present
            if type_annotation.is_some() {
                 Box::new(Node::LiteralExpression(LiteralExpression {
                     location: location.clone(),
                     value: LiteralValue::Void,
                 }))
            } else {
                 return Err(format!("Variable '{}' must be initialized", identifier));
            }
        };
        
        Ok(Node::DeclareStatement(DeclareStatement {
            location,
            name: identifier,
            type_annotation,
            value,
            is_mutable,
        }))
    }
    
    fn _parse_function_declaration(&mut self) -> Result<Node, String> {
        let location = self.previous().location.clone();
        
        self.consume(&TokenKind::Function, "Expected 'function'")?;
        let name = self.parse_identifier()?;
        
        let mut parameters = Vec::new();
        
        if self.match_token(&[TokenKind::That]) && self.match_token(&[TokenKind::Takes]) {
            loop {
                let param_name = self.parse_identifier()?;
                let param_type = if self.match_token(&[TokenKind::As]) {
                    Some(self.parse_type()?)
                } else {
                    None
                };
                
                parameters.push(Parameter {
                    name: param_name,
                    type_annotation: param_type,
                });
                
                // check if next is "and" - if not, we're done with parameters
                if !self.match_token(&[TokenKind::And]) {
                    break;
                }
                // if we consumed "and", check if next is "returns" - if so, we're done with parameters
                // peek ahead to see if next is "returns" without consuming it
                if self.check(&TokenKind::Returns) {
                    // don't consume "returns" here - let return type parsing handle it
                    // but we need to backtrack the "and" we just consumed
                    // Actually, we can't backtrack easily, so let's just break and let return type parsing handle "and returns"
                    break;
                }
                // otherwise, continue loop for more parameters
            }
        }
        
        // parse return type - check for "returns" or "and returns"
        // After parameter loop: if it saw "and returns", we consumed "and" and are now at "returns"
        // So we should check for "returns" first
        let return_type = if self.check(&TokenKind::Returns) {
            // "returns" is here - consume it and parse type
            self.advance(); // consume "returns"
            self.skip_newlines();
            Some(self.parse_type()?)
        } else if self.match_token(&[TokenKind::And]) {
            // we have "and" - check if next is "returns"
            if self.match_token(&[TokenKind::Returns]) {
                // "returns" was consumed by match_token
                self.skip_newlines();
                Some(self.parse_type()?)
            } else {
                None
            }
        } else {
            None
        };
        

        
        // parse body
        self.skip_newlines();
        let start_indent = self.current_token().location.column;
        let mut statements = Vec::new();
        
        loop {
            if self.is_at_end() || self.check(&TokenKind::Eof) {
                break;
            }
            
            let current_indent = self.current_token().location.column;
            // if we dedent, we are done with the function body
            if current_indent < start_indent {
                break;
            }
            
            if self.check(&TokenKind::Newline) {
                self.advance();
                continue;
            }
            
            statements.push(self.parse_statement()?);
        }
        println!("Parsed function body with {} statements", statements.len());
        
        let body = Box::new(Node::Program(Program {
            location: location.clone(),
            statements,
        }));
        
        Ok(Node::FunctionDeclaration(FunctionDeclaration {
            location,
            name,
            parameters,
            return_type,
            body,
            is_async: false,
        }))
    }
    
    fn parse_import(&mut self) -> Result<Node, String> {
        let location = self.previous().location.clone();
        let start_token = self.previous().kind.clone();
        
        let mut module_name = String::new();
        let mut alias = None;
        let mut specific_imports = Vec::new();
        let mut is_file_path = false;
        
        if matches!(start_token, TokenKind::Import) {
            // Syntax: import <module> [as <alias>]
            
            // check for "the module" (legacy)
            if self.check(&TokenKind::The) {
                 self.advance();
                 if self.check(&TokenKind::Identifier(String::from("module"))) {
                     self.advance();
                 }
            }
            
            if self.check(&TokenKind::Identifier(String::from("file"))) {
                self.advance(); // consume "file"
                is_file_path = true;
                
                if let TokenKind::Text(path) = &self.current_token().kind {
                    module_name = path.clone();
                    self.advance();
                } else {
                    return Err("Expected file path string after 'import file'".to_string());
                }
            } else if let TokenKind::Text(path) = &self.current_token().kind {
                // Allow import "package/module" directly
                module_name = path.clone();
                self.advance();
            } else {
                module_name = self.parse_identifier()?;
            }
            
            // check for alias
            if self.match_token(&[TokenKind::As]) {
                alias = Some(self.parse_identifier()?);
            }
            
        } else if matches!(start_token, TokenKind::From) {
            // Syntax: from file '<path>' import <items>
            // or: from <module> import <items>
            
            if self.check(&TokenKind::Identifier(String::from("file"))) {
                self.advance(); // consume "file"
                is_file_path = true;
                
                // expect string literal for path
                if let TokenKind::Text(path) = &self.current_token().kind {
                    module_name = path.clone();
                    self.advance();
                } else {
                    return Err("Expected file path string after 'from file'".to_string());
                }
            } else {
                // from <module>
                module_name = self.parse_identifier()?;
            }
            
            self.consume(&TokenKind::Import, "Expected 'import' after source")?;
            
            // parse specific imports
            loop {
                // handle "function name", "class name" etc. - just take the name
                if matches!(self.current_token().kind, TokenKind::Function | TokenKind::Variable) {
                    self.advance();
                } else if let TokenKind::Identifier(ref s) = self.current_token().kind {
                    if s.to_lowercase() == "class" {
                        self.advance();
                    }
                }
                
                specific_imports.push(self.parse_identifier()?);
                
                if !self.match_token(&[TokenKind::Comma, TokenKind::And]) {
                    break;
                }
            }
        }
        
        Ok(Node::ImportStatement(ImportStatement {
            location,
            module_name,
            alias,
            specific_imports,
            is_file_path,
        }))
    }
    
    fn parse_conditional(&mut self) -> Result<Node, String> {
        let location = self.previous().location.clone();
        self.advance(); // consume "if"
        
        let condition = Box::new(self.parse_expression()?);
        // tolerate optional 'then'
        let _ = self.match_token(&[TokenKind::Then]);
        
        // parse THEN block: multiple statements until 'else/otherwise' or 'end if'
        self.skip_newlines();
        let start_indent = self.current_token().location.column;
        let mut then_statements: Vec<Node> = Vec::new();
        loop {
            if self.is_at_end() || self.check(&TokenKind::Eof) { break; }
            self.skip_newlines();
            let current_indent = self.current_token().location.column;
            // check for 'end if' at same or less indent
            if current_indent <= start_indent {
                if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                    if s.to_lowercase() == "end" {
                        if let Some(kind) = self.peek_kind_skip_newlines(1) {
                            if matches!(kind, TokenKind::If) {
                                // consume 'end if'
                                self.advance();
                                self.advance();
                                self.skip_newlines();
                                break;
                            }
                        }
                    }
                }
                // stop if we see an 'else' or 'otherwise'
                if self.check(&TokenKind::Else) || self.check(&TokenKind::Otherwise) {
                    break;
                }
            }
            // if top-level declaration at column 0, stop (defensive)
            if current_indent == 0 {
                if self.check(&TokenKind::Define) || self.check(&TokenKind::Import) { break; }
            }
            match self.parse_statement() {
                Ok(stmt) => then_statements.push(stmt),
                Err(_) => break,
            }
        }
        // optional ELSE block
        self.skip_newlines();
        let else_branch = if self.match_token(&[TokenKind::Otherwise, TokenKind::Else]) {
            // 'else if' -> nested conditional
            if self.check(&TokenKind::If) {
                Some(Box::new(self.parse_conditional()?))
            } else {
                // parse ELSE block until 'end if'
                self.skip_newlines();
                let else_start = self.current_token().location.column;
                let mut else_statements: Vec<Node> = Vec::new();
                loop {
                    if self.is_at_end() || self.check(&TokenKind::Eof) { break; }
                    self.skip_newlines();
                    let current_indent = self.current_token().location.column;
                    if current_indent <= else_start {
                        if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                            if s.to_lowercase() == "end" {
                                if let Some(kind) = self.peek_kind_skip_newlines(1) {
                                    if matches!(kind, TokenKind::If) {
                                        self.advance(); // end
                                        self.advance(); // if
                                        self.skip_newlines();
                                    break;
                                    }
                                }
                            }
                        }
                    }
                    if current_indent == 0 {
                        if self.check(&TokenKind::Define) || self.check(&TokenKind::Import) { break; }
                    }
                    match self.parse_statement() {
                        Ok(stmt) => else_statements.push(stmt),
                        Err(_) => break,
                    }
                }
                // build else block node
                let else_node = if else_statements.is_empty() {
                    Node::LiteralExpression(LiteralExpression { location: self.current_token().location.clone(), value: LiteralValue::Void })
                } else if else_statements.len() == 1 {
                    else_statements.remove(0)
                } else {
                    // wrap in program
                    let first_loc = match &else_statements[0] {
                        Node::AssignStatement(a) => a.location.clone(),
                        Node::DeclareStatement(d) => d.location.clone(),
                        Node::ExpressionStatement(e) => e.location.clone(),
                        Node::ConditionalStatement(c) => c.location.clone(),
                        Node::LoopStatement(l) => l.location.clone(),
                        Node::ReturnStatement(r) => r.location.clone(),
                        Node::UsingStatement(u) => u.location.clone(),
                        Node::IndexSetStatement(s) => s.location.clone(),
                        Node::SetStatement(s) => s.location.clone(),
                        Node::ThrowStatement(t) => t.location.clone(),
                        _ => self.current_token().location.clone(),
                    };
                    Node::Program(Program { location: first_loc, statements: else_statements })
                };
                Some(Box::new(else_node))
            }
        } else {
            None
        };
        // build then block node
        let then_node = if then_statements.is_empty() {
            Node::LiteralExpression(LiteralExpression { location: self.current_token().location.clone(), value: LiteralValue::Void })
        } else if then_statements.len() == 1 {
            then_statements.remove(0)
        } else {
            let first_loc = match &then_statements[0] {
                Node::AssignStatement(a) => a.location.clone(),
                Node::DeclareStatement(d) => d.location.clone(),
                Node::ExpressionStatement(e) => e.location.clone(),
                Node::ConditionalStatement(c) => c.location.clone(),
                Node::LoopStatement(l) => l.location.clone(),
                Node::ReturnStatement(r) => r.location.clone(),
                Node::UsingStatement(u) => u.location.clone(),
                Node::IndexSetStatement(s) => s.location.clone(),
                Node::SetStatement(s) => s.location.clone(),
                Node::ThrowStatement(t) => t.location.clone(),
                _ => self.current_token().location.clone(),
            };
            Node::Program(Program { location: first_loc, statements: then_statements })
        };
        Ok(Node::ConditionalStatement(ConditionalStatement { location, condition, then_branch: Box::new(then_node), else_branch }))
    }
    
    fn parse_for_loop(&mut self) -> Result<Node, String> {
    let location = self.previous().location.clone();
    // already consumed "for each"
    
    let iterator = self.parse_identifier()?;
    // tolerate newlines before 'in'
    self.consume(&TokenKind::In, "Expected 'in'")?;
    let collection = Box::new(self.parse_expression()?);
    self.consume(&TokenKind::Do, "Expected 'do'")?;
    
    // parse loop body: multiple statements until 'end for'
    self.skip_newlines();
    let start_indent = self.current_token().location.column;
    let mut statements: Vec<Node> = Vec::new();
    
    loop {
        if self.is_at_end() || self.check(&TokenKind::Eof) { break; }
        
        self.skip_newlines();
        
        let current_indent = self.current_token().location.column;
        
        // check for 'end for'
        if let TokenKind::Identifier(ref s) = &self.current_token().kind {
            if s.to_lowercase() == "end" {
                if let Some(kind) = self.peek_kind_skip_newlines(1) {
                    if matches!(kind, TokenKind::For) {
                        // consume 'end for'
                        self.advance();
                        self.advance();
                        self.skip_newlines();
                        break;
                    }
                }
            }
        }
        
        // if top-level declaration at column 0, stop (defensive)
        if current_indent == 0 && start_indent > 0 {
             if self.check(&TokenKind::Define) || self.check(&TokenKind::Import) { break; }
        }
        
        match self.parse_statement() {
            Ok(stmt) => statements.push(stmt),
            Err(_) => break,
        }
    }
    
    let body = if statements.is_empty() {
        Box::new(Node::LiteralExpression(LiteralExpression { location: self.current_token().location.clone(), value: LiteralValue::Void }))
    } else if statements.len() == 1 {
        Box::new(statements.remove(0))
    } else {
        let first_loc = match &statements[0] {
            Node::AssignStatement(a) => a.location.clone(),
            Node::DeclareStatement(d) => d.location.clone(),
            Node::ExpressionStatement(e) => e.location.clone(),
            Node::ConditionalStatement(c) => c.location.clone(),
            Node::LoopStatement(l) => l.location.clone(),
            Node::ReturnStatement(r) => r.location.clone(),
            Node::UsingStatement(u) => u.location.clone(),
            Node::IndexSetStatement(s) => s.location.clone(),
            Node::SetStatement(s) => s.location.clone(),
            Node::ThrowStatement(t) => t.location.clone(),
            _ => self.current_token().location.clone(),
        };
        Box::new(Node::Program(Program { location: first_loc, statements }))
    };
    
    Ok(Node::LoopStatement(LoopStatement {
        location,
        loop_type: LoopType::ForEach,
        condition: None,
        iterator: Some(iterator),
        collection: Some(collection),
        body,
    }))
    }
    
    fn parse_while_loop(&mut self) -> Result<Node, String> {
        let location = self.current_token().location.clone();
        let _loop_indent = location.column;
        self.advance(); // consume "while"
        
        let condition = Box::new(self.parse_expression()?);
        
        self.consume(&TokenKind::Do, "Expected 'do' after while condition")?;
        
        // parse body - can be multiple statements
        self.skip_newlines();
        let start_indent = self.current_token().location.column;
        let mut statements = Vec::new();
        
        loop {
            // check if we've hit end of file
            if self.is_at_end() || self.check(&TokenKind::Eof) {
                break;
            }
            self.skip_newlines();
            
            // check indentation
            let current_indent = self.current_token().location.column;
            
            // Check for "end while" - this must come before the dedent check
            // "end while" should be at the same indentation as the while statement (loop_indent)
            // or at least <= start_indent
            if current_indent <= start_indent {
                // check for "end while" first
                if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                    if s.to_lowercase() == "end" {
                        if let Some(kind) = self.peek_kind_skip_newlines(1) {
                            if matches!(kind, TokenKind::While) {
                                // found "end while", consume it and stop parsing body
                                self.advance(); // consume "end"
                                self.advance(); // consume "while" (peek already checked it)
                                // skip any newlines after "end while"
                                self.skip_newlines();
                                break;
                            }
                        }
                    }
                }
                
                // also check for new top-level declarations at column 0
                if current_indent == 0 {
                    if self.check(&TokenKind::Define) || self.check(&TokenKind::Import) {
                        break;
                    }
                    if self.check(&TokenKind::The) {
                        if let Some(kind) = self.peek_kind_skip_newlines(1) {
                            if matches!(kind, TokenKind::Variable) {
                                break;
                            }
                        }
                    }
                }
            }

            // if we're back at less indentation (and wasn't end while), end of loop body
            if current_indent < start_indent {
                break;
            }
            
            // try to parse a statement (only if we didn't break above)
            // BUT: if we're at column 0 and see "define", we should have broken above
            // so if we reach here and current_indent is 0, something is wrong
            if current_indent == 0 && self.check(&TokenKind::Define) {
                // we should have broken above, but didn't - break now to be safe
                break;
            }
            
            match self.parse_statement() {
                Ok(stmt) => {
                    statements.push(stmt);
                    self.skip_newlines();
                }
                Err(e) => {

                    // can't parse more, stop
                    break;
                }
            }
        }
        
        // wrap statements in body
        let body = if statements.is_empty() {
            Box::new(Node::LiteralExpression(LiteralExpression {
                location: self.current_token().location.clone(),
                value: LiteralValue::Void,
            }))
        } else if statements.len() == 1 {
            Box::new(statements.remove(0))
        } else {
            let first_location = match &statements[0] {
                Node::AssignStatement(a) => a.location.clone(),
                Node::DeclareStatement(d) => d.location.clone(),
                Node::ExpressionStatement(e) => e.location.clone(),
                Node::ConditionalStatement(c) => c.location.clone(),
                Node::LoopStatement(l) => l.location.clone(),
                Node::ReturnStatement(r) => r.location.clone(),
                Node::UsingStatement(u) => u.location.clone(),
                Node::IndexSetStatement(s) => s.location.clone(),
                Node::SetStatement(s) => s.location.clone(),
                Node::ThrowStatement(t) => t.location.clone(),
                _ => self.current_token().location.clone(),
            };
            Box::new(Node::Program(Program {
                location: first_location,
                statements,
            }))
        };
        
        Ok(Node::LoopStatement(LoopStatement {
            location,
            loop_type: LoopType::While,
            condition: Some(condition),
            iterator: None,
            collection: None,
            body,
        }))
    }
    
    fn parse_return(&mut self) -> Result<Node, String> {
        let location = self.previous().location.clone();
        
        let expression = if !self.check(&TokenKind::Eof) && !self.check(&TokenKind::Newline) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };
        
        Ok(Node::ReturnStatement(ReturnStatement {
            location,
            expression,
        }))
    }
    
    fn parse_call_statement(&mut self) -> Result<Node, String> {
        let location = self.previous().location.clone();
        let call_expr = self.parse_call_expression()?;
        Ok(Node::ExpressionStatement(ExpressionStatement {
            location,
            expression: Box::new(call_expr),
        }))
    }



    
    fn _parse_expression_statement(&mut self) -> Result<Node, String> {
        let location = self.current_token().location.clone();
        
        // if we can't parse anything, just return nothing for now
        if self.is_at_end() || self.check(&TokenKind::Eof) {
            return Ok(Node::ExpressionStatement(ExpressionStatement {
                location: location.clone(),
                expression: Box::new(Node::LiteralExpression(LiteralExpression {
                    location,
                    value: LiteralValue::Void,
                })),
            }));
        }
        
        match self.parse_expression() {
            Ok(expr) => Ok(Node::ExpressionStatement(ExpressionStatement {
                location: location.clone(),
                expression: Box::new(expr),
            })),
            Err(_) => {
                // if expression parsing fails, just skip this statement
                Ok(Node::ExpressionStatement(ExpressionStatement {
                    location,
                    expression: Box::new(Node::LiteralExpression(LiteralExpression {
                        location: self.current_token().location.clone(),
                        value: LiteralValue::Void,
                    })),
                }))
            }
        }
    }
    
    fn parse_expression(&mut self) -> Result<Node, String> {
        self.parse_or()
    }
    
    fn parse_or(&mut self) -> Result<Node, String> {
        let mut expr = self.parse_and()?;
        
        while self.match_token(&[TokenKind::Or]) {
            let op = Operator::Or;
            let right = Box::new(self.parse_and()?);
            expr = Node::OperationExpression(OperationExpression {
                location: self.previous().location.clone(),
                operator: op,
                left: Box::new(expr),
                right: Some(right),
            });
        }
        
        Ok(expr)
    }
    
    fn parse_and(&mut self) -> Result<Node, String> {
        let mut expr = self.parse_comparison()?;
        
        while self.match_token(&[TokenKind::And]) {
            let op = Operator::And;
            let right = Box::new(self.parse_comparison()?);
            expr = Node::OperationExpression(OperationExpression {
                location: self.previous().location.clone(),
                operator: op,
                left: Box::new(expr),
                right: Some(right),
            });
        }
        
        Ok(expr)
    }
    
    fn parse_comparison(&mut self) -> Result<Node, String> {
        let mut expr = self.parse_term()?;
        
        // handle "is less than", "is greater than", "equals", etc.
        loop {
            // check for "exists" (postfix operator)
            if self.match_token(&[TokenKind::Exists]) {
                expr = Node::OperationExpression(OperationExpression {
                    location: self.previous().location.clone(),
                    operator: Operator::Exists,
                    left: Box::new(expr),
                    right: None,
                });
                continue;
            }
            
            // IMPORTANT: peek ahead BEFORE consuming "is" to check for "is greater than" or "is less than"
            if self.check(&TokenKind::Is) {
                let peek_greater = self.peek_kind_skip_newlines(1); // skip "is"
                let peek_than = self.peek_kind_skip_newlines(2); // skip "is" and "greater"/"less"
                
                // check for "is greater than" or "is less than"
                if matches!(peek_greater, Some(TokenKind::GreaterThan)) && matches!(peek_than, Some(TokenKind::Than)) {
                    // consume "is", "greater", and "than"
                    self.advance(); // consume "is"
                    self.advance(); // consume "greater"
                    self.advance(); // consume "than"
                    
                    // check for "or equal to"
                    if self.match_token(&[TokenKind::Or]) {
                        if self.match_token(&[TokenKind::Equals]) {
                            if self.match_token(&[TokenKind::To]) {
                                let right = Box::new(self.parse_term()?);
                                expr = Node::OperationExpression(OperationExpression {
                                    location: self.previous().location.clone(),
                                    operator: Operator::GreaterThanOrEqual,
                                    left: Box::new(expr),
                                    right: Some(right),
                                });
                                continue;
                            }
                        }
                    } else {
                        let right = Box::new(self.parse_term()?);
                        expr = Node::OperationExpression(OperationExpression {
                            location: self.previous().location.clone(),
                            operator: Operator::GreaterThan,
                            left: Box::new(expr),
                            right: Some(right),
                        });
                        continue;
                    }
                } else if matches!(peek_greater, Some(TokenKind::LessThan)) && matches!(peek_than, Some(TokenKind::Than)) {
                    // consume "is", "less", and "than"
                    self.advance(); // consume "is"
                    self.advance(); // consume "less"
                    self.advance(); // consume "than"
                    
                    // check for "or equal to"
                    if self.match_token(&[TokenKind::Or]) {
                        if self.match_token(&[TokenKind::Equals]) {
                            if self.match_token(&[TokenKind::To]) {
                                let right = Box::new(self.parse_term()?);
                                expr = Node::OperationExpression(OperationExpression {
                                    location: self.previous().location.clone(),
                                    operator: Operator::LessThanOrEqual,
                                    left: Box::new(expr),
                                    right: Some(right),
                                });
                                continue;
                            }
                        }
                    } else {
                        let right = Box::new(self.parse_term()?);
                        expr = Node::OperationExpression(OperationExpression {
                            location: self.previous().location.clone(),
                            operator: Operator::LessThan,
                            left: Box::new(expr),
                            right: Some(right),
                        });
                        continue;
                    }
                } else if matches!(peek_greater, Some(TokenKind::Not)) {
                     // consume "is", "not"
                     self.advance(); // consume "is"
                     self.advance(); // consume "not"
                     
                     let right = Box::new(self.parse_term()?);
                     expr = Node::OperationExpression(OperationExpression {
                         location: self.previous().location.clone(),
                         operator: Operator::NotEquals,
                         left: Box::new(expr),
                         right: Some(right),
                     });
                     continue;
                } else {
                    // "is" alone -> treat as equality check
                    self.advance(); // consume "is"
                    let right = Box::new(self.parse_term()?);
                    expr = Node::OperationExpression(OperationExpression {
                        location: self.previous().location.clone(),
                        operator: Operator::Equals,
                        left: Box::new(expr),
                        right: Some(right),
                    });
                    continue;
                }
            }
            
            // check for "equals" (without "is")
            if self.match_token(&[TokenKind::Equals]) {
                let right = Box::new(self.parse_term()?);
                expr = Node::OperationExpression(OperationExpression {
                    location: self.previous().location.clone(),
                    operator: Operator::Equals,
                    left: Box::new(expr),
                    right: Some(right),
                });
                continue;
            }
            
            // check for "greater than" (without "is")
            if self.match_token(&[TokenKind::GreaterThan]) {
                // optional "than" if using symbol >
                let _ = self.match_token(&[TokenKind::Than]);
                
                // check for "or equal to"
                if self.match_token(&[TokenKind::Or]) {
                    if self.match_token(&[TokenKind::Equals]) {
                        if self.match_token(&[TokenKind::To]) {
                            let right = Box::new(self.parse_term()?);
                            expr = Node::OperationExpression(OperationExpression {
                                location: self.previous().location.clone(),
                                operator: Operator::GreaterThanOrEqual,
                                left: Box::new(expr),
                                right: Some(right),
                            });
                            continue;
                        }
                    }
                } else if self.match_token(&[TokenKind::Equals]) {
                    // handle >= (GreaterThan followed by Equals)
                    let right = Box::new(self.parse_term()?);
                    expr = Node::OperationExpression(OperationExpression {
                        location: self.previous().location.clone(),
                        operator: Operator::GreaterThanOrEqual,
                        left: Box::new(expr),
                        right: Some(right),
                    });
                    continue;
                } else {
                    let right = Box::new(self.parse_term()?);
                    expr = Node::OperationExpression(OperationExpression {
                        location: self.previous().location.clone(),
                        operator: Operator::GreaterThan,
                        left: Box::new(expr),
                        right: Some(right),
                    });
                    continue;
                }
            }
            
            // check for "less than" (without "is")
            if self.match_token(&[TokenKind::LessThan]) {
                // optional "than" if using symbol <
                let _ = self.match_token(&[TokenKind::Than]);
                
                // check for "or equal to"
                if self.match_token(&[TokenKind::Or]) {
                    if self.match_token(&[TokenKind::Equals]) {
                        if self.match_token(&[TokenKind::To]) {
                            let right = Box::new(self.parse_term()?);
                            expr = Node::OperationExpression(OperationExpression {
                                location: self.previous().location.clone(),
                                operator: Operator::LessThanOrEqual,
                                left: Box::new(expr),
                                right: Some(right),
                            });
                            continue;
                        }
                    }
                } else if self.match_token(&[TokenKind::Equals]) {
                    // handle <= (LessThan followed by Equals)
                    let right = Box::new(self.parse_term()?);
                    expr = Node::OperationExpression(OperationExpression {
                        location: self.previous().location.clone(),
                        operator: Operator::LessThanOrEqual,
                        left: Box::new(expr),
                        right: Some(right),
                    });
                    continue;
                } else {
                    let right = Box::new(self.parse_term()?);
                    expr = Node::OperationExpression(OperationExpression {
                        location: self.previous().location.clone(),
                        operator: Operator::LessThan,
                        left: Box::new(expr),
                        right: Some(right),
                    });
                    continue;
                }
            }

            break;
        }

        
        Ok(expr)
    }
    
    fn parse_term(&mut self) -> Result<Node, String> {
        let mut expr = self.parse_factor()?;
        // println!("DEBUG: parse_term after factor: {:?}, next: {:?}", expr, self.current_token());
        
        while self.match_token(&[TokenKind::Plus, TokenKind::Minus]) {
            // println!("DEBUG: parse_term found op: {:?}", self.previous());
            let op = match self.previous().kind {
                TokenKind::Plus => Operator::Plus,
                TokenKind::Minus => Operator::Minus,
                _ => unreachable!(),
            };
            let right = Box::new(self.parse_factor()?);
            expr = Node::OperationExpression(OperationExpression {
                location: self.previous().location.clone(),
                operator: op,
                left: Box::new(expr),
                right: Some(right),
            });
        }
        
        Ok(expr)
    }
    
    fn parse_factor(&mut self) -> Result<Node, String> {
        let mut expr = self.parse_unary()?;
        
        // handle "times", "divided by", "modulo"
        loop {
            if self.match_token(&[TokenKind::Times]) {
                let right = Box::new(self.parse_unary()?);
                expr = Node::OperationExpression(OperationExpression {
                    location: self.previous().location.clone(),
                    operator: Operator::Times,
                    left: Box::new(expr),
                    right: Some(right),
                });
                continue;
            }
            if self.match_token(&[TokenKind::DividedBy]) {
                // "divided by" - if next token is also DividedBy (from "by"), skip it
                if self.check(&TokenKind::DividedBy) {
                    self.advance(); // skip the "by" token
                }
                let right = Box::new(self.parse_unary()?);
                expr = Node::OperationExpression(OperationExpression {
                    location: self.previous().location.clone(),
                    operator: Operator::DividedBy,
                    left: Box::new(expr),
                    right: Some(right),
                });
                continue;
            }
            if self.match_token(&[TokenKind::Modulo]) {
                let right = Box::new(self.parse_unary()?);
                expr = Node::OperationExpression(OperationExpression {
                    location: self.previous().location.clone(),
                    operator: Operator::Modulo,
                    left: Box::new(expr),
                    right: Some(right),
                });
                continue;
            }
            break;
        }
        
        Ok(expr)
    }
    
    fn parse_unary(&mut self) -> Result<Node, String> {
        if self.match_token(&[TokenKind::Not, TokenKind::Minus]) {
            let op = match self.previous().kind {
                TokenKind::Not => Operator::Not,
                TokenKind::Minus => Operator::Minus,
                _ => unreachable!(),
            };
            let right = Box::new(self.parse_unary()?);
            return Ok(Node::OperationExpression(OperationExpression {
                location: self.previous().location.clone(),
                operator: op,
                left: Box::new(Node::LiteralExpression(LiteralExpression {
                    location: self.previous().location.clone(),
                    value: LiteralValue::Number(0.0),
                })),
                right: Some(right),
            }));
        }
        
        if self.match_token(&[TokenKind::Start]) {
            let location = self.previous().location.clone();
            let expr = self.parse_unary()?;
            return Ok(Node::StartExpression(StartExpression {
                location,
                expression: Box::new(expr),
            }));
        }
        
        if self.match_token(&[TokenKind::Wait]) {
            let location = self.previous().location.clone();
            if self.match_token(&[TokenKind::For]) {
                let expr = self.parse_unary()?;
                return Ok(Node::WaitExpression(WaitExpression {
                    location,
                    expression: Box::new(expr),
                }));
            } else {
                return Err("Expected 'for' after 'wait'".to_string());
            }
        }
        
        self.parse_call()
    }
    
    fn parse_call(&mut self) -> Result<Node, String> {
        // check for standalone function call: "call function name with args"
        if self.check(&TokenKind::Call) {
            return self.parse_call_expression();
        }
        
        let mut expr = self.parse_primary()?;
        
        loop {
            if self.match_token(&[TokenKind::Period]) {
                // property access or method call
                let name = self.parse_identifier()?;

                
                // check for method call - STRICTLY FORBIDDEN
                if self.match_token(&[TokenKind::LeftParen]) {
                    return Err(format!(
                        "{}:{}:{}: C-style function calls like 'obj.method()' are not allowed in Layman. Please use 'call function method on obj' or 'call method using obj'. Layman is designed to be pure English.",
                        expr.location().file,
                        expr.location().line,
                        expr.location().column
                    ));
                } else {
                    // property access
                    expr = Node::AccessExpression(AccessExpression {
                        location: expr.location(),
                        object: Box::new(expr),
                        property: name,
                    });
                }
            } else if self.match_token(&[TokenKind::LeftBracket]) {
                // index access
                let index = self.parse_expression()?;
                self.consume(&TokenKind::RightBracket, "Expected ']' after index")?;
                
                expr = Node::IndexExpression(IndexExpression {
                    location: expr.location(),
                    object: Box::new(expr),
                    index: Box::new(index),
                });
            } else {
                break;
            }
        }
        
        Ok(expr)
    }
    
    fn parse_throw_statement(&mut self) -> Result<Node, String> {
        let location = self.previous().location.clone();
        // check if we have an expression to throw
        self.skip_newlines();
        if self.is_at_end() || self.check(&TokenKind::Eof) || self.check(&TokenKind::Newline) {
            return Err(format!("{}:{}:{}: throw statement requires an expression", location.file, location.line, location.column));
        }
        let expression = self.parse_expression()?;
        Ok(Node::ThrowStatement(ThrowStatement {
            location,
            expression: Box::new(expression),
        }))
    }
    
    fn parse_try_catch_statement(&mut self) -> Result<Node, String> {
        let location = self.previous().location.clone();
        
        // parse try block statements until "catch" or "end try"
        let mut try_statements = Vec::new();
        self.skip_newlines();
        
        loop {
            if self.is_at_end() || self.check(&TokenKind::Eof) {
                break;
            }
            self.skip_newlines();
            
            if self.check(&TokenKind::Catch) {
                break;
            }
            
            // check for "end try"
            if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                if s.to_lowercase() == "end" {
                    if let Some(TokenKind::Try) = self.peek_kind_skip_newlines(1) {
                        break;
                    }
                }
            }
            
            let stmt = self.parse_statement()?;
            try_statements.push(stmt);
            
            self.skip_newlines();
        }
        
        let try_block = if try_statements.is_empty() {
            Box::new(Node::LiteralExpression(LiteralExpression {
                location: location.clone(),
                value: LiteralValue::Void,
            }))
        } else if try_statements.len() == 1 {
            Box::new(try_statements.remove(0))
        } else {
            Box::new(Node::Program(Program {
                location: location.clone(),
                statements: try_statements,
            }))
        };
        
        // check for catch
        let (catch_block, error_variable) = if self.match_token(&[TokenKind::Catch]) {
            // parse error variable name (optional)
            let error_var = if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                let var_name = s.clone();
                self.advance();
                Some(var_name)
            } else {
                None
            };
            
            // parse catch block statements until "end try"
            let mut catch_statements = Vec::new();
            self.skip_newlines();
            
            loop {
                if self.is_at_end() || self.check(&TokenKind::Eof) {
                    break;
                }
                self.skip_newlines();
                
                // check for "end try"
                if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                    if s.to_lowercase() == "end" {
                        if let Some(TokenKind::Try) = self.peek_kind_skip_newlines(1) {
                            break;
                        }
                    }
                }
                
                let stmt = self.parse_statement()?;
                catch_statements.push(stmt);
                
                self.skip_newlines();
            }
            
            let catch = if catch_statements.is_empty() {
                Box::new(Node::LiteralExpression(LiteralExpression {
                    location: location.clone(),
                    value: LiteralValue::Void,
                }))
            } else if catch_statements.len() == 1 {
                Box::new(catch_statements.remove(0))
            } else {
                Box::new(Node::Program(Program {
                    location: location.clone(),
                    statements: catch_statements,
                }))
            };
            
            (Some(catch), error_var)
        } else {
            (None, None)
        };
        
        // check for "end try" (optional, for clarity)
        self.skip_newlines();
        if let TokenKind::Identifier(ref s) = &self.current_token().kind {
            if s.to_lowercase() == "end" {
                self.advance();
                if self.match_token(&[TokenKind::Try]) {
                    // consumed "end try"
                }
            }
        }
        
        Ok(Node::TryCatchStatement(TryCatchStatement {
            location,
            try_block,
            catch_block,
            error_variable,
        }))
    }
    
    fn parse_describe_statement(&mut self) -> Result<Node, String> {
        let location = self.previous().location.clone();
        
        // parse suite name (string or identifier)
        self.skip_newlines();
        if self.is_at_end() || self.check(&TokenKind::Eof) {
            return Err(format!("{}:{}:{}: describe statement requires a suite name", location.file, location.line, location.column));
        }
        
        // check if next token is a keyword which means missing name
        if matches!(self.current_token().kind, TokenKind::Test | TokenKind::Describe | TokenKind::Expect) {
            return Err(format!("{}:{}:{}: describe statement requires a suite name", location.file, location.line, location.column));
        }
        
        let suite_name = if matches!(self.current_token().kind, TokenKind::Text(_)) {
            if let TokenKind::Text(name) = &self.current_token().kind {
                let name_str = name.clone();
                self.advance();
                name_str
            } else {
                self.parse_identifier()?
            }
        } else {
            self.parse_identifier()?
        };
        
        // parse body - multiple statements until "end describe" or "end"
        let mut statements = Vec::new();
        
        loop {
            self.skip_newlines();
            
            // check for "end"
            if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                if s.to_lowercase() == "end" {
                    // Check for "end describe"
                    if let Some(TokenKind::Describe) = self.peek_kind_skip_newlines(1) {
                        self.advance(); // consume "end"
                        self.advance(); // consume "describe"
                        break;
                    }
                    
                    // Check for mismatch (e.g. "end test")
                    if let Some(TokenKind::Test) = self.peek_kind_skip_newlines(1) {
                        return Err("Mismatched closing block. Found 'end test' inside 'describe' block. Expected 'end describe'.".to_string());
                    }
                    
                    // Allow generic "end" for now, but strictly it should be paired
                    self.advance(); // consume "end"
                    break;
                }
            }
            
            if self.is_at_end() || self.check(&TokenKind::Eof) {
                break;
            }
            
            // parse statement
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }
        
        // create a call expression that will be handled by test framework
        // wrap statements in a Program node
        let body_program = Node::Program(Program {
            location: location.clone(),
            statements,
        });
        
        Ok(Node::CallExpression(CallExpression {
            location: location.clone(),
            function_name: "describe".to_string(),
            arguments: vec![
                Node::LiteralExpression(LiteralExpression {
                    location: location.clone(),
                    value: LiteralValue::String(suite_name),
                }),
                body_program,
            ],
        }))
    }
    
    fn parse_test_statement(&mut self) -> Result<Node, String> {
        let location = self.previous().location.clone();
        
        // parse test name (string or identifier)
        self.skip_newlines();
        if self.is_at_end() || self.check(&TokenKind::Eof) {
            return Err(format!("{}:{}:{}: test statement requires a test name", location.file, location.line, location.column));
        }
        
        // check if next token is a keyword (test/describe) which means missing name
        if matches!(self.current_token().kind, TokenKind::Test | TokenKind::Describe | TokenKind::Expect) {
            return Err(format!("{}:{}:{}: test statement requires a test name", location.file, location.line, location.column));
        }
        
        let test_name = if matches!(self.current_token().kind, TokenKind::Text(_)) {
            if let TokenKind::Text(name) = &self.current_token().kind {
                let name_str = name.clone();
                self.advance();
                name_str
            } else {
                self.parse_identifier()?
            }
        } else {
            self.parse_identifier()?
        };
        
        // parse body - multiple statements until "end test" or "end"
        let mut statements = Vec::new();
        
        loop {
            self.skip_newlines();
            
            // check for "end"
            if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                if s.to_lowercase() == "end" {
                    // Check for "end test"
                    if let Some(TokenKind::Test) = self.peek_kind_skip_newlines(1) {
                        self.advance(); // consume "end"
                        self.advance(); // consume "test"
                        break;
                    }
                    
                    // Check for mismatch (e.g. "end describe")
                    if let Some(TokenKind::Describe) = self.peek_kind_skip_newlines(1) {
                        return Err("Mismatched closing block. Found 'end describe' inside 'test' block. Expected 'end test'.".to_string());
                    }
                    
                    self.advance(); // consume "end"
                    break;
                }
            }
            
            if self.is_at_end() || self.check(&TokenKind::Eof) {
                break;
            }
            
            // parse statement
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }
        
        Ok(Node::TestStatement(TestStatement {
            location,
            name: test_name,
            body: Box::new(Node::Program(Program {
                location: self.current_token().location.clone(),
                statements,
            })),
        }))

        

    }
    
    fn parse_expect_statement(&mut self) -> Result<Node, String> {
        let location = self.previous().location.clone();
        
        // parse: "expect actual is expected"
        self.skip_newlines();
        if self.is_at_end() || self.check(&TokenKind::Eof) {
            return Err(format!("{}:{}:{}: expect statement requires an actual value", location.file, location.line, location.column));
        }
        let actual = self.parse_expression()?;
        
        // check if 'is' was already consumed by parse_expression (as an operator)
        if let Node::OperationExpression(ref op) = actual {
            if op.operator == Operator::Equals {
                // 'is' was treated as equality operator
                // extract left and right operands
                return Ok(Node::CallExpression(CallExpression {
                    location: location.clone(),
                    function_name: "expect".to_string(),
                    arguments: vec![*op.left.clone(), *op.right.clone().unwrap()],
                }));
            }
        }
        
        // consume "is"
        self.skip_newlines();
        // println!("DEBUG: parse_expect_statement check is: {:?}", self.current_token());
        if !self.check(&TokenKind::Is) {
            return Err(format!("{}:{}:{}: expect statement requires 'is' keyword, got {:?}", location.file, location.line, location.column, self.current_token()));
        }
        self.consume(&TokenKind::Is, "Expected 'is' in expect statement")?;
        
        self.skip_newlines();
        if self.is_at_end() || self.check(&TokenKind::Eof) {
            return Err(format!("{}:{}:{}: expect statement requires an expected value", location.file, location.line, location.column));
        }
        let expected = self.parse_expression()?;
        
        // create a call expression: expect(actual, expected)
        Ok(Node::CallExpression(CallExpression {
            location: location.clone(),
            function_name: "expect".to_string(),
            arguments: vec![actual, expected],
        }))
    }
    
    fn parse_print_statement(&mut self) -> Result<Node, String> {
        let location = self.previous().location.clone();
        
        // skip optional "the variable" - handle explicitly
        if self.check(&TokenKind::The) {
            self.advance(); // consume "the"
            if self.check(&TokenKind::Variable) {
                self.advance(); // consume "variable"
            }
        }
        
        let expr = self.parse_expression()?;
        
        let call_expr = Node::CallExpression(CallExpression {
            location: location.clone(),
            function_name: "print".to_string(),
            arguments: vec![expr],
        });
        
        Ok(Node::ExpressionStatement(ExpressionStatement {
            location,
            expression: Box::new(call_expr),
        }))
    }
    
    fn parse_call_expression(&mut self) -> Result<Node, String> {
        let location = self.previous().location.clone();
        self.advance(); // consume "call"
        
        // skip optional "function"
        self.match_token(&[TokenKind::Function]);
        
        let mut function_name = self.parse_identifier()?;
        
        // check for dot notation (e.g. module.function)
        while self.match_token(&[TokenKind::Period]) {
            let part = self.parse_identifier()?;
            function_name = format!("{}.{}", function_name, part);
        }
        
        // check for method call: "call function X on Y with Z"
        if let TokenKind::Identifier(ref s) = &self.current_token().kind {
            if s.to_lowercase().trim() == "on" {
                // this is a method call
                self.advance(); // consume "on"
                let object = Box::new(self.parse_expression()?); // parse the object
                
                let mut arguments = Vec::new();
                
                // check for "with" for arguments
        if self.match_token(&[TokenKind::With]) {
            // handle argument parsing - check for "value", "amount", etc.
                    loop {
                        // skip optional argument keywords like "value", "amount", "argument"
                        if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                            let low = s.to_lowercase();
                            let low_str = low.trim();
                            if matches!(low_str, "value" | "amount" | "argument") {
                                self.advance();
                            }
                        }
                        
                        // parse a single argument
                        let arg_value = self.parse_term()?;
                        arguments.push(arg_value);
                        
                        // check for comma or "and"
                        self.skip_newlines();
                        
                        // CRITICAL: Check if "and" is followed by "wait" (terminator for run concurrently)
                        if self.check(&TokenKind::And) {
                            if let Some(k) = self.peek_kind_skip_newlines(1) {
                                if matches!(k, TokenKind::Wait) {
                                    break;
                                }
                            }
                        }
                        
                        if !self.match_token(&[TokenKind::Comma, TokenKind::And]) {
                            break;
                        } else {
                            // optional "argument", "value", "amount" after "and"
                            if let Some(TokenKind::Identifier(ref s)) = self.peek_kind_skip_newlines(0) {
                                let low = s.to_lowercase();
                                let low_str = low.trim();
                                if matches!(low_str, "argument" | "value" | "amount") {
                                    self.advance();
                                }
                            }
                            continue;
                        }
                    }
                }
                
                return Ok(Node::MethodCall(MethodCall {
                    location,
                    object,
                    method_name: function_name,
                    arguments,
                }));
            }
        }
        
        // regular function call
        let mut arguments = Vec::new();
        
        if self.match_token(&[TokenKind::With]) {
            loop {
                // skip optional "argument"
                if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                    if s.to_lowercase().trim() == "argument" {
                        self.advance();
                    }
                }
                
                // parse a single argument - use parse_term() to avoid consuming "and" between arguments
                // parse_term() stops at comparison operators, so "and" between arguments won't be consumed
                let arg_value = self.parse_term()?;
                
                arguments.push(arg_value);
                
                // CRITICAL: Check if "and" is followed by "wait" (terminator for run concurrently)
                self.skip_newlines();
                if self.check(&TokenKind::And) {
                    if let Some(k) = self.peek_kind_skip_newlines(1) {
                        if matches!(k, TokenKind::Wait) {
                            break;
                        }
                    }
                }
                
                // check for comma or "and" between arguments
                if self.match_token(&[TokenKind::Comma, TokenKind::And]) {
                    // optional "argument" after "and"
                    if let Some(TokenKind::Identifier(ref s)) = self.peek_kind_skip_newlines(0) {
                         if s.to_lowercase().trim() == "argument" {
                             self.advance();
                         }
                    }
                    continue;
                } else {
                    break;
                }

            }
        }
        
        Ok(Node::CallExpression(CallExpression {
            location,
            function_name,
            arguments,
        }))
    }
    
    fn _parse_call_expression_with_base(&mut self, base: Node) -> Result<Node, String> {
        // this is for method calls - simplified for now
        // self.parse_call_expression() // BUG: this consumes "call" token which might not be there!
        Ok(base)
    }
    


    fn parse_primary(&mut self) -> Result<Node, String> {
        let location = self.current_token().location.clone();
        
        if self.match_token(&[TokenKind::True]) {
            return Ok(Node::LiteralExpression(LiteralExpression {
                location,
                value: LiteralValue::Bool(true),
            }));
        }
        
        if self.match_token(&[TokenKind::False]) {
            return Ok(Node::LiteralExpression(LiteralExpression {
                location,
                value: LiteralValue::Bool(false),
            }));
        }
        
        if self.match_token(&[TokenKind::Nothing]) {
            return Ok(Node::LiteralExpression(LiteralExpression {
                location,
                value: LiteralValue::Nothing,
            }));
        }
        
        if self.match_token(&[TokenKind::Function]) {
            if self.match_token(&[TokenKind::Of]) {
                let mut parameters = Vec::new();
                
                // Parse first parameter
                let param_name = self.parse_identifier()?;
                parameters.push(Parameter { name: param_name, type_annotation: None });
                
                // Parse additional parameters
                while self.match_token(&[TokenKind::Comma]) {
                    let param_name = self.parse_identifier()?;
                    parameters.push(Parameter { name: param_name, type_annotation: None });
                }
                
                self.consume(&TokenKind::Returning, "Expected 'returning' in lambda")?;
                let body = self.parse_expression()?;
                
                let body_stmt = Node::ReturnStatement(ReturnStatement {
                    location: body.location(),
                    expression: Some(Box::new(body)),
                });
                
                return Ok(Node::FunctionDeclaration(FunctionDeclaration {
                    location: self.previous().location.clone(),
                    name: "<lambda>".to_string(),
                    parameters,
                    return_type: None,
                    body: Box::new(body_stmt),
                    is_async: false,
                }));
            } else {
                return Err("Expected 'of' after 'function' in lambda expression".to_string());
            }
        }
        

        
        
        if let TokenKind::Text(ref s) = &self.current_token().kind {
            let text = s.clone();
            self.advance();
            return Ok(Node::LiteralExpression(LiteralExpression {
                location,
                value: LiteralValue::String(text),
            }));
        }
        
        if let TokenKind::Number(n) = self.current_token().kind {
            self.advance();
            return Ok(Node::LiteralExpression(LiteralExpression {
                location,
                value: LiteralValue::Number(n),
            }));
        }
        
        // list literal parsing
        if self.check(&TokenKind::The) || self.check(&TokenKind::A) {
            let is_list = if self.check(&TokenKind::The) {
                // check for "the list"
                if let Some(kind) = self.peek_kind_skip_newlines(1) {
                    if matches!(kind, TokenKind::Identifier(ref s) if s.to_lowercase() == "list") {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                // check for "a list" or "an empty list"
                if let Some(kind) = self.peek_kind_skip_newlines(1) {
                    matches!(kind, TokenKind::Identifier(ref s) if s.to_lowercase() == "list" || s.to_lowercase() == "empty")
                } else {
                    false
                }
            };
            
            let is_dictionary = if self.check(&TokenKind::The) {
                // check for "the dictionary"
                if let Some(kind) = self.peek_kind_skip_newlines(1) {
                    matches!(kind, TokenKind::Dictionary)
                } else {
                    false
                }
            } else {
                // check for "a dictionary" or "an empty dictionary"
                if let Some(kind) = self.peek_kind_skip_newlines(1) {
                    matches!(kind, TokenKind::Dictionary) || 
                    (matches!(kind, TokenKind::Identifier(ref s) if s.to_lowercase() == "empty") && 
                     self.peek_kind_skip_newlines(2).map_or(false, |k| matches!(k, TokenKind::Dictionary)))
                } else {
                    false
                }
            };
            
            if is_dictionary {
                self.advance(); // consume "the"/"a"/"an"
                
                // check for "empty"
                if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                    if s.to_lowercase() == "empty" {
                        self.advance();
                    }
                }
                
                self.consume(&TokenKind::Dictionary, "Expected 'dictionary'")?;
                
                let mut args = Vec::new();
                
                // check for "containing" or "with"
                let is_with_or_containing = match &self.current_token().kind {
                    TokenKind::With => true,
                    TokenKind::Identifier(s) if s.to_lowercase() == "containing" || s.to_lowercase() == "with" => true,
                    _ => false
                };
                
                if is_with_or_containing {
                    self.advance();
                        
                        // parse entries: "key" is value, ...
                        loop {
                             // parse key
                             let key_expr = self.parse_term()?;
                             args.push(key_expr);
                             
                             // expect "is" or "equals"
                             if self.match_token(&[TokenKind::Is]) || self.match_token(&[TokenKind::Equals]) {
                                 // ok
                             } else {
                                 return Err("Expected 'is' after dictionary key".to_string());
                             }
                             
                             let value_expr = self.parse_term()?;
                             args.push(value_expr);
                             
                             // check for comma
                             if self.check(&TokenKind::Comma) {
                                 self.advance();
                             }
                             
                             // check if we're done
                             if self.is_at_end() || self.check(&TokenKind::Eof) || self.check(&TokenKind::Newline) {
                                 break;
                             }
                             
                             // heuristic: if next token is not start of an expression, break
                             // keys are usually strings or identifiers
                             if !matches!(self.current_token().kind, TokenKind::Text(_) | TokenKind::Identifier(_)) {
                                 break;
                             }
                        }
                    }
                
                return Ok(Node::CallExpression(CallExpression {
                    location: location.clone(),
                    function_name: "create_dictionary".to_string(),
                    arguments: args,
                }));
            }
            
            if is_list {
                // consume "the"/"a"/"an"
                self.advance();
                
                // check for "empty list"
                if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                    if s.to_lowercase() == "empty" {
                        self.advance(); // consume "empty"
                    }
                }

                // consume "list"
                if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                    if s.to_lowercase() == "list" {
                        self.advance();
                        
                        // check for "containing" or "with"
                        let mut elements = Vec::new();
                        let mut has_elements = false;
                        
                        if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                            if s.to_lowercase() == "containing" || s.to_lowercase() == "with" {
                                self.advance();
                                has_elements = true;
                            }
                        }
                        
                        if has_elements {
                            // parse list elements: "one two three" or "1, 2, 3"
                            loop {
                                // skip optional "and"
                                if self.check(&TokenKind::And) {
                                    self.advance();
                                }
                                
                                // parse an element
                                let elem = self.parse_term()?;
                                elements.push(elem);
                                
                                // check for comma
                                if self.check(&TokenKind::Comma) {
                                    self.advance();
                                }
                                
                                // check if we're done
                                if self.is_at_end() || self.check(&TokenKind::Eof) || self.check(&TokenKind::Newline) {
                                    break;
                                }
                                
                                // heuristic: if next token is not start of an expression, break
                                // This is tricky in natural language. 
                                // For now, assume list ends at newline or when we can't parse another term
                                // But parse_term might consume too much.
                                // Let's rely on commas or "and" or just sequence of values.
                                
                                // If next is a keyword that starts a statement, break
                                if matches!(self.current_token().kind, TokenKind::Define | TokenKind::Return | TokenKind::If | TokenKind::For | TokenKind::While | TokenKind::Test | TokenKind::Expect | TokenKind::Describe | TokenKind::The) {
                                    break;
                                }
                                
                                // check for "print" identifier
                                if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                                    let low = s.to_lowercase();
                                    if low == "print" || low == "the" {
                                        break;
                                    }
                                }
                            }
                        }
                        
                        return Ok(Node::CallExpression(CallExpression {
                            location,
                            function_name: "create_list".to_string(),
                            arguments: elements,
                        }));
                    }
                }
            }
        }

        // item access: "item X of Y"
        if let TokenKind::Identifier(ref s) = &self.current_token().kind {
            if s.to_lowercase() == "item" {
                // check lookahead to see if this is "item <index> of <object>"
                // or just a variable named "item"
                // if next token is a statement terminator or operator, treat as variable
                if let Some(next) = self.peek_kind(1) {
                    if matches!(next, TokenKind::Newline | TokenKind::Eof | TokenKind::Plus | TokenKind::Minus | TokenKind::Times | TokenKind::DividedBy | TokenKind::Modulo | TokenKind::Equals | TokenKind::Is | TokenKind::GreaterThan | TokenKind::LessThan | TokenKind::And | TokenKind::Or | TokenKind::Then | TokenKind::Do) {
                         // treat as variable
                         // fall through to default identifier handling
                    } else {
                        self.advance(); // consume "item"
                        
                        let index = Box::new(self.parse_term()?);
                
                // expect "of" or "in"
                if matches!(self.current_token().kind, TokenKind::Of | TokenKind::In) {
                    self.advance();
                    let object = Box::new(self.parse_term()?);
                    
                    return Ok(Node::IndexExpression(IndexExpression {
                        location,
                        object,
                        index,
                    }));
                } else if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                    if s.to_lowercase() == "of" || s.to_lowercase() == "in" {
                        self.advance();
                        let object = Box::new(self.parse_term()?);
                        
                        return Ok(Node::IndexExpression(IndexExpression {
                            location,
                            object,
                            index,
                        }));
                    }
                }
                
                return Err(format!("Expected 'of' or 'in' after 'item <index>', got: {:?}", self.current_token()));
            }
        }
    }
}

        
        // object creation: "is a new Type with"
        // use peek to check ahead without consuming tokens
        // NOTE: this must come BEFORE treating "a" as a variable, otherwise "is a new" won't work
        if self.check(&TokenKind::Is) {
            if let Some(kind_ahead) = self.peek_kind_skip_newlines(1) {
                if matches!(kind_ahead, TokenKind::A) {
                    if let Some(kind_ahead2) = self.peek_kind_skip_newlines(2) {
                        if matches!(kind_ahead2, TokenKind::New) {
                            // consume "is a new"
                            self.advance(); // consume "is"
                            self.advance(); // consume "a"
                            self.advance(); // consume "new"
                            
                            // parse class name
                            let class_name = self.parse_identifier()?;
                            
                            // parse property assignments after "with"
                            let mut properties = Vec::new();
                            if self.match_token(&[TokenKind::With]) {
                                // skip newlines for multi-line property assignments
                                self.skip_newlines();
                                
                                loop {
                                    // each property: "name which is value"
                                    let prop_location = self.current_token().location.clone();
                                    
                                    // Check if this identifier is followed by "which", "as", "is", or "equals"
                                    // If not, it's likely the start of a new statement (e.g. "print")
                                    if let Some(next_kind) = self.peek_kind_skip_newlines(1) {
                                        let is_prop_assign = match next_kind {
                                            TokenKind::Identifier(s) => {
                                                let s = s.to_lowercase();
                                                s == "which" || s == "as" || s == "is" || s == "equals"
                                            },
                                            TokenKind::As | TokenKind::Is | TokenKind::Equals => true,
                                            _ => false
                                        };
                                        
                                        if !is_prop_assign {
                                            break;
                                        }
                                    }

                                    let prop_name = self.parse_identifier()?;
                                    
                                    // skip "which is", "as", "is", or "equals"
                                    if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                                        let s = s.to_lowercase();
                                        if s == "which" {
                                            self.advance();
                                            if self.match_token(&[TokenKind::Is]) {
                                                // consumed "is"
                                            } else if let TokenKind::Identifier(ref s2) = &self.current_token().kind {
                                                if s2.to_lowercase() == "is" {
                                                    self.advance();
                                                }
                                            }
                                        } else if s == "as" || s == "is" || s == "equals" {
                                            self.advance();
                                        }
                                    } else if self.match_token(&[TokenKind::As, TokenKind::Is, TokenKind::Equals]) {
                                        // consumed
                                    }
                                    
                                    let prop_value = Box::new(self.parse_expression()?);
                                    
                                    properties.push(PropertyAssignment {
                                        location: prop_location,
                                        name: prop_name,
                                        value: prop_value,
                                    });
                                    
                                    // check for next property (after newline) or end
                                    self.skip_newlines();
                                    if self.is_at_end() || self.check(&TokenKind::Eof) {
                                        break;
                                    }
                                    // if next token is not an identifier (property name), we're done
                                    if !matches!(&self.current_token().kind, TokenKind::Identifier(_) | TokenKind::TypeIdentifier(_)) {
                                        break;
                                    }
                                }
                            }
                            
                            return Ok(Node::ObjectCreation(ObjectCreation {
                                location,
                                class_name,
                                properties,
                            }));
                        }
                    }
                }
            }
        }
        
        // handle property access: "get X from Y"
        if let TokenKind::Identifier(ref s) = &self.current_token().kind {
            if s.to_lowercase().trim() == "get" {
                let prop_location = self.current_token().location.clone();
                self.advance(); // consume "get"
                self.skip_newlines();
                let property_name = self.parse_identifier()?;
                
                // expect "from" - skip newlines first
                self.skip_newlines();
                if self.match_token(&[TokenKind::From]) {
                    // consume "from"
                    self.skip_newlines();
                    let object = Box::new(self.parse_term()?);
                    return Ok(Node::AccessExpression(AccessExpression {
                        location: prop_location,
                        object,
                        property: property_name,
                    }));
                }
                return Err(format!("Expected 'from' after 'get {}', got: {:?}", property_name, self.current_token()));
            }
        }
        
        // handle variable references - allow single-letter keywords as identifiers
        // this handles cases like "a modulo b" where "a" is a variable name
        // but NOT when it's part of "is a new" (which we checked above)
        // object creation: "new Type" or "a new Type"

        let is_new = if self.check(&TokenKind::New) {
            true
        } else if self.check(&TokenKind::A) {
            if let Some(kind) = self.peek_kind_skip_newlines(1) {
                matches!(kind, TokenKind::New)
            } else {
                false
            }
        } else {
            false
        };

        if is_new {
            if self.check(&TokenKind::A) {
                self.advance(); // consume "a"
            }
            self.consume(&TokenKind::New, "Expected 'new'")?;
            
            // parse class name
            let class_name = self.parse_identifier()?;
            
            // parse property assignments after "with"
            let mut properties = Vec::new();
            if self.match_token(&[TokenKind::With]) {
                // skip newlines for multi-line property assignments
                self.skip_newlines();
                
                loop {
                    // each property: "name which is value" or "name as value"
                    let prop_location = self.current_token().location.clone();
                    
                    // Check if this identifier is followed by "which", "as", "is", or "equals"
                    // If not, it's likely the start of a new statement (e.g. "print")
                    if let Some(next_kind) = self.peek_kind_skip_newlines(1) {
                        let is_prop_assign = match next_kind {
                            TokenKind::Identifier(s) => {
                                let s = s.to_lowercase();
                                s == "which" || s == "as" || s == "is" || s == "equals"
                            },
                            TokenKind::As | TokenKind::Is | TokenKind::Equals => true,
                            _ => false
                        };
                        
                        if !is_prop_assign {
                            break;
                        }
                    }

                    let prop_name = self.parse_identifier()?;
                    
                    // skip "which is", "as", "is", or "equals"
                    if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                        let s = s.to_lowercase();
                        if s == "which" {
                            self.advance();
                            if self.match_token(&[TokenKind::Is]) {
                                // consumed "is"
                            } else if let TokenKind::Identifier(ref s2) = &self.current_token().kind {
                                if s2.to_lowercase() == "is" {
                                    self.advance();
                                }
                            }
                        } else if s == "as" || s == "is" || s == "equals" {
                            self.advance();
                        }
                    } else if self.match_token(&[TokenKind::As, TokenKind::Is, TokenKind::Equals]) {
                        // consumed
                    }
                    
                    let prop_value = Box::new(self.parse_expression()?);
                    
                    properties.push(PropertyAssignment {
                        location: prop_location,
                        name: prop_name,
                        value: prop_value,
                    });
                    
                    // check for next property (after newline) or end
                    self.skip_newlines();
                    
                    // check for comma or "and"
                    if self.match_token(&[TokenKind::Comma, TokenKind::And]) {
                        self.skip_newlines();
                    }
                    
                    if self.is_at_end() || self.check(&TokenKind::Eof) {
                        break;
                    }
                    // if next token is not an identifier (property name), we're done
                    if !matches!(&self.current_token().kind, TokenKind::Identifier(_) | TokenKind::TypeIdentifier(_)) {
                        break;
                    }
                }
            }
            
            return Ok(Node::ObjectCreation(ObjectCreation {
                location,
                class_name,
                properties,
            }));
        }

        // handle variable references - allow single-letter keywords as identifiers
        // this handles cases like "a modulo b" where "a" is a variable name
        // but NOT when it's part of "is a new" (which we checked above)
        if matches!(self.current_token().kind, TokenKind::A) {
            let var_name = match self.current_token().kind {
                TokenKind::A => {
                    self.advance();
                    "a".to_string()
                }
                _ => unreachable!(),
            };
            return Ok(Node::VariableExpression(VariableExpression {
                location,
                identifier: var_name,
            }));
        }
        

        
        // identifier
        if matches!(&self.current_token().kind, TokenKind::Identifier(_) | TokenKind::TypeIdentifier(_)) {
            let ident = self.parse_identifier()?;
            return Ok(Node::VariableExpression(VariableExpression {
                location,
                identifier: ident,
            }));
        }
        
        Err(format!("Unexpected token: {:?}", self.current_token()))
    }
    

    
    fn parse_class_declaration(&mut self, location: Location) -> Result<Node, String> {
        // parse class name
        let class_name = self.parse_identifier()?;
        
        // expect "that has" or "that extends"
        self.consume(&TokenKind::That, "Expected 'that' after class name")?;
        let extends_class = if let TokenKind::Identifier(ref s) = &self.current_token().kind {
            if s.to_lowercase() == "extends" {
                self.advance();
                Some(self.parse_identifier()?)
            } else if s.to_lowercase() == "has" {
                self.advance();
                None
            } else {
                return Err("Expected 'has' or 'extends' after 'that'".to_string());
            }
        } else {
            return Err("Expected 'has' or 'extends' after 'that'".to_string());
        };
        
        // skip newlines
        self.skip_newlines();
        
        // parse properties and methods
        let mut properties = Vec::new();
        let mut methods = Vec::new();
        let start_indent = self.current_token().location.column;
        
        loop {
            // check if we've hit end of file
            if self.is_at_end() || self.check(&TokenKind::Eof) {
                break;
            }
            
            // check indentation - if we're back at column 0 or less indented, we're done
            let current_indent = self.current_token().location.column;
            if current_indent < start_indent {
                break;
            }
            
            // check for "define" keyword (for methods)
            if self.check(&TokenKind::Define) {
                let method_loc = self.current_token().location.clone();
                self.advance(); // consume "define"
                
                // expect "function"
                if self.check(&TokenKind::Function) {
                    self.advance(); // consume "function"
                    let method_node = self.parse_function_declaration(method_loc, false)?;
                    if let Node::FunctionDeclaration(method) = method_node {
                        methods.push(method);
                    } else {
                        return Err("Expected function declaration".to_string());
                    }
                    self.skip_newlines(); // Skip newlines after method
                    continue;
                } else {
                    return Err("Expected 'function' after 'define' in class body".to_string());
                }
            }
            
            // check for "property" keyword
            if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                if s.to_lowercase() == "property" {
                    self.advance(); // consume "property"
                    
                    // parse property name
                    let prop_name = self.parse_identifier()?;
                    
                    // expect "which is"
                    if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                        if s.to_lowercase() == "which" {
                            self.advance();
                            // check for "is" - could be Identifier or Is token
                            if let TokenKind::Identifier(ref s2) = &self.current_token().kind {
                                if s2.to_lowercase() == "is" {
                                    self.advance();
                                }
                            } else if matches!(&self.current_token().kind, TokenKind::Is) {
                                self.advance();
                            }
                        }
                    } else if matches!(&self.current_token().kind, TokenKind::Is) {
                        // just "is" without "which"
                        self.advance();
                    }
                    
                    // parse property type - can be "which is Type" or "of type Type"
                    let prop_type = if matches!(self.current_token().kind, TokenKind::Of) {
                        // "of type Type" syntax
                        self.advance(); // consume "of"
                        if let TokenKind::Identifier(ref s2) = &self.current_token().kind {
                            if s2.to_lowercase() == "type" {
                                self.advance(); // consume "type"
                                self.parse_type()?
                            } else {
                                return Err(format!("Expected 'type' after 'of', got: {:?}", self.current_token()));
                            }
                        } else {
                            return Err(format!("Expected 'type' after 'of', got: {:?}", self.current_token()));
                        }
                    } else if let TokenKind::Identifier(ref s) = &self.current_token().kind {
                        if s.to_lowercase() == "of" {
                            // "of type Type" syntax
                            self.advance(); // consume "of"
                            if let TokenKind::Identifier(ref s2) = &self.current_token().kind {
                                if s2.to_lowercase() == "type" {
                                    self.advance(); // consume "type"
                                    self.parse_type()?
                                } else {
                                    return Err(format!("Expected 'type' after 'of', got: {:?}", self.current_token()));
                                }
                            } else {
                                return Err(format!("Expected 'type' after 'of', got: {:?}", self.current_token()));
                            }
                        } else {
                            // If "which is" was handled, and "of type" was not found,
                            // then the type should be directly next.
                            // If it's an identifier, it could be a basic type or a custom class type.
                            // If it's a TypeIdentifier, it's a type.
                            if matches!(&self.current_token().kind, TokenKind::TypeIdentifier(_)) {
                                self.parse_type()?
                            } else {
                                // handle Text, Number, String, Bool, etc. as identifiers
                                let type_name = s.clone();
                                self.advance();
                                // convert to Type
                                match type_name.to_lowercase().as_str() {
                                    "text" | "string" => Type::BasicType(BasicType::String),
                                    "number" => Type::BasicType(BasicType::Number),
                                    "bool" | "boolean" => Type::BasicType(BasicType::Bool),
                                    "void" | "nothing" => Type::BasicType(BasicType::Void),
                                    _ => Type::GenericType(type_name), // custom class type
                                }
                            }
                        }
                    } else if matches!(&self.current_token().kind, TokenKind::TypeIdentifier(_) | TokenKind::Any | TokenKind::Dictionary) {
                        self.parse_type()?
                    } else {
                        return Err(format!("Expected type, got: {:?}", self.current_token()));
                    };
                    
                    properties.push(PropertyDeclaration {
                        location: self.previous().location.clone(),
                        name: prop_name,
                        type_annotation: prop_type,
                    });
                    
                    // skip newlines after property
                    self.skip_newlines();
                } else {
                    return Err(format!("Expected 'property' or 'define' in class body, got: {:?}", self.current_token()));
                }
            } else {
                 return Err(format!("Expected 'property' or 'define' in class body, got: {:?}", self.current_token()));
            }
        }
        
        Ok(Node::ClassDeclaration(ClassDeclaration {
            location,
            name: class_name,
            extends: extends_class,
            properties,
            methods,
        }))
    }
    
    fn parse_identifier(&mut self) -> Result<String, String> {
        match &self.current_token().kind {
            TokenKind::Identifier(ref s) | TokenKind::TypeIdentifier(ref s) => {
                let ident = s.clone();
                self.advance();
                Ok(ident)
            }
            // allow single-letter keywords as identifiers in identifier contexts
            // this handles cases like "the variable a of type Number"
            TokenKind::A => {
                self.advance();
                Ok("a".to_string())
            }
            TokenKind::Number(n) if *n == 0.0 => {
                self.advance();
                Ok("zero".to_string())
            }
            _ => Err(format!("Expected identifier, got: {:?}", self.current_token())),
        }
    }
    
    fn parse_type(&mut self) -> Result<Type, String> {
        // simplified type parsing
        // skip newlines before parsing type
        self.skip_newlines();
        
        // Check if current token is Returns - if so, we have a parsing error upstream
        if matches!(&self.current_token().kind, TokenKind::Returns) {
            return Err(format!("Unexpected 'returns' token when parsing type - this indicates a parser bug. Current token: {:?}", self.current_token()));
        }
        
        if matches!(&self.current_token().kind, TokenKind::Maybe) {
            self.advance(); // consume "maybe"
            let inner = self.parse_type()?;
            return Ok(Type::CompositeType(CompositeType::Maybe(Box::new(inner))));
        }
        
        if matches!(&self.current_token().kind, TokenKind::TypeIdentifier(_) | TokenKind::Identifier(_) | TokenKind::Dictionary | TokenKind::Any) {
            let name = if matches!(&self.current_token().kind, TokenKind::Dictionary) {
                self.advance();
                "dictionary".to_string()
            } else if matches!(&self.current_token().kind, TokenKind::Any) {
                self.advance();
                "any".to_string()
            } else {
                self.parse_identifier()?
            };
        
            match name.to_lowercase().as_str() {
                "number" => Ok(Type::BasicType(BasicType::Number)),
                "any" => Ok(Type::BasicType(BasicType::Any)),
                "string" | "text" => Ok(Type::BasicType(BasicType::String)),  // text kept for backward compat
                "bool" | "boolean" => Ok(Type::BasicType(BasicType::Bool)),    // boolean kept for backward compat
                "void" | "nothing" => Ok(Type::BasicType(BasicType::Void)),     // nothing kept for backward compat
                "list" => {
                    // check for "of <Type>"
                    if self.check(&TokenKind::Of) {
                        self.advance(); // consume "of"
                        let inner = self.parse_type()?;
                        Ok(Type::CompositeType(CompositeType::List(Box::new(inner))))
                    } else {
                        // generic list (List<Any>)
                        Ok(Type::CompositeType(CompositeType::List(Box::new(Type::BasicType(BasicType::Any)))))
                    }
                }
                "dictionary" => {
                // check for "of <Type>, <Type>"
                if self.check(&TokenKind::Of) {
                    self.advance(); // consume "of"
                    let key_type = self.parse_type()?;
                    if self.match_token(&[TokenKind::Comma, TokenKind::To]) {
                        let value_type = self.parse_type()?;
                        Ok(Type::CompositeType(CompositeType::DictionaryType(Box::new(key_type), Box::new(value_type))))
                    } else {
                        // assume key is string if not specified? Or error?
                        // For now, error if comma missing
                        Err("Expected ',' or 'to' after key type in Dictionary definition".to_string())
                    }
                    } else {
                        // generic dictionary
                        Ok(Type::CompositeType(CompositeType::DictionaryType(Box::new(Type::BasicType(BasicType::String)), Box::new(Type::BasicType(BasicType::Any)))))
                    }
                }
                _ => {
                    // Assume custom type (Class)
                    Ok(Type::ClassType(name))
                }
            }
        } else {
            Err(format!("Expected type, got: {:?}", self.current_token()))
        }
    }
    
    fn match_token(&mut self, kinds: &[TokenKind]) -> bool {
        self.skip_newlines();
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }
        false
    }
    
    fn parse_run_concurrently_statement(&mut self) -> Result<Node, String> {
        let location = self.previous().location.clone();
        
        self.consume(&TokenKind::Concurrently, "Expected 'concurrently' after 'run'")?;
        
        self.skip_newlines();
        let mut statements = Vec::new();
        let start_indent = self.current_token().location.column;
        
        while !self.is_at_end() {
             let current_indent = self.current_token().location.column;
             
             if self.check(&TokenKind::And) {
                 if let Some(k1) = self.peek_kind_skip_newlines(1) {
                     if matches!(k1, TokenKind::Wait) {
                         break;
                     }
                 }
             }
             
             if current_indent < start_indent && start_indent > 0 {
                 break;
             }
             
             statements.push(self.parse_statement()?);
             self.skip_newlines();
        }
        
        self.consume(&TokenKind::And, "Expected 'and' after run block")?;
        self.consume(&TokenKind::Wait, "Expected 'wait' after 'and'")?;
        self.consume(&TokenKind::For, "Expected 'for' after 'wait'")?;
        self.consume(&TokenKind::All, "Expected 'all' after 'for'")?;
        
        Ok(Node::RunConcurrentlyStatement(RunConcurrentlyStatement {
            location,
            statements,
        }))
    }
    
    fn parse_inspect_statement(&mut self) -> Result<Node, String> {
        // inspect <expr>
        //   case <Variant> do ...
        //   case <Variant> do ...
        
        let location = self.current_token().location.clone();
        self.advance(); // consume "inspect"
        
        let expression = Box::new(self.parse_expression()?);
        
        // Optional newline
        if self.check(&TokenKind::Newline) {
            self.advance();
        }
        
        let mut cases = Vec::new();
        let start_indent = self.current_token().location.column;
        
        loop {
            self.skip_newlines();
            let current_indent = self.current_token().location.column;
            
            if current_indent < start_indent {
                break;
            }
            
            if self.match_token(&[TokenKind::Case]) {
                let variant_name = self.parse_identifier()?;
                
                self.consume(&TokenKind::Do, "Expected 'do' after case variant")?;
                
                // Parse body (block)
                let body_start_indent = self.current_token().location.column;
                let mut statements = Vec::new();
                
                loop {
                    self.skip_newlines();
                    let body_indent = self.current_token().location.column;
                    if body_indent <= start_indent { // dedent back to case level or lower
                        break;
                    }
                    
                    if self.is_at_end() { break; }
                    
                    statements.push(self.parse_statement()?);
                }
                
                let body = if statements.is_empty() {
                     Box::new(Node::LiteralExpression(LiteralExpression {
                        location: self.current_token().location.clone(),
                        value: LiteralValue::Void,
                    }))
                } else if statements.len() == 1 {
                    Box::new(statements.remove(0))
                } else {
                     Box::new(Node::Program(Program {
                        location: statements[0].location().clone(),
                        statements,
                    }))
                };
                
                cases.push(Case {
                    variant_name,
                    body,
                });
            } else {
                break;
            }
        }
        
        Ok(Node::InspectStatement(InspectStatement {
            location,
            expression,
            cases,
        }))
    }

    fn parse_using_statement(&mut self) -> Result<Node, String> {
        let location = self.previous().location.clone();
        
        // "using" already consumed
        
        let resource = self.parse_expression()?;
        
        self.consume(&TokenKind::As, "Expected 'as' after resource expression")?;
        
        let identifier = self.parse_identifier()?;
        
        self.consume(&TokenKind::Do, "Expected 'do' after identifier")?;
        self.skip_newlines();
        
        // Parse body
        let mut statements = Vec::new();
        let start_indent = self.current_token().location.column;
        
        while !self.is_at_end() {
             // Check for "end using" - optional explicit terminator
             if self.check(&TokenKind::Identifier(String::new())) { // Check for "end" identifier
                 if let TokenKind::Identifier(ref s) = self.current_token().kind {
                     if s.to_lowercase() == "end" {
                         if let Some(kind) = self.peek_kind_skip_newlines(1) {
                             if matches!(kind, TokenKind::Using) {
                                 self.advance(); // consume end
                                 self.advance(); // consume using
                                 break;
                             }
                         }
                     }
                 }
             }
             
             // Check indentation
             let current_indent = self.current_token().location.column;
             if current_indent < start_indent && start_indent > 0 {
                 break;
             }
             
             // Parse statement
             match self.parse_statement() {
                 Ok(stmt) => {
                     statements.push(stmt);
                     self.skip_newlines();
                 }
                 Err(e) => return Err(e),
             }
        }
        
        // Wrap statements in body
        let body = if statements.is_empty() {
             Box::new(Node::LiteralExpression(LiteralExpression {
                 location: location.clone(),
                 value: LiteralValue::Void,
             }))
        } else if statements.len() == 1 {
             Box::new(statements.remove(0))
        } else {
             Box::new(Node::Program(Program {
                 location: location.clone(),
                 statements,
             }))
        };
        
        Ok(Node::UsingStatement(UsingStatement {
            location,
            resource: Box::new(resource),
            identifier,
            body,
        }))
    }


    
    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        let current = &self.current_token().kind;
        match (kind, current) {
            (TokenKind::Identifier(k), TokenKind::Identifier(c)) => k == c,
            (TokenKind::TypeIdentifier(k), TokenKind::TypeIdentifier(c)) => k == c,
            (a, b) => std::mem::discriminant(a) == std::mem::discriminant(b),
        }
    }
    
    fn consume(&mut self, kind: &TokenKind, message: &str) -> Result<(), String> {
        self.skip_newlines();
        if self.check(kind) {
            self.advance();
            Ok(())
        } else {
            Err(format!("{}: expected {:?}, got {:?}", message, kind, self.current_token()))
        }
    }
    
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    
    fn is_at_end(&self) -> bool {
        matches!(self.current_token().kind, TokenKind::Eof)
    }
    
    fn current_token(&self) -> &Token {
        &self.tokens[self.current.min(self.tokens.len() - 1)]
    }
    
    fn previous(&self) -> &Token {
        if self.current > 0 {
            &self.tokens[self.current - 1]
        } else {
            &self.tokens[0]
        }
    }
}

impl Parser {
    fn skip_newlines(&mut self) {
        let mut guard = 0usize;
        while !self.is_at_end() && matches!(self.current_token().kind, TokenKind::Newline) {
            self.advance();
            guard += 1;
            if guard > 1000 { break; }
        }
    }
    fn is_noop_marker(s: &str) -> bool {
        matches!(s,
            "output" | "output:")
    }
    fn peek_kind_skip_newlines(&self, offset: usize) -> Option<&TokenKind> {
        let mut idx = self.current + offset;
        while idx < self.tokens.len() {
            let k = &self.tokens[idx].kind;
            if matches!(k, TokenKind::Newline) {
                idx += 1;
                continue;
            }
            return Some(k);
        }
        None
    }
    
    fn peek_kind(&self, offset: usize) -> Option<&TokenKind> {
        let idx = self.current + offset;
        if idx < self.tokens.len() {
            Some(&self.tokens[idx].kind)
        } else {
            None
        }
    }
}

