#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Compiled,
    Interpreted,
    Target,
    Let,
    Fn,
    If,
    Else,
    Elif,
    While,
    For,
    Return,
    Import,
    Module,
    Actor,
    Contract,
    Component,
    Flow,
    Deployment,
    Cluster,
    State,
    Signal,
    View,
    Export,
    Async,
    Await,
    Spawn,
    Match,
    Move,
    Borrow,
    Defer,
    Require,
    Send,
    
    // Literals
    Identifier(String),
    Number(f64),
    String(String),
    Boolean(bool),
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    And,
    Or,
    Not,
    Arrow, // =>
    
    // Delimiters
    LBrace,    // {
    RBrace,    // }
    LParen,    // (
    RParen,    // )
    LBracket,  // [
    RBracket,  // ]
    Comma,
    Semicolon,
    Colon,
    Dot,
    Question,  // ?
    Exclamation, // !
    
    // Special
    EOF,
    Newline,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

pub struct Lexer {
    source: Vec<char>,
    current: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            current: 0,
            line: 1,
            col: 1,
        }
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        // Check for header first
        if self.check_header("[Compiled]") {
            tokens.push(Token {
                kind: TokenKind::Compiled,
                line: 1,
                col: 1,
            });
            self.skip_line();
        } else if self.check_header("[Interpreted]") {
            tokens.push(Token {
                kind: TokenKind::Interpreted,
                line: 1,
                col: 1,
            });
            self.skip_line();
        }
        
        // Check for [target ...]
        if self.check_header("[target") {
            tokens.push(Token {
                kind: TokenKind::Target,
                line: self.line,
                col: self.col,
            });
            self.advance_n(7); // skip "[target"
            self.skip_whitespace();
            let target = self.read_identifier();
            if !target.is_empty() {
                tokens.push(Token {
                    kind: TokenKind::Identifier(target),
                    line: self.line,
                    col: self.col,
                });
            }
            self.skip_whitespace();
            if self.peek() == ']' {
                self.advance();
            }
            self.skip_line();
        }
        
        while !self.is_at_end() {
            self.skip_whitespace();
            if self.is_at_end() {
                break;
            }
            
            let start_line = self.line;
            let start_col = self.col;
            
            let token = self.scan_token();
            if let Some(t) = token {
                tokens.push(Token {
                    kind: t,
                    line: start_line,
                    col: start_col,
                });
            }
        }
        
        tokens.push(Token {
            kind: TokenKind::EOF,
            line: self.line,
            col: self.col,
        });
        
        tokens
    }
    
    fn check_header(&mut self, header: &str) -> bool {
        let saved_current = self.current;
        let saved_line = self.line;
        let saved_col = self.col;
        
        for ch in header.chars() {
            if self.peek() != ch {
                self.current = saved_current;
                self.line = saved_line;
                self.col = saved_col;
                return false;
            }
            self.advance();
        }
        
        true
    }
    
    fn skip_line(&mut self) {
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }
        if self.peek() == '\n' {
            self.advance();
        }
    }
    
    fn scan_token(&mut self) -> Option<TokenKind> {
        let ch = self.advance();
        
        match ch {
            '(' => Some(TokenKind::LParen),
            ')' => Some(TokenKind::RParen),
            '{' => Some(TokenKind::LBrace),
            '}' => Some(TokenKind::RBrace),
            '[' => Some(TokenKind::LBracket),
            ']' => Some(TokenKind::RBracket),
            ',' => Some(TokenKind::Comma),
            ';' => Some(TokenKind::Semicolon),
            ':' => Some(TokenKind::Colon),
            '.' => Some(TokenKind::Dot),
            '?' => Some(TokenKind::Question),
            '!' => Some(TokenKind::Exclamation),
            '+' => Some(TokenKind::Plus),
            '-' => {
                if self.peek() == '>' {
                    self.advance();
                    Some(TokenKind::Arrow)
                } else {
                    Some(TokenKind::Minus)
                }
            },
            '*' => Some(TokenKind::Star),
            '/' => {
                if self.peek() == '/' {
                    // Line comment
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                    None
                } else if self.peek() == '#' {
                    // Also support # comments
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                    None
                } else {
                    Some(TokenKind::Slash)
                }
            },
            '%' => Some(TokenKind::Percent),
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    Some(TokenKind::EqualEqual)
                } else {
                    Some(TokenKind::Equal)
                }
            },
            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    Some(TokenKind::NotEqual)
                } else {
                    Some(TokenKind::Not)
                }
            },
            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    Some(TokenKind::LessEqual)
                } else {
                    Some(TokenKind::LessThan)
                }
            },
            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    Some(TokenKind::GreaterEqual)
                } else {
                    Some(TokenKind::GreaterThan)
                }
            },
            '&' => {
                if self.peek() == '&' {
                    self.advance();
                    Some(TokenKind::And)
                } else {
                    Some(TokenKind::Borrow)
                }
            },
            '|' => {
                if self.peek() == '|' {
                    self.advance();
                    Some(TokenKind::Or)
                } else {
                    None // Invalid
                }
            },
            '"' => Some(self.string()),
            '\'' => Some(self.string()),
            '0'..='9' => {
                self.current -= 1;
                self.col -= 1;
                Some(self.number())
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                self.current -= 1;
                self.col -= 1;
                Some(self.identifier_or_keyword())
            },
            '\n' => {
                self.line += 1;
                self.col = 1;
                Some(TokenKind::Newline)
            },
            _ => None, // Skip unknown characters
        }
    }
    
    fn string(&mut self) -> TokenKind {
        let quote = self.source[self.current - 1];
        let mut value = String::new();
        
        while !self.is_at_end() && self.peek() != quote {
            if self.peek() == '\\' {
                self.advance();
                match self.peek() {
                    'n' => {
                        value.push('\n');
                        self.advance();
                    },
                    't' => {
                        value.push('\t');
                        self.advance();
                    },
                    '\\' => {
                        value.push('\\');
                        self.advance();
                    },
                    '"' => {
                        value.push('"');
                        self.advance();
                    },
                    _ => {
                        value.push('\\');
                    },
                }
            } else {
                value.push(self.advance());
            }
        }
        
        if !self.is_at_end() {
            self.advance(); // Consume closing quote
        }
        
        TokenKind::String(value)
    }
    
    fn number(&mut self) -> TokenKind {
        let mut value = String::new();
        
        while self.peek().is_ascii_digit() {
            value.push(self.advance());
        }
        
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            value.push(self.advance());
            while self.peek().is_ascii_digit() {
                value.push(self.advance());
            }
        }
        
        TokenKind::Number(value.parse().unwrap_or(0.0))
    }
    
    fn identifier_or_keyword(&mut self) -> TokenKind {
        let ident = self.read_identifier();
        
        match ident.as_str() {
            "let" => TokenKind::Let,
            "fn" => TokenKind::Fn,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "elif" => TokenKind::Elif,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "in" => TokenKind::Identifier("in".to_string()), // Special: keep as identifier for for loops
            "return" => TokenKind::Return,
            "import" => TokenKind::Import,
            "module" => TokenKind::Module,
            "actor" => TokenKind::Actor,
            "contract" => TokenKind::Contract,
            "component" => TokenKind::Component,
            "flow" => TokenKind::Flow,
            "deployment" => TokenKind::Deployment,
            "cluster" => TokenKind::Cluster,
            "state" => TokenKind::State,
            "signal" => TokenKind::Signal,
            "view" => TokenKind::View,
            "export" => TokenKind::Export,
            "async" => TokenKind::Async,
            "await" => TokenKind::Await,
            "spawn" => TokenKind::Spawn,
            "match" => TokenKind::Match,
            "move" => TokenKind::Move,
            "borrow" => TokenKind::Borrow,
            "defer" => TokenKind::Defer,
            "require" => TokenKind::Require,
            "send" => TokenKind::Send,
            "self" => TokenKind::Identifier("self".to_string()), // Special: keep as identifier for actors
            "true" => TokenKind::Boolean(true),
            "false" => TokenKind::Boolean(false),
            "const" => TokenKind::Identifier(ident), // Keep as identifier for now
            _ => TokenKind::Identifier(ident),
        }
    }
    
    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            ident.push(self.advance());
        }
        ident
    }
    
    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.peek().is_whitespace() && self.peek() != '\n' {
            self.advance();
        }
    }
    
    fn advance(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let ch = self.source[self.current];
        self.current += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        ch
    }
    
    fn advance_n(&mut self, n: usize) {
        for _ in 0..n {
            self.advance();
        }
    }
    
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }
    
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }
    
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

