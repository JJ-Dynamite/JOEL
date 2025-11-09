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
    
    pub fn parse(&mut self) -> Program {
        let mode = if !self.tokens.is_empty() {
            match self.tokens[0].kind {
                TokenKind::Compiled => ExecutionMode::Compiled,
                TokenKind::Interpreted => ExecutionMode::Interpreted,
                _ => ExecutionMode::Unknown,
            }
        } else {
            ExecutionMode::Unknown
        };
        
        // Skip header tokens
        if matches!(self.peek().kind, TokenKind::Compiled | TokenKind::Interpreted) {
            self.advance();
        }
        
        let mut target = None;
        if matches!(self.peek().kind, TokenKind::Target) {
            self.advance();
            if let TokenKind::Identifier(t) = &self.peek().kind {
                target = Some(t.clone());
                self.advance();
            }
        }
        
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            } else {
                self.advance(); // Skip error token
            }
        }
        
        Program {
            mode,
            target,
            statements,
        }
    }
    
    fn declaration(&mut self) -> Option<Stmt> {
        match self.peek().kind {
            TokenKind::Let => {
                self.advance();
                self.let_declaration()
            },
            TokenKind::Fn => {
                self.advance();
                self.function()
            },
            TokenKind::Module => {
                self.advance();
                self.module()
            },
            TokenKind::Import => {
                self.advance();
                self.import()
            },
            TokenKind::Actor => {
                self.advance();
                self.actor()
            },
            TokenKind::Contract => {
                self.advance();
                self.contract()
            },
            TokenKind::Component => {
                self.advance();
                self.component()
            },
            TokenKind::Flow => {
                self.advance();
                self.flow()
            },
            TokenKind::Deployment => {
                self.advance();
                self.deployment()
            },
            TokenKind::Cluster => {
                self.advance();
                self.cluster()
            },
            _ => self.statement(),
        }
    }
    
    fn let_declaration(&mut self) -> Option<Stmt> {
        let name = if let TokenKind::Identifier(n) = &self.peek().kind {
            let n = n.clone();
            self.advance();
            n
        } else {
            return None;
        };
        
        let type_annot = if self.peek().kind == TokenKind::Colon {
            self.advance();
            if let TokenKind::Identifier(t) = &self.peek().kind {
                let t = t.clone();
                self.advance();
                Some(t)
            } else {
                None
            }
        } else {
            None
        };
        
        if self.peek().kind != TokenKind::Equal {
            return None;
        }
        self.advance();
        
        let value = self.expression()?;
        
        Some(Stmt::Let {
            name,
            type_annot,
            value,
        })
    }
    
    fn function(&mut self) -> Option<Stmt> {
        let name = if let TokenKind::Identifier(n) = &self.peek().kind {
            let n = n.clone();
            self.advance();
            n
        } else {
            return None;
        };
        
        if self.peek().kind != TokenKind::LParen {
            return None;
        }
        self.advance();
        
        let mut params = Vec::new();
        if self.peek().kind != TokenKind::RParen {
            loop {
                let param_name = if let TokenKind::Identifier(n) = &self.peek().kind {
                    let n = n.clone();
                    self.advance();
                    n
                } else {
                    break;
                };
                
                let param_type = if self.peek().kind == TokenKind::Colon {
                    self.advance();
                    if let TokenKind::Identifier(t) = &self.peek().kind {
                        let t = t.clone();
                        self.advance();
                        Some(t)
                    } else {
                        None
                    }
                } else {
                    None
                };
                
                params.push((param_name, param_type));
                
                if self.peek().kind != TokenKind::Comma {
                    break;
                }
                self.advance();
            }
        }
        
        if self.peek().kind != TokenKind::RParen {
            return None;
        }
        self.advance();
        
        let return_type = if self.peek().kind == TokenKind::Arrow {
            self.advance();
            if let TokenKind::Identifier(t) = &self.peek().kind {
                let t = t.clone();
                self.advance();
                Some(t)
            } else {
                None
            }
        } else {
            None
        };
        
        let body = self.block()?;
        
        Some(Stmt::Fn {
            name,
            params,
            return_type,
            body,
        })
    }
    
    fn module(&mut self) -> Option<Stmt> {
        let name = if let TokenKind::Identifier(n) = &self.peek().kind {
            let n = n.clone();
            self.advance();
            n
        } else {
            return None;
        };
        
        Some(Stmt::Module { name })
    }
    
    fn import(&mut self) -> Option<Stmt> {
        let module = if let TokenKind::Identifier(n) = &self.peek().kind {
            let n = n.clone();
            self.advance();
            n
        } else if let TokenKind::String(s) = &self.peek().kind {
            let s = s.clone();
            self.advance();
            s
        } else {
            return None;
        };
        
        let alias = if self.peek().kind == TokenKind::Identifier {
            if let TokenKind::Identifier(a) = &self.peek().kind {
                let a = a.clone();
                self.advance();
                Some(a)
            } else {
                None
            }
        } else {
            None
        };
        
        Some(Stmt::Import { module, alias })
    }
    
    fn actor(&mut self) -> Option<Stmt> {
        let name = if let TokenKind::Identifier(n) = &self.peek().kind {
            let n = n.clone();
            self.advance();
            n
        } else {
            return None;
        };
        
        if self.peek().kind != TokenKind::LBrace {
            return None;
        }
        self.advance();
        
        let mut fields = Vec::new();
        let mut methods = Vec::new();
        
        while self.peek().kind != TokenKind::RBrace && !self.is_at_end() {
            if self.peek().kind == TokenKind::State {
                self.advance();
                if self.peek().kind == TokenKind::Let {
                    self.advance();
                    if let Some(Stmt::Let { name, type_annot, value }) = self.let_declaration() {
                        fields.push((name, type_annot, value));
                    }
                }
            } else if self.peek().kind == TokenKind::Fn {
                self.advance();
                if let Some(method) = self.function() {
                    methods.push(method);
                }
            } else {
                self.advance();
            }
        }
        
        if self.peek().kind == TokenKind::RBrace {
            self.advance();
        }
        
        Some(Stmt::Actor {
            name,
            fields,
            methods,
        })
    }
    
    fn contract(&mut self) -> Option<Stmt> {
        let name = if let TokenKind::Identifier(n) = &self.peek().kind {
            let n = n.clone();
            self.advance();
            n
        } else {
            return None;
        };
        
        if self.peek().kind != TokenKind::LBrace {
            return None;
        }
        self.advance();
        
        let mut fields = Vec::new();
        let mut methods = Vec::new();
        
        while self.peek().kind != TokenKind::RBrace && !self.is_at_end() {
            if self.peek().kind == TokenKind::State {
                self.advance();
                if self.peek().kind == TokenKind::Let {
                    self.advance();
                    if let Some(Stmt::Let { name, type_annot, value }) = self.let_declaration() {
                        fields.push((name, type_annot, value));
                    }
                }
            } else if self.peek().kind == TokenKind::Fn {
                self.advance();
                if let Some(method) = self.function() {
                    methods.push(method);
                }
            } else {
                self.advance();
            }
        }
        
        if self.peek().kind == TokenKind::RBrace {
            self.advance();
        }
        
        Some(Stmt::Contract {
            name,
            fields,
            methods,
        })
    }
    
    fn component(&mut self) -> Option<Stmt> {
        let name = if let TokenKind::Identifier(n) = &self.peek().kind {
            let n = n.clone();
            self.advance();
            n
        } else {
            return None;
        };
        
        if self.peek().kind != TokenKind::LParen {
            return None;
        }
        self.advance();
        
        if self.peek().kind != TokenKind::RParen {
            return None;
        }
        self.advance();
        
        if self.peek().kind != TokenKind::LBrace {
            return None;
        }
        self.advance();
        
        let mut body = Vec::new();
        while self.peek().kind != TokenKind::RBrace && !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                body.push(stmt);
            } else {
                self.advance();
            }
        }
        
        if self.peek().kind == TokenKind::RBrace {
            self.advance();
        }
        
        Some(Stmt::Component { name, body })
    }
    
    fn flow(&mut self) -> Option<Stmt> {
        let name = if let TokenKind::Identifier(n) = &self.peek().kind {
            let n = n.clone();
            self.advance();
            n
        } else {
            return None;
        };
        
        if self.peek().kind != TokenKind::LBrace {
            return None;
        }
        self.advance();
        
        let mut nodes = Vec::new();
        while self.peek().kind != TokenKind::RBrace && !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                nodes.push(stmt);
            } else {
                self.advance();
            }
        }
        
        if self.peek().kind == TokenKind::RBrace {
            self.advance();
        }
        
        Some(Stmt::Flow { name, nodes })
    }
    
    fn deployment(&mut self) -> Option<Stmt> {
        if let TokenKind::String(name) = &self.peek().kind {
            let name = name.clone();
            self.advance();
            
            if self.peek().kind != TokenKind::LBrace {
                return None;
            }
            self.advance();
            
            let mut config = Vec::new();
            while self.peek().kind != TokenKind::RBrace && !self.is_at_end() {
                if let Some(stmt) = self.declaration() {
                    config.push(stmt);
                } else {
                    self.advance();
                }
            }
            
            if self.peek().kind == TokenKind::RBrace {
                self.advance();
            }
            
            Some(Stmt::Deployment { name, config })
        } else {
            None
        }
    }
    
    fn cluster(&mut self) -> Option<Stmt> {
        if let TokenKind::String(name) = &self.peek().kind {
            let name = name.clone();
            self.advance();
            
            if self.peek().kind != TokenKind::LBrace {
                return None;
            }
            self.advance();
            
            let mut config = Vec::new();
            while self.peek().kind != TokenKind::RBrace && !self.is_at_end() {
                if let Some(stmt) = self.declaration() {
                    config.push(stmt);
                } else {
                    self.advance();
                }
            }
            
            if self.peek().kind == TokenKind::RBrace {
                self.advance();
            }
            
            Some(Stmt::Cluster { name, config })
        } else {
            None
        }
    }
    
    fn statement(&mut self) -> Option<Stmt> {
        match self.peek().kind {
            TokenKind::Print => {
                self.advance();
                if self.peek().kind == TokenKind::LParen {
                    self.advance();
                    let expr = self.expression()?;
                    if self.peek().kind == TokenKind::RParen {
                        self.advance();
                    }
                    Some(Stmt::Print(expr))
                } else {
                    let expr = self.expression()?;
                    Some(Stmt::Print(expr))
                }
            },
            TokenKind::Return => {
                self.advance();
                let value = if self.peek().kind != TokenKind::Semicolon && self.peek().kind != TokenKind::Newline {
                    Some(self.expression()?)
                } else {
                    None
                };
                Some(Stmt::Return(value))
            },
            TokenKind::If => {
                self.advance();
                self.if_statement()
            },
            TokenKind::While => {
                self.advance();
                self.while_statement()
            },
            TokenKind::For => {
                self.advance();
                self.for_statement()
            },
            TokenKind::LBrace => {
                self.advance();
                Some(Stmt::Block(self.block()?))
            },
            _ => {
                let expr = self.expression()?;
                Some(Stmt::Expr(expr))
            },
        }
    }
    
    fn if_statement(&mut self) -> Option<Stmt> {
        let condition = self.expression()?;
        let then_branch = self.block()?;
        
        let else_branch = if self.peek().kind == TokenKind::Else {
            self.advance();
            Some(self.block()?)
        } else if self.peek().kind == TokenKind::Elif {
            self.advance();
            // For simplicity, treat elif as nested if
            Some(vec![self.if_statement()?])
        } else {
            None
        };
        
        Some(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }
    
    fn while_statement(&mut self) -> Option<Stmt> {
        let condition = self.expression()?;
        let body = self.block()?;
        Some(Stmt::While { condition, body })
    }
    
    fn for_statement(&mut self) -> Option<Stmt> {
        let var = if let TokenKind::Identifier(n) = &self.peek().kind {
            let n = n.clone();
            self.advance();
            n
        } else {
            return None;
        };
        
        // Check for "in" keyword
        if let TokenKind::Identifier(ref s) = self.peek().kind {
            if s == "in" {
                self.advance();
            } else {
                return None;
            }
        } else {
            return None;
        }
        
        let iterable = self.expression()?;
        let body = self.block()?;
        
        Some(Stmt::For {
            var,
            iterable,
            body,
        })
    }
    
    fn block(&mut self) -> Option<Vec<Stmt>> {
        let mut statements = Vec::new();
        
        if self.peek().kind == TokenKind::LBrace {
            self.advance();
            while self.peek().kind != TokenKind::RBrace && !self.is_at_end() {
                if let Some(stmt) = self.declaration() {
                    statements.push(stmt);
                } else {
                    self.advance();
                }
            }
            if self.peek().kind == TokenKind::RBrace {
                self.advance();
            }
        } else {
            // Single statement
            if let Some(stmt) = self.statement() {
                statements.push(stmt);
            }
        }
        
        Some(statements)
    }
    
    fn expression(&mut self) -> Option<Expr> {
        self.assignment()
    }
    
    fn assignment(&mut self) -> Option<Expr> {
        let expr = self.or()?;
        // Assignment parsing can be added here later
        Some(expr)
    }
    
    fn or(&mut self) -> Option<Expr> {
        let mut expr = self.and()?;
        
        while self.peek().kind == TokenKind::Or {
            let op = BinaryOp::Or;
            self.advance();
            let right = self.and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        
        Some(expr)
    }
    
    fn and(&mut self) -> Option<Expr> {
        let mut expr = self.equality()?;
        
        while self.peek().kind == TokenKind::And {
            let op = BinaryOp::And;
            self.advance();
            let right = self.equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        
        Some(expr)
    }
    
    fn equality(&mut self) -> Option<Expr> {
        let mut expr = self.comparison()?;
        
        while matches!(self.peek().kind, TokenKind::EqualEqual | TokenKind::NotEqual) {
            let op = match self.peek().kind {
                TokenKind::EqualEqual => BinaryOp::Equal,
                TokenKind::NotEqual => BinaryOp::NotEqual,
                _ => break,
            };
            self.advance();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        
        Some(expr)
    }
    
    fn comparison(&mut self) -> Option<Expr> {
        let mut expr = self.term()?;
        
        while matches!(
            self.peek().kind,
            TokenKind::GreaterThan
                | TokenKind::GreaterEqual
                | TokenKind::LessThan
                | TokenKind::LessEqual
        ) {
            let op = match self.peek().kind {
                TokenKind::GreaterThan => BinaryOp::GreaterThan,
                TokenKind::GreaterEqual => BinaryOp::GreaterEqual,
                TokenKind::LessThan => BinaryOp::LessThan,
                TokenKind::LessEqual => BinaryOp::LessEqual,
                _ => break,
            };
            self.advance();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        
        Some(expr)
    }
    
    fn term(&mut self) -> Option<Expr> {
        let mut expr = self.factor()?;
        
        while matches!(self.peek().kind, TokenKind::Plus | TokenKind::Minus) {
            let op = match self.peek().kind {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Subtract,
                _ => break,
            };
            self.advance();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        
        Some(expr)
    }
    
    fn factor(&mut self) -> Option<Expr> {
        let mut expr = self.unary()?;
        
        while matches!(self.peek().kind, TokenKind::Star | TokenKind::Slash | TokenKind::Percent) {
            let op = match self.peek().kind {
                TokenKind::Star => BinaryOp::Multiply,
                TokenKind::Slash => BinaryOp::Divide,
                TokenKind::Percent => BinaryOp::Modulo,
                _ => break,
            };
            self.advance();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        
        Some(expr)
    }
    
    fn unary(&mut self) -> Option<Expr> {
        if matches!(self.peek().kind, TokenKind::Not | TokenKind::Minus) {
            let op = match self.peek().kind {
                TokenKind::Not => UnaryOp::Not,
                TokenKind::Minus => UnaryOp::Negate,
                _ => return self.call(),
            };
            self.advance();
            let expr = self.unary()?;
            Some(Expr::Unary {
                op,
                expr: Box::new(expr),
            })
        } else {
            self.call()
        }
    }
    
    fn call(&mut self) -> Option<Expr> {
        let mut expr = self.primary()?;
        
        loop {
            if self.peek().kind == TokenKind::LParen {
                self.advance();
                let mut args = Vec::new();
                if self.peek().kind != TokenKind::RParen {
                    loop {
                        args.push(self.expression()?);
                        if self.peek().kind != TokenKind::Comma {
                            break;
                        }
                        self.advance();
                    }
                }
                if self.peek().kind == TokenKind::RParen {
                    self.advance();
                }
                
                if let Expr::Identifier(callee) = expr {
                    expr = Expr::Call {
                        callee,
                        args,
                    };
                }
            } else if self.peek().kind == TokenKind::Dot {
                self.advance();
                if let TokenKind::Identifier(member) = &self.peek().kind {
                    let member = member.clone();
                    self.advance();
                    expr = Expr::Member {
                        object: Box::new(expr),
                        member,
                    };
                } else {
                    break;
                }
            } else if self.peek().kind == TokenKind::LBracket {
                self.advance();
                let index = self.expression()?;
                if self.peek().kind == TokenKind::RBracket {
                    self.advance();
                }
                expr = Expr::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else {
                break;
            }
        }
        
        Some(expr)
    }
    
    fn primary(&mut self) -> Option<Expr> {
        match self.peek().kind {
            TokenKind::Number(n) => {
                self.advance();
                Some(Expr::Number(n))
            },
            TokenKind::String(s) => {
                self.advance();
                Some(Expr::String(s))
            },
            TokenKind::Boolean(b) => {
                self.advance();
                Some(Expr::Boolean(b))
            },
            TokenKind::Identifier(ref name) => {
                let name = name.clone();
                self.advance();
                Some(Expr::Identifier(name))
            },
            TokenKind::LParen => {
                self.advance();
                let expr = self.expression()?;
                if self.peek().kind == TokenKind::RParen {
                    self.advance();
                }
                Some(expr)
            },
            TokenKind::LBracket => {
                self.advance();
                let mut elements = Vec::new();
                if self.peek().kind != TokenKind::RBracket {
                    loop {
                        elements.push(self.expression()?);
                        if self.peek().kind != TokenKind::Comma {
                            break;
                        }
                        self.advance();
                    }
                }
                if self.peek().kind == TokenKind::RBracket {
                    self.advance();
                }
                Some(Expr::List(elements))
            },
            _ => None,
        }
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }
    
    fn is_at_end(&self) -> bool {
        matches!(self.tokens[self.current].kind, TokenKind::EOF)
    }
}

