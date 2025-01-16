// lexer for pure english layman syntax
// tokenizes natural english text into structured tokens

use crate::ast::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub location: Location,
    pub text: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // keywords
    Define,
    Function,
    Variable,
    Constant,
    The,
    Equals,
    Is,
    A,      // for "is a new Type" pattern
    New,    // for object creation
    Call,
    If,
    Then,
    Otherwise,
    Else,
    For,
    Each,
    In,
    Do,
    While,
    Repeat,
    Until,
    Return,
    Import,
    Use,
    Using,
    With,
    As,
    That,
    Takes,
    Struct,
    Dictionary,
    Keys,
    Values,
    And,
    Returns,
    Returning,
    From,
    Or,
    Not,
    To,
    Throw,
    Try,
    Catch,
    Expect,
    Test,
    Describe,
    Of,
    Any,
    Background,
    Start,
    Wait,
    Run,
    Concurrently,
    All,
    Either,
    Inspect,
    Case,
    Maybe,
    Nothing,
    Exists,

    
    // operators
    Plus,
    Minus,
    Times,
    DividedBy,
    Modulo,
    GreaterThan,
    LessThan,
    Than,
    GreaterThanOrEqual,
    LessThanOrEqual,
    NotEquals,
    
    // literals
    Number(f64),
    Text(String),
    Boolean(bool),
    
    // identifiers
    Identifier(String),
    TypeIdentifier(String), // capitalized
    
    // punctuation
    Newline,
    Period,
    Comma,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    
    // literals
    True,
    False,
    
    // eof
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
    current_file: String,
}

