// Ownership and Borrow Checker for JOEL
// Inspired by Rust's ownership system

use crate::ast::*;
use crate::diagnostics::{DiagnosticReporter, SourceLocation};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Ownership {
    Owned,      // Value is owned
    Borrowed,   // Value is borrowed (immutable reference)
    Mutable,    // Value is mutably borrowed
    Moved,      // Value has been moved
}

#[derive(Debug, Clone)]
pub struct VariableState {
    pub ownership: Ownership,
    pub scope_level: usize,
}

pub struct BorrowChecker {
    reporter: DiagnosticReporter,
    variables: HashMap<String, VariableState>,
    scope_level: usize,
    in_function: bool,
}

impl BorrowChecker {
    pub fn new(source: &str) -> Self {
        Self {
            reporter: DiagnosticReporter::new(source),
            variables: HashMap::new(),
            scope_level: 0,
            in_function: false,
        }
    }
    
    pub fn check(&mut self, program: &Program) -> bool {
        // Only check ownership in Compiled mode
        if program.mode != ExecutionMode::Compiled {
            return true;
        }
        
        for stmt in &program.statements {
            self.check_statement(stmt);
        }
        
        !self.reporter.has_errors()
    }
    
    fn check_statement(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, value, .. } => {
                self.check_expression(value);
                // New variable is owned
                self.variables.insert(name.clone(), VariableState {
                    ownership: Ownership::Owned,
                    scope_level: self.scope_level,
                });
            },
            Stmt::Const { name, value, .. } => {
                self.check_expression(value);
                // Const is owned and immutable
                self.variables.insert(name.clone(), VariableState {
                    ownership: Ownership::Owned,
                    scope_level: self.scope_level,
                });
            },
            Stmt::Expr(expr) => {
                self.check_expression(expr);
            },
            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    self.check_expression(expr);
                }
            },
            Stmt::If { condition, then_branch, else_branch } => {
                self.check_expression(condition);
                self.begin_scope();
                for stmt in then_branch {
                    self.check_statement(stmt);
                }
                self.end_scope();
                
                if let Some(else_branch) = else_branch {
                    self.begin_scope();
                    for stmt in else_branch {
                        self.check_statement(stmt);
                    }
                    self.end_scope();
                }
            },
            Stmt::While { condition, body } => {
                self.check_expression(condition);
                self.begin_scope();
                for stmt in body {
                    self.check_statement(stmt);
                }
                self.end_scope();
            },
            Stmt::For { var, iterable, body } => {
                self.check_expression(iterable);
                self.begin_scope();
                // Loop variable is owned in the loop scope
                self.variables.insert(var.clone(), VariableState {
                    ownership: Ownership::Owned,
                    scope_level: self.scope_level,
                });
                for stmt in body {
                    self.check_statement(stmt);
                }
                self.end_scope();
            },
            Stmt::Block(statements) => {
                self.begin_scope();
                for stmt in statements {
                    self.check_statement(stmt);
                }
                self.end_scope();
            },
            Stmt::Fn { name: _, params, body, .. } => {
                self.in_function = true;
                self.begin_scope();
                
                // Parameters are owned
                for (param_name, _) in params {
                    self.variables.insert(param_name.clone(), VariableState {
                        ownership: Ownership::Owned,
                        scope_level: self.scope_level,
                    });
                }
                
                for stmt in body {
                    self.check_statement(stmt);
                }
                
                self.end_scope();
                self.in_function = false;
            },
            _ => {}
        }
    }
    
    fn check_expression(&mut self, expr: &Expr) {
        match expr {
            Expr::Identifier(name) => {
                if let Some(var_state) = self.variables.get(name) {
                    match var_state.ownership {
                        Ownership::Moved => {
                            self.reporter.error(
                                format!("Use of moved value: {}", name),
                                None,
                            );
                        },
                        _ => {
                            // Reading is fine for owned/borrowed values
                        }
                    }
                }
            },
            Expr::Binary { left, op: _, right } => {
                self.check_expression(left);
                self.check_expression(right);
            },
            Expr::Unary { expr, .. } => {
                self.check_expression(expr);
            },
            Expr::Call { callee: _, args } => {
                for arg in args {
                    self.check_expression(arg);
                    // Function calls may move values (simplified for now)
                    if let Expr::Identifier(name) = arg {
                        if let Some(var_state) = self.variables.get_mut(name) {
                            if var_state.ownership == Ownership::Owned {
                                var_state.ownership = Ownership::Moved;
                            }
                        }
                    }
                }
            },
            Expr::Member { object, .. } => {
                self.check_expression(object);
            },
            Expr::Index { object, index } => {
                self.check_expression(object);
                self.check_expression(index);
            },
            Expr::List(elements) => {
                for elem in elements {
                    self.check_expression(elem);
                }
            },
            Expr::Map(pairs) => {
                for (key, value) in pairs {
                    self.check_expression(key);
                    self.check_expression(value);
                }
            },
            _ => {}
        }
    }
    
    fn begin_scope(&mut self) {
        self.scope_level += 1;
    }
    
    fn end_scope(&mut self) {
        // Remove variables from this scope
        self.variables.retain(|_, state| state.scope_level < self.scope_level);
        self.scope_level -= 1;
    }
    
    pub fn print_diagnostics(&self) {
        self.reporter.print_all();
    }
    
    pub fn has_errors(&self) -> bool {
        self.reporter.has_errors()
    }
}

