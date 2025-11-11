use crate::ast::*;
use crate::pattern_matcher::PatternMatcher;
use crate::async_runtime::AsyncRuntime;
use crate::iterator::{JoelIterator, RangeIterator, ListIterator};
use crate::parallel::ParallelRuntime;
use crate::exhaustiveness_checker::ExhaustivenessChecker;
use crate::coroutine::CoroutineRuntime;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
    Function {
        name: String,
        params: Vec<(String, Option<String>)>,
        body: Vec<Stmt>,
    },
    None,
}

pub struct VM {
    globals: HashMap<String, Value>,
    stack: Vec<HashMap<String, Value>>,
    async_runtime: AsyncRuntime,
    parallel_runtime: ParallelRuntime,
    coroutine_runtime: CoroutineRuntime,
}

impl VM {
    pub fn new() -> Self {
        let mut vm = Self {
            globals: HashMap::new(),
            stack: Vec::new(),
            async_runtime: AsyncRuntime::new(),
            parallel_runtime: ParallelRuntime::new(4), // 4 threads by default
            coroutine_runtime: CoroutineRuntime::new(),
        };
        vm.init_builtins();
        vm
    }
    
    fn init_builtins(&mut self) {
        // Built-in functions will be added here
    }
    
    pub fn interpret(&mut self, program: &Program) -> Result<(), String> {
        println!("🚀 JOEL Runtime - Mode: {:?}", program.mode);
        if let Some(ref target) = program.target {
            println!("   Target: {}", target);
        }
        println!();
        
        for stmt in &program.statements {
            self.execute(stmt)?;
        }
        
        Ok(())
    }
    