impl Lexer {
    pub fn new(input: &str, file: String) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 0,
            current_file: file,
        }
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        while !self.is_at_end() {
            self.skip_whitespace();
            if self.is_at_end() {
                break;
            }
            
            // skip comments
            if self.peek() == '#' {
                self.skip_line();
                continue;
            }
            
            let start_pos = self.position;
            let start_line = self.line;
            let start_col = self.column;
            
            let token = self.next_token()?;
            
            // check if we got EOF from unknown char (position should have advanced due to advance() in next_token)
            // if token is EOF but we're not actually at end, it means unknown char was skipped
            if matches!(token, TokenKind::Eof) {
                if self.is_at_end() {
                    // real EOF
                    break;
                }
                // EOF from unknown char - advance was called so position moved, just skip
                continue;
            }
            
            // only add non-EOF tokens (unless it's the final EOF)
            if !matches!(token, TokenKind::Eof) {
                let text: String = if start_pos < self.position && self.position <= self.input.len() {
                    self.input[start_pos..self.position].iter().collect()
                } else {
                    String::new()
                };
                
                tokens.push(Token {
                    kind: token,
                    location: Location {
                        file: self.current_file.clone(),
                        line: start_line,
                        column: start_col,
                        source: Some(text.clone()),
                    },
                    text,
                });
            } else {
                // reached EOF naturally
                break;
            }
        }
        
        tokens.push(Token {
            kind: TokenKind::Eof,
            location: Location {
                file: self.current_file.clone(),
                line: self.line,
                column: self.column,
                source: None,
            },
            text: String::new(),
        });
        
        Ok(tokens)
    }
    
    fn next_token(&mut self) -> Result<TokenKind, String> {
        let c = self.advance();
        
        match c {
            '\n' => Ok(TokenKind::Newline),
            '.' => Ok(TokenKind::Period),
            ',' => Ok(TokenKind::Comma),
            '+' => Ok(TokenKind::Plus),
            '-' => Ok(TokenKind::Minus),
            '*' => Ok(TokenKind::Times),
            '/' => Ok(TokenKind::DividedBy),
            '=' => Ok(TokenKind::Equals),
            '<' => Ok(TokenKind::LessThan),
            '>' => Ok(TokenKind::GreaterThan),
            '%' => Ok(TokenKind::Modulo),
            '(' => Ok(TokenKind::LeftParen),
            ')' => Ok(TokenKind::RightParen),
            '[' => Ok(TokenKind::LeftBracket),
            ']' => Ok(TokenKind::RightBracket),
            '\'' | '"' | '‘' | '’' | '“' | '”' => {
                // support straight and smart quotes
                let start = self.position - 1;
                let opener = c;
                // determine allowable closing quotes
                let is_closing = |ch: char, opener: char| -> bool {
                    match opener {
                        '\'' | '’' => ch == '\'' || ch == '’',
                        '"' | '”' => ch == '"' || ch == '”',
                        '‘' => ch == '’' || ch == '\'',
                        '“' => ch == '”' || ch == '"',
                        _ => ch == opener,
                    }
                };

                while !self.is_at_end() {
                    let peek = self.peek();
                    if is_closing(peek, opener) {
                        break;
                    }
                    if peek == '\n' {
                        // graceful termination at newline
                        break;
                    }
                    self.advance();
                }

                // if we stopped at a closing quote, consume it
                if !self.is_at_end() && is_closing(self.peek(), opener) {
                    // consume closing quote
                    self.advance();
                    let text: String = self.input[start+1..self.position-1].iter().collect();
                    Ok(TokenKind::Text(text))
                } else {
                    // no closing quote before newline/end: return text up to current position
                    let text: String = self.input[start+1..self.position].iter().collect();
                    Ok(TokenKind::Text(text))
                }
            }
            _ if c.is_alphabetic() || c == '_' => {
                let start = self.position - 1;
                // read a single word (stop at whitespace or punctuation)
                let mut word_end = self.position;
                while !self.is_at_end() {
                    let peek = self.peek();
                    if peek.is_alphanumeric() || peek == '_' {
                        self.advance();
                        word_end = self.position;
                    } else if peek == '\n' || peek == '\t' || peek == ' ' || peek == '.' || peek == ',' {
                        break;
                    } else {
                        break;
                    }
                }
                let word: String = self.input[start..word_end].iter().collect();
                self.match_keyword_or_identifier(&word)
            }
            _ if c.is_ascii_digit() => {
                let start = self.position - 1;
                while self.peek().is_ascii_digit() || self.peek() == '.' {
                    self.advance();
                }
                let num_str: String = self.input[start..self.position].iter().collect();
                match num_str.parse::<f64>() {
                    Ok(n) => Ok(TokenKind::Number(n)),
                    Err(_) => Err(format!("Invalid number: {}", num_str)),
                }
            }
            _ => {
                // skip unknown characters
                // advance() was already called, so we've consumed the char
                // return a special marker that tokenize() will handle
                // by not advancing position, we signal that nothing was tokenized
                if self.is_at_end() {
                    Ok(TokenKind::Eof)
                } else {
                    // return EOF to signal skip (tokenize will check if position advanced)
                    Ok(TokenKind::Eof)
                }
            }
        }
    }
    
    fn match_keyword_or_identifier(&self, word: &str) -> Result<TokenKind, String> {
        // normalize: lowercase for keywords, preserve case for identifiers
        let lower = word.to_lowercase().trim().to_string();
        
        match lower.as_str() {
            "define" => Ok(TokenKind::Define),
            "function" => Ok(TokenKind::Function),
            "variable" => Ok(TokenKind::Variable),
            "constant" => Ok(TokenKind::Constant),
            "the" => Ok(TokenKind::The),
            "equals" => Ok(TokenKind::Equals),
            "equal" => Ok(TokenKind::Equals), // "equal" also maps to Equals for "or equal to"
            "is" => Ok(TokenKind::Is),
            "a" => Ok(TokenKind::A),
            "an" => Ok(TokenKind::A),
            "new" => Ok(TokenKind::New),
            "call" => Ok(TokenKind::Call),
            "if" => Ok(TokenKind::If),
            "then" => Ok(TokenKind::Then),
            "otherwise" => Ok(TokenKind::Otherwise),
            "else" => Ok(TokenKind::Else),
            "for" => Ok(TokenKind::For),
            "each" => Ok(TokenKind::Each),
            "in" => Ok(TokenKind::In),
            "do" => Ok(TokenKind::Do),
            "while" => Ok(TokenKind::While),
            "repeat" => Ok(TokenKind::Repeat),
            "until" => Ok(TokenKind::Until),
            "return" => Ok(TokenKind::Return),
            "import" => Ok(TokenKind::Import),
            "use" => Ok(TokenKind::Use),
            "using" => Ok(TokenKind::Using),
            "with" => Ok(TokenKind::With),
            "as" => Ok(TokenKind::As),
            "that" => Ok(TokenKind::That),
            "takes" => Ok(TokenKind::Takes),
            "struct" => Ok(TokenKind::Struct),
            "dictionary" => Ok(TokenKind::Dictionary),
            "keys" => Ok(TokenKind::Keys),
            "values" => Ok(TokenKind::Values),
            "and" => Ok(TokenKind::And),
            "returns" => Ok(TokenKind::Returns),
            "returning" => Ok(TokenKind::Returning),
            "from" => Ok(TokenKind::From),
            "or" => Ok(TokenKind::Or),
            "not" => Ok(TokenKind::Not),
            "to" => Ok(TokenKind::To),
            "throw" => Ok(TokenKind::Throw),
            "try" => Ok(TokenKind::Try),
            "catch" => Ok(TokenKind::Catch),
            "expect" => Ok(TokenKind::Expect),
            "test" => Ok(TokenKind::Test),
            "describe" => Ok(TokenKind::Describe),
            "of" => Ok(TokenKind::Of),
            "any" => Ok(TokenKind::Any),
            "background" => Ok(TokenKind::Background),
            "asynchronous" => Ok(TokenKind::Background),
            "start" => Ok(TokenKind::Start),
            "wait" => Ok(TokenKind::Wait),
            "run" => Ok(TokenKind::Run),
            "concurrently" => Ok(TokenKind::Concurrently),
            "all" => Ok(TokenKind::All),
            "either" => Ok(TokenKind::Either),
            "inspect" => Ok(TokenKind::Inspect),
            "case" => Ok(TokenKind::Case),
            "maybe" => Ok(TokenKind::Maybe),
            "nothing" => Ok(TokenKind::Nothing),
            "exists" => Ok(TokenKind::Exists),

            "constant" => Ok(TokenKind::Constant),
            "plus" => Ok(TokenKind::Plus),
            "minus" => Ok(TokenKind::Minus),
            "times" => Ok(TokenKind::Times),
            "divided" => {
                // "divided" alone becomes DividedBy, but we'll check for "by" next
                Ok(TokenKind::DividedBy)
            }
            "by" => {
                // if previous token was "divided", this completes "divided by"
                // otherwise, "by" alone is not a valid token - treat as identifier
                // but for now, map to DividedBy for backward compatibility
                Ok(TokenKind::DividedBy)
            }
            "modulo" => Ok(TokenKind::Modulo),
            "greater" => Ok(TokenKind::GreaterThan),
            "than" => Ok(TokenKind::Than),
            "less" => Ok(TokenKind::LessThan),
            "true" => Ok(TokenKind::True),
            "false" => Ok(TokenKind::False),
            "nothing" => Ok(TokenKind::Nothing),
            _ => {
                // check if it starts with capital (type identifier)
                if word.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                    Ok(TokenKind::TypeIdentifier(word.to_string()))
                } else {
                    Ok(TokenKind::Identifier(word.to_string()))
                }
            }
        }
    }
    

    
    fn peek(&self) -> char {
        if self.position >= self.input.len() {
            '\0'
        } else {
            self.input[self.position]
        }
    }
    
    fn advance(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        
        let c = self.input[self.position];
        self.position += 1;
        
        if c == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
        
        c
    }
    
    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            let c = self.peek();
            if c == ' ' || c == '\t' || c == '\r' {
                self.advance();
            } else if c == '#' {
                // skip comment until end of line
                self.skip_line();
            } else if c == '\n' {
                // keep newlines as tokens
                break;
            } else {
                break;
            }
        }
    }
    
    fn skip_line(&mut self) {
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }
        if !self.is_at_end() {
            self.advance(); // consume newline
        }
    }
    
    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
}

