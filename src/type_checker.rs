use crate::ast::*;
use crate::types::Type;
use crate::diagnostics::{DiagnosticReporter, SourceLocation};
use crate::lexer::Token;
use std::collections::HashMap;

pub struct TypeChecker {
    reporter: DiagnosticReporter,
    scopes: Vec<HashMap<String, Type>>,
    functions: HashMap<String, (Vec<(String, Type)>, Type)>, // name -> (params, return_type)
    current_function_return: Option<Type>,
}

impl TypeChecker {
    pub fn new(source: &str) -> Self {
        Self {
            reporter: DiagnosticReporter::new(source),
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
            current_function_return: None,
        }
    }
    
    pub fn check(&mut self, program: &Program) -> bool {
        // Only type check in Compiled mode
        if program.mode != ExecutionMode::Compiled {
            return true; // Skip type checking for interpreted mode
        }
        
        // First pass: collect function signatures
        for stmt in &program.statements {
            self.collect_declarations(stmt);
        }
        
        // Second pass: type check all statements
        for stmt in &program.statements {
            self.check_statement(stmt);
        }
        
        !self.reporter.has_errors()
    }
    
    fn collect_declarations(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Fn { name, params, return_type, .. } => {
                let param_types: Vec<(String, Type)> = params.iter()
                    .map(|(name, type_str)| {
                        let ty = type_str.as_ref()
                            .and_then(|s| Type::from_string(s))
                            .unwrap_or(Type::Unknown);
                        (name.clone(), ty)
                    })
                    .collect();
                
                let ret_type = return_type.as_ref()
                    .and_then(|s| Type::from_string(s))
                    .unwrap_or(Type::None);
                
                self.functions.insert(name.clone(), (param_types, ret_type));
            },
            Stmt::Let { name, type_annot, .. } => {
                if let Some(type_str) = type_annot {
                    if let Some(ty) = Type::from_string(type_str) {
                        self.define_variable(name.clone(), ty);
                    }
                }
            },
            _ => {}
        }
    }
    
    fn check_statement(&mut self, stmt: &Stmt) -> Type {
        match stmt {
            Stmt::Let { name, type_annot, value } => {
                let value_type = self.check_expression(value);
                
                if let Some(type_str) = type_annot {
                    if let Some(expected_type) = Type::from_string(type_str) {
                        if !value_type.can_coerce_to(&expected_type) && value_type != Type::Unknown {
                            self.reporter.error(
                                format!("Type mismatch: expected {}, got {}", 
                                    expected_type.to_string(), value_type.to_string()),
                                None,
                            );
                        }
                        self.define_variable(name.clone(), expected_type.clone());
                        expected_type
                    } else {
                        self.reporter.error(
                            format!("Unknown type: {}", type_str),
                            None,
                        );
                        self.define_variable(name.clone(), value_type.clone());
                        value_type
                    }
                } else {
                    // Type inference
                    if value_type == Type::Unknown {
                        self.reporter.warning(
                            format!("Cannot infer type for variable '{}', defaulting to Any", name),
                            None,
                        );
                        self.define_variable(name.clone(), Type::Any);
                        Type::Any
                    } else {
                        self.define_variable(name.clone(), value_type.clone());
                        value_type
                    }
                }
            },
            Stmt::Const { name, type_annot, value } => {
                let value_type = self.check_expression(value);
                
                if let Some(type_str) = type_annot {
                    if let Some(expected_type) = Type::from_string(type_str) {
                        if !value_type.can_coerce_to(&expected_type) && value_type != Type::Unknown {
                            self.reporter.error(
                                format!("Type mismatch: expected {}, got {}", 
                                    expected_type.to_string(), value_type.to_string()),
                                None,
                            );
                        }
                        self.define_variable(name.clone(), expected_type.clone());
                        expected_type
                    } else {
                        self.reporter.error(
                            format!("Unknown type: {}", type_str),
                            None,
                        );
                        self.define_variable(name.clone(), value_type.clone());
                        value_type
                    }
                } else {
                    if value_type == Type::Unknown {
                        self.reporter.error(
                            format!("Const '{}' requires explicit type annotation", name),
                            None,
                        );
                        self.define_variable(name.clone(), Type::Any);
                        Type::Any
                    } else {
                        self.define_variable(name.clone(), value_type.clone());
                        value_type
                    }
                }
            },
            Stmt::Expr(expr) => {
                self.check_expression(expr)
            },
            Stmt::Print(expr) => {
                self.check_expression(expr);
                Type::None
            },
            Stmt::Return(expr) => {
                let return_type = if let Some(expr) = expr {
                    self.check_expression(expr)
                } else {
                    Type::None
                };
                
                if let Some(expected_return) = &self.current_function_return {
                    if !return_type.can_coerce_to(expected_return) && return_type != Type::Unknown {
                        self.reporter.error(
                            format!("Return type mismatch: expected {}, got {}", 
                                expected_return.to_string(), return_type.to_string()),
                            None,
                        );
                    }
                }
                return_type
            },
            Stmt::If { condition, then_branch, else_branch } => {
                let cond_type = self.check_expression(condition);
                if cond_type != Type::Bool && cond_type != Type::Unknown && cond_type != Type::Any {
                    self.reporter.error(
                        format!("If condition must be bool, got {}", cond_type.to_string()),
                        None,
                    );
                }
                
                self.begin_scope();
                let then_type = self.check_block(then_branch);
                self.end_scope();
                
                if let Some(else_branch) = else_branch {
                    self.begin_scope();
                    let else_type = self.check_block(else_branch);
                    self.end_scope();
                    
                    // Both branches should return compatible types
                    if then_type != else_type && then_type != Type::Unknown && else_type != Type::Unknown {
                        // This is a warning, not an error
                        self.reporter.warning(
                            format!("If/else branches return different types: {} and {}", 
                                then_type.to_string(), else_type.to_string()),
                            None,
                        );
                    }
                }
                
                Type::None
            },
            Stmt::While { condition, body } => {
                let cond_type = self.check_expression(condition);
                if cond_type != Type::Bool && cond_type != Type::Unknown && cond_type != Type::Any {
                    self.reporter.error(
                        format!("While condition must be bool, got {}", cond_type.to_string()),
                        None,
                    );
                }
                
                self.begin_scope();
                self.check_block(body);
                self.end_scope();
                
                Type::None
            },
            Stmt::For { var, iterable, body } => {
                let iter_type = self.check_expression(iterable);
                match &iter_type {
                    Type::List(_) | Type::Any | Type::Unknown => {
                        // OK
                    },
                    _ => {
                        self.reporter.error(
                            format!("For loop iterable must be a list, got {}", iter_type.to_string()),
                            None,
                        );
                    }
                }
                
                self.begin_scope();
                // Infer loop variable type from list element type
                if let Type::List(elem_type) = &iter_type {
                    self.define_variable(var.clone(), *elem_type.clone());
                } else {
                    self.define_variable(var.clone(), Type::Any);
                }
                self.check_block(body);
                self.end_scope();
                
                Type::None
            },
            Stmt::Block(statements) => {
                self.begin_scope();
                let result = self.check_block(statements);
                self.end_scope();
                result
            },
            Stmt::Fn { name, params, return_type, body } => {
                let param_types: Vec<(String, Type)> = params.iter()
                    .map(|(name, type_str)| {
                        let ty = type_str.as_ref()
                            .and_then(|s| Type::from_string(s))
                            .unwrap_or(Type::Unknown);
                        (name.clone(), ty)
                    })
                    .collect();
                
                let ret_type = return_type.as_ref()
                    .and_then(|s| Type::from_string(s))
                    .unwrap_or(Type::None);
                
                self.begin_scope();
                self.current_function_return = Some(ret_type.clone());
                
                // Define parameters in scope
                for (param_name, param_type) in &param_types {
                    self.define_variable(param_name.clone(), param_type.clone());
                }
                
                self.check_block(body);
                self.current_function_return = None;
                self.end_scope();
                
                Type::None
            },
            _ => Type::None,
        }
    }
    
    fn check_block(&mut self, statements: &[Stmt]) -> Type {
        let mut last_type = Type::None;
        for stmt in statements {
            last_type = self.check_statement(stmt);
        }
        last_type
    }
    
    fn check_expression(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::Number(n) => {
                if n.fract() == 0.0 {
                    Type::I32
                } else {
                    Type::F64
                }
            },
            Expr::String(_) => Type::Str,
            Expr::Boolean(_) => Type::Bool,
            Expr::Identifier(name) => {
                self.get_variable_type(name).unwrap_or_else(|| {
                    self.reporter.error(
                        format!("Undefined variable: {}", name),
                        None,
                    );
                    Type::Unknown
                })
            },
            Expr::Match { expr, arms } => {
                let _expr_type = self.check_expression(expr);
                // Check each arm's pattern and body
                for arm in arms {
                    self.begin_scope();
                    // Pattern matching type checking (simplified)
                    self.check_block(&arm.body);
                    self.end_scope();
                }
                // Return type of first arm (simplified)
                Type::Any
            },
            Expr::Destructure { pattern: _, value } => {
                self.check_expression(value);
                Type::None
            },
            Expr::Async { body } => {
                self.check_expression(body)
            },
            Expr::Await { expr } => {
                self.check_expression(expr)
            },
            Expr::Yield(expr) => {
                if let Some(e) = expr {
                    self.check_expression(e)
                } else {
                    Type::None
                }
            },
            Expr::Generator { body } => {
                self.check_block(body);
                Type::Any // Generator type
            },
            Expr::Coroutine { body } => {
                self.check_block(body);
                Type::Any // Coroutine type
            },
            Expr::Suspend => Type::None,
            Expr::Resume { coroutine } => {
                self.check_expression(coroutine);
                Type::Any
            },
            Expr::Binary { left, op, right } => {
                let left_type = self.check_expression(left);
                let right_type = self.check_expression(right);
                
                match op {
                    BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply | 
                    BinaryOp::Divide | BinaryOp::Modulo => {
                        if left_type.is_numeric() && right_type.is_numeric() {
                            // Promote to the larger type
                            if left_type.is_float() || right_type.is_float() {
                                Type::F64
                            } else if left_type == Type::I64 || right_type == Type::I64 {
                                Type::I64
                            } else {
                                Type::I32
                            }
                        } else if left_type == Type::Str || right_type == Type::Str {
                            Type::Str // String concatenation
                        } else {
                            self.reporter.error(
                                format!("Cannot apply {:?} to {} and {}", 
                                    op, left_type.to_string(), right_type.to_string()),
                                None,
                            );
                            Type::Unknown
                        }
                    },
                    BinaryOp::Equal | BinaryOp::NotEqual => {
                        if left_type.can_coerce_to(&right_type) || right_type.can_coerce_to(&left_type) {
                            Type::Bool
                        } else {
                            self.reporter.error(
                                format!("Cannot compare {} and {}", 
                                    left_type.to_string(), right_type.to_string()),
                                None,
                            );
                            Type::Unknown
                        }
                    },
                    BinaryOp::LessThan | BinaryOp::LessEqual | 
                    BinaryOp::GreaterThan | BinaryOp::GreaterEqual => {
                        if left_type.is_numeric() && right_type.is_numeric() {
                            Type::Bool
                        } else {
                            self.reporter.error(
                                format!("Cannot compare {} and {} with {:?}", 
                                    left_type.to_string(), right_type.to_string(), op),
                                None,
                            );
                            Type::Unknown
                        }
                    },
                    BinaryOp::And | BinaryOp::Or => {
                        if left_type == Type::Bool && right_type == Type::Bool {
                            Type::Bool
                        } else {
                            self.reporter.error(
                                format!("Logical operators require bool, got {} and {}", 
                                    left_type.to_string(), right_type.to_string()),
                                None,
                            );
                            Type::Unknown
                        }
                    },
                }
            },
            Expr::Unary { op, expr } => {
                let expr_type = self.check_expression(expr);
                match op {
                    UnaryOp::Not => {
                        if expr_type == Type::Bool {
                            Type::Bool
                        } else {
                            self.reporter.error(
                                format!("Not operator requires bool, got {}", expr_type.to_string()),
                                None,
                            );
                            Type::Unknown
                        }
                    },
                    UnaryOp::Negate => {
                        if expr_type.is_numeric() {
                            expr_type
                        } else {
                            self.reporter.error(
                                format!("Negation requires numeric type, got {}", expr_type.to_string()),
                                None,
                            );
                            Type::Unknown
                        }
                    },
                }
            },
            Expr::Call { callee, args } => {
                let (param_types, return_type) = if let Some(info) = self.functions.get(callee) {
                    (info.0.clone(), info.1.clone())
                } else {
                    // Built-in function - check based on name
                    match callee.as_str() {
                        "print" => {
                            for arg in args {
                                self.check_expression(arg);
                            }
                            return Type::None;
                        },
                        "range" => {
                            if args.len() == 1 || args.len() == 2 {
                                for arg in args {
                                    let arg_type = self.check_expression(arg);
                                    if !arg_type.is_integer() && arg_type != Type::Unknown && arg_type != Type::Any {
                                        self.reporter.error(
                                            format!("range() expects integer arguments, got {}", arg_type.to_string()),
                                            None,
                                        );
                                    }
                                }
                                return Type::List(Box::new(Type::I32));
                            } else {
                                self.reporter.error(
                                    format!("range() expects 1 or 2 arguments, got {}", args.len()),
                                    None,
                                );
                                return Type::Unknown;
                            }
                        },
                        _ => {
                            self.reporter.error(
                                format!("Unknown function: {}", callee),
                                None,
                            );
                            return Type::Unknown;
                        }
                    }
                };
                
                if args.len() != param_types.len() {
                    self.reporter.error(
                        format!("Function '{}' expects {} arguments, got {}", 
                            callee, param_types.len(), args.len()),
                        None,
                    );
                    return Type::Unknown;
                }
                
                for (i, (arg, (_, param_type))) in args.iter().zip(param_types.iter()).enumerate() {
                    let arg_type = self.check_expression(arg);
                    if !arg_type.can_coerce_to(param_type) && arg_type != Type::Unknown {
                        self.reporter.error(
                            format!("Argument {} to '{}': expected {}, got {}", 
                                i + 1, callee, param_type.to_string(), arg_type.to_string()),
                            None,
                        );
                    }
                }
                
                return_type
            },
            Expr::Member { object, member: _ } => {
                let obj_type = self.check_expression(object);
                // For now, we only support maps
                if let Type::Map(_, value_type) = obj_type {
                    *value_type.clone()
                } else {
                    self.reporter.error(
                        format!("Cannot access member on {}", obj_type.to_string()),
                        None,
                    );
                    Type::Unknown
                }
            },
            Expr::Index { object, index } => {
                let obj_type = self.check_expression(object);
                let idx_type = self.check_expression(index);
                
                if !idx_type.is_integer() && idx_type != Type::Unknown && idx_type != Type::Any {
                    self.reporter.error(
                        format!("Index must be integer, got {}", idx_type.to_string()),
                        None,
                    );
                }
                
                match obj_type {
                    Type::List(elem_type) => *elem_type,
                    Type::Map(_, value_type) => *value_type,
                    Type::Any | Type::Unknown => Type::Any,
                    _ => {
                        self.reporter.error(
                            format!("Cannot index {}", obj_type.to_string()),
                            None,
                        );
                        Type::Unknown
                    }
                }
            },
            Expr::List(elements) => {
                if elements.is_empty() {
                    Type::List(Box::new(Type::Any))
                } else {
                    let first_type = self.check_expression(&elements[0]);
                    for elem in elements.iter().skip(1) {
                        let elem_type = self.check_expression(elem);
                        if elem_type != first_type && elem_type != Type::Unknown && first_type != Type::Unknown {
                            self.reporter.warning(
                                format!("List contains mixed types: {} and {}", 
                                    first_type.to_string(), elem_type.to_string()),
                                None,
                            );
                        }
                    }
                    Type::List(Box::new(first_type))
                }
            },
            Expr::Map(pairs) => {
                let mut key_type = Type::Unknown;
                let mut value_type = Type::Unknown;
                
                for (key, value) in pairs {
                    let kt = self.check_expression(key);
                    let vt = self.check_expression(value);
                    
                    if key_type == Type::Unknown {
                        key_type = kt.clone();
                    } else if kt != key_type && kt != Type::Unknown {
                        self.reporter.warning(
                            format!("Map contains mixed key types: {} and {}", 
                                key_type.to_string(), kt.to_string()),
                            None,
                        );
                    }
                    
                    if value_type == Type::Unknown {
                        value_type = vt.clone();
                    } else if vt != value_type && vt != Type::Unknown {
                        self.reporter.warning(
                            format!("Map contains mixed value types: {} and {}", 
                                value_type.to_string(), vt.to_string()),
                            None,
                        );
                    }
                }
                
                if key_type == Type::Unknown {
                    key_type = Type::Str; // Default to string keys
                }
                if value_type == Type::Unknown {
                    value_type = Type::Any;
                }
                
                Type::Map(Box::new(key_type), Box::new(value_type))
            },
            Expr::None => Type::None,
        }
    }
    
    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    
    fn end_scope(&mut self) {
        self.scopes.pop();
    }
    
    fn define_variable(&mut self, name: String, ty: Type) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, ty);
        }
    }
    
    fn get_variable_type(&self, name: &str) -> Option<Type> {
        // Check scopes from innermost to outermost
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        None
    }
    
    pub fn print_diagnostics(&self) {
        self.reporter.print_all();
    }
    
    pub fn has_errors(&self) -> bool {
        self.reporter.has_errors()
    }
}