    fn execute(&mut self, stmt: &Stmt) -> Result<Value, String> {
        match stmt {
            Stmt::Let { name, value, .. } => {
                let val = self.evaluate(value)?;
                self.define_variable(name.clone(), val.clone());
                Ok(val)
            },
            Stmt::Const { name, value, .. } => {
                let val = self.evaluate(value)?;
                self.define_variable(name.clone(), val.clone());
                Ok(val)
            },
            Stmt::Expr(expr) => {
                self.evaluate(expr)
            },
            Stmt::Print(expr) => {
                let value = self.evaluate(expr)?;
                println!("{}", self.value_to_string(&value));
                Ok(Value::None)
            },
            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    self.evaluate(expr)
                } else {
                    Ok(Value::None)
                }
            },
            Stmt::If { condition, then_branch, else_branch } => {
                let cond = self.evaluate(condition)?;
                if self.is_truthy(&cond) {
                    self.execute_block(then_branch)
                } else if let Some(else_branch) = else_branch {
                    self.execute_block(else_branch)
                } else {
                    Ok(Value::None)
                }
            },
            Stmt::While { condition, body } => {
                loop {
                    let cond_val = self.evaluate(condition)?;
                    if !self.is_truthy(&cond_val) {
                        break;
                    }
                    self.execute_block(body)?;
                }
                Ok(Value::None)
            },
            Stmt::For { var, iterable, body } => {
                let iter_val = self.evaluate(iterable)?;
                match iter_val {
                    Value::List(list) => {
                        let mut iter = ListIterator::new(list);
                        while iter.has_next() {
                            if let Some(item) = iter.next() {
                                self.define_variable(var.clone(), item);
                                self.execute_block(body)?;
                            }
                        }
                    },
                    Value::Number(end) => {
                        // Range iteration: for i in range(0, n)
                        let end_val = end as i64;
                        let mut iter = RangeIterator::new(0, end_val, 1);
                        while iter.has_next() {
                            if let Some(item) = iter.next() {
                                self.define_variable(var.clone(), item);
                                self.execute_block(body)?;
                            }
                        }
                    },
                    _ => {
                        return Err(format!("For loop expects a list or range, got {:?}", iter_val));
                    },
                }
                Ok(Value::None)
            },
            Stmt::Block(statements) => {
                self.execute_block(statements)
            },
            Stmt::Fn { name, params, body, .. } => {
                let func = Value::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                };
                self.define_variable(name.clone(), func);
                Ok(Value::None)
            },
            Stmt::Module { name } => {
                println!("📦 Module: {}", name);
                Ok(Value::None)
            },
            Stmt::Import { module, alias } => {
                let import_name = alias.as_ref().unwrap_or(module);
                println!("📥 Import: {} as {}", module, import_name);
                Ok(Value::None)
            },
            Stmt::Actor { name, fields, methods } => {
                println!("🎭 Actor: {} ({} fields, {} methods)", name, fields.len(), methods.len());
                Ok(Value::None)
            },
            Stmt::MatchStmt { expr, arms } => {
                // Check exhaustiveness (warning only)
                if let Err(e) = ExhaustivenessChecker::check_exhaustive(expr, arms) {
                    eprintln!("⚠️  Exhaustiveness warning: {}", e);
                }
                
                // Check for overlapping patterns
                if let Err(e) = ExhaustivenessChecker::check_overlap(arms) {
                    return Err(format!("Pattern matching error: {}", e));
                }
                
                let value = self.evaluate(expr)?;
                for arm in arms {
                    if PatternMatcher::matches_nested(&arm.pattern, &value) {
                        // Check guard if present
                        if let Some(ref guard) = arm.guard {
                            let guard_result = self.evaluate(guard)?;
                            if !self.is_truthy(&guard_result) {
                                continue;
                            }
                        }
                        
                        // Extract bindings
                        let bindings = PatternMatcher::extract_bindings(&arm.pattern, &value);
                        for (name, val) in bindings {
                            self.define_variable(name, val);
                        }
                        
                        // Execute matched arm
                        return self.execute_block(&arm.body);
                    }
                }
                Err("No pattern matched".to_string())
            },
            Stmt::AsyncFn { name, params, return_type: _, body } => {
                // Store async function (simplified - would need proper async runtime)
                self.define_variable(name.clone(), Value::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                });
                Ok(Value::None)
            },
            Stmt::ParallelFor { var, iterable, body } => {
                let iter_val = self.evaluate(iterable)?;
                match iter_val {
                    Value::List(list) => {
                        // Execute in parallel (simplified - would need VM context in closure)
                        // For now, just execute sequentially
                        for item in list {
                            self.define_variable(var.clone(), item);
                            self.execute_block(body)?;
                        }
                    },
                    _ => {
                        return Err(format!("Parallel for expects a list, got {:?}", iter_val));
                    },
                }
                Ok(Value::None)
            },
            Stmt::ParallelMap { var, iterable, body } => {
                let iter_val = self.evaluate(iterable)?;
                match iter_val {
                    Value::List(list) => {
                        // Map in parallel (simplified - would need VM context)
                        let mut results = Vec::new();
                        for item in list {
                            self.define_variable(var.clone(), item.clone());
                            let result = self.execute_block(body)?;
                            results.push(result);
                        }
                        Ok(Value::List(results))
                    },
                    _ => {
                        return Err(format!("Parallel map expects a list, got {:?}", iter_val));
                    },
                }
            },
            Stmt::CoroutineFn { name, params, return_type: _, body } => {
                // Store coroutine function (simplified - would need proper coroutine runtime)
                self.define_variable(name.clone(), Value::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                });
                Ok(Value::None)
            },
            Stmt::Contract { name, fields, methods } => {
                println!("📜 Contract: {} ({} fields, {} methods)", name, fields.len(), methods.len());
                Ok(Value::None)
            },
            Stmt::Component { name, body } => {
                println!("🎨 Component: {} ({} statements)", name, body.len());
                Ok(Value::None)
            },
            Stmt::Flow { name, nodes } => {
                println!("🔄 Flow: {} ({} nodes)", name, nodes.len());
                Ok(Value::None)
            },
            Stmt::Deployment { name, config } => {
                println!("🐳 Deployment: {} ({} config items)", name, config.len());
                Ok(Value::None)
            },
            Stmt::Cluster { name, config } => {
                println!("☸️  Cluster: {} ({} config items)", name, config.len());
                Ok(Value::None)
            },
        }
    }
    
    fn execute_block(&mut self, statements: &[Stmt]) -> Result<Value, String> {
        self.stack.push(HashMap::new());
        
        for stmt in statements {
            self.execute(stmt)?;
        }
        
        self.stack.pop();
        Ok(Value::None)
    }
    
    fn evaluate(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
            Expr::Identifier(name) => {
                self.get_variable(name)
            },
            Expr::Binary { left, op, right } => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.binary_op(&left_val, op, &right_val)
            },
            Expr::Unary { op, expr } => {
                let val = self.evaluate(expr)?;
                self.unary_op(op, &val)
            },
            Expr::Call { callee, args } => {
                let func = self.get_variable(callee)?;
                match func {
                    Value::Function { params, body, .. } => {
                        self.call_function(&params, args, &body)
                    },
                    _ => {
                        // Built-in functions
                        self.call_builtin(callee, args)
                    },
                }
            },
            Expr::Member { object, member } => {
                let obj = self.evaluate(object)?;
                match obj {
                    Value::Map(map) => {
                        map.get(member).cloned().ok_or_else(|| format!("No member '{}'", member))
                    },
                    _ => Err(format!("Cannot access member '{}' on non-object", member)),
                }
            },
            Expr::Index { object, index } => {
                let obj = self.evaluate(object)?;
                let idx = self.evaluate(index)?;
                match (obj, idx) {
                    (Value::List(list), Value::Number(n)) => {
                        let i = n as usize;
                        if i < list.len() {
                            Ok(list[i].clone())
                        } else {
                            Err(format!("Index {} out of bounds", i))
                        }
                    },
                    _ => Err("Invalid index operation".to_string()),
                }
            },
            Expr::List(elements) => {
                let mut list = Vec::new();
                for elem in elements {
                    list.push(self.evaluate(elem)?);
                }
                Ok(Value::List(list))
            },
            Expr::Map(pairs) => {
                let mut map = HashMap::new();
                for (key, value) in pairs {
                    let key_str = match key {
                        Expr::String(s) => s.clone(),
                        Expr::Identifier(s) => s.clone(),
                        _ => return Err("Map keys must be strings or identifiers".to_string()),
                    };
                    map.insert(key_str, self.evaluate(value)?);
                }
                Ok(Value::Map(map))
            },
            Expr::None => Ok(Value::None),
            Expr::Match { expr, arms } => {
                let value = self.evaluate(expr)?;
                for arm in arms {
                    if PatternMatcher::matches(&arm.pattern, &value) {
                        if let Some(ref guard) = arm.guard {
                            let guard_result = self.evaluate(guard)?;
                            if !self.is_truthy(&guard_result) {
                                continue;
                            }
                        }
                        
                        let bindings = PatternMatcher::extract_bindings(&arm.pattern, &value);
                        for (name, val) in bindings {
                            self.define_variable(name, val);
                        }
                        
                        return self.execute_block(&arm.body);
                    }
                }
                Err("No pattern matched".to_string())
            },
            Expr::Destructure { pattern, value } => {
                let val = self.evaluate(value)?;
                let bindings = PatternMatcher::extract_bindings(&pattern, &val);
                for (name, val) in bindings {
                    self.define_variable(name, val);
                }
                Ok(Value::None)
            },
            Expr::Async { body } => {
                // Async expression (simplified - would need async runtime)
                self.evaluate(body)
            },
            Expr::Await { expr } => {
                // Await expression (simplified - would need async runtime)
                self.evaluate(expr)
            },
            Expr::Yield(expr) => {
                // Yield expression (simplified - would need generator runtime)
                if let Some(expr) = expr {
                    self.evaluate(expr)
                } else {
                    Ok(Value::None)
                }
            },
            Expr::Generator { body } => {
                // Create generator (simplified)
                self.execute_block(body)?;
                Ok(Value::None) // Generator would return iterator in full implementation
            },
            Expr::Coroutine { body } => {
                // Create coroutine (simplified)
                let coroutine_id = self.coroutine_runtime.create();
                // Execute coroutine body (simplified - would need separate execution context)
                self.execute_block(body)?;
                Ok(Value::Number(coroutine_id as f64)) // Return coroutine ID
            },
            Expr::Suspend => {
                // Suspend current coroutine (simplified)
                Ok(Value::None)
            },
            Expr::Resume { coroutine } => {
                // Resume coroutine
                let coroutine_val = self.evaluate(coroutine)?;
                if let Value::Number(id) = coroutine_val {
                    let coroutine_id = id as usize;
                    self.coroutine_runtime.resume(coroutine_id)
                        .map_err(|e| format!("Failed to resume coroutine: {}", e))
                } else {
                    Err("Resume expects a coroutine ID".to_string())
                }
            },
        }
    }
    
    fn binary_op(&self, left: &Value, op: &BinaryOp, right: &Value) -> Result<Value, String> {
        match (left, op, right) {
            (Value::Number(a), BinaryOp::Add, Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::Number(a), BinaryOp::Subtract, Value::Number(b)) => Ok(Value::Number(a - b)),
            (Value::Number(a), BinaryOp::Multiply, Value::Number(b)) => Ok(Value::Number(a * b)),
            (Value::Number(a), BinaryOp::Divide, Value::Number(b)) => {
                if *b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Number(a / b))
                }
            },
            (Value::Number(a), BinaryOp::Modulo, Value::Number(b)) => Ok(Value::Number(a % b)),
            (Value::String(a), BinaryOp::Add, Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::String(a), BinaryOp::Add, Value::Number(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::Number(a), BinaryOp::Add, Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::Number(a), BinaryOp::Equal, Value::Number(b)) => Ok(Value::Boolean(a == b)),
            (Value::String(a), BinaryOp::Equal, Value::String(b)) => Ok(Value::Boolean(a == b)),
            (Value::Boolean(a), BinaryOp::Equal, Value::Boolean(b)) => Ok(Value::Boolean(a == b)),
            (Value::Number(a), BinaryOp::NotEqual, Value::Number(b)) => Ok(Value::Boolean(a != b)),
            (Value::Number(a), BinaryOp::LessThan, Value::Number(b)) => Ok(Value::Boolean(a < b)),
            (Value::Number(a), BinaryOp::LessEqual, Value::Number(b)) => Ok(Value::Boolean(a <= b)),
            (Value::Number(a), BinaryOp::GreaterThan, Value::Number(b)) => Ok(Value::Boolean(a > b)),
            (Value::Number(a), BinaryOp::GreaterEqual, Value::Number(b)) => Ok(Value::Boolean(a >= b)),
            (Value::Boolean(a), BinaryOp::And, Value::Boolean(b)) => Ok(Value::Boolean(*a && *b)),
            (Value::Boolean(a), BinaryOp::Or, Value::Boolean(b)) => Ok(Value::Boolean(*a || *b)),
            _ => Err(format!("Invalid binary operation: {:?} {:?} {:?}", left, op, right)),
        }
    }
    
    fn unary_op(&self, op: &UnaryOp, val: &Value) -> Result<Value, String> {
        match (op, val) {
            (UnaryOp::Not, Value::Boolean(b)) => Ok(Value::Boolean(!b)),
            (UnaryOp::Negate, Value::Number(n)) => Ok(Value::Number(-n)),
            _ => Err(format!("Invalid unary operation: {:?} {:?}", op, val)),
        }
    }
    
    fn call_function(&mut self, params: &[(String, Option<String>)], args: &[Expr], body: &[Stmt]) -> Result<Value, String> {
        if args.len() != params.len() {
            return Err(format!("Expected {} arguments, got {}", params.len(), args.len()));
        }
        
        self.stack.push(HashMap::new());
        
        for (i, (param_name, _)) in params.iter().enumerate() {
            let arg_val = self.evaluate(&args[i])?;
            self.stack.last_mut().unwrap().insert(param_name.clone(), arg_val);
        }
        
        let mut result = Value::None;
        for stmt in body {
            result = self.execute(stmt)?;
        }
        
        self.stack.pop();
        Ok(result)
    }
    
    fn call_builtin(&mut self, name: &str, args: &[Expr]) -> Result<Value, String> {
        match name {
            "print" => {
                let mut parts = Vec::new();
                for arg in args {
                    let val = self.evaluate(arg)?;
                    parts.push(self.value_to_string(&val));
                }
                println!("{}", parts.join(" "));
                Ok(Value::None)
            },
            "range" => {
                if args.len() == 2 {
                    let start = self.evaluate(&args[0])?;
                    let end = self.evaluate(&args[1])?;
                    if let (Value::Number(s), Value::Number(e)) = (start, end) {
                        let mut list = Vec::new();
                        for i in (s as i64)..(e as i64) {
                            list.push(Value::Number(i as f64));
                        }
                        Ok(Value::List(list))
                    } else {
                        Err("range() expects two numbers".to_string())
                    }
                } else if args.len() == 1 {
                    let end = self.evaluate(&args[0])?;
                    if let Value::Number(e) = end {
                        let mut list = Vec::new();
                        for i in 0..(e as i64) {
                            list.push(Value::Number(i as f64));
                        }
                        Ok(Value::List(list))
                    } else {
                        Err("range() expects a number".to_string())
                    }
                } else {
                    Err("range() expects 1 or 2 arguments".to_string())
                }
            },
            _ => Err(format!("Unknown function: {}", name)),
        }
    }
    
    fn define_variable(&mut self, name: String, value: Value) {
        if let Some(frame) = self.stack.last_mut() {
            frame.insert(name, value);
        } else {
            self.globals.insert(name, value);
        }
    }
    
    fn get_variable(&self, name: &str) -> Result<Value, String> {
        // Check stack frames (local scope)
        for frame in self.stack.iter().rev() {
            if let Some(value) = frame.get(name) {
                return Ok(value.clone());
            }
        }
        
        // Check globals
        if let Some(value) = self.globals.get(name) {
            return Ok(value.clone());
        }
        
        Err(format!("Undefined variable: {}", name))
    }
    
    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Boolean(false) => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::None => false,
            _ => true,
        }
    }
    
    fn value_to_string(&self, value: &Value) -> String {
        match value {
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            },
            Value::String(s) => s.clone(),
            Value::Boolean(b) => format!("{}", b),
            Value::List(list) => {
                let items: Vec<String> = list.iter().map(|v| self.value_to_string(v)).collect();
                format!("[{}]", items.join(", "))
            },
            Value::Map(map) => {
                let pairs: Vec<String> = map.iter()
                    .map(|(k, v)| format!("{}: {}", k, self.value_to_string(v)))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            },
            Value::Function { name, .. } => format!("<function {}>", name),
            Value::None => "None".to_string(),
        }
    }
}

