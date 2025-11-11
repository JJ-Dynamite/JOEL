// Standard Library Core Modules for JOEL

pub mod core {
    // Core utilities and functions
    
    pub fn print(args: &[crate::vm::Value]) -> crate::vm::Value {
        let mut parts = Vec::new();
        for arg in args {
            parts.push(value_to_string(arg));
        }
        println!("{}", parts.join(" "));
        crate::vm::Value::None
    }
    
    pub fn range(args: &[crate::vm::Value]) -> Result<crate::vm::Value, String> {
        if args.len() == 2 {
            if let (crate::vm::Value::Number(s), crate::vm::Value::Number(e)) = (&args[0], &args[1]) {
                let mut list = Vec::new();
                for i in (*s as i64)..(*e as i64) {
                    list.push(crate::vm::Value::Number(i as f64));
                }
                Ok(crate::vm::Value::List(list))
            } else {
                Err("range() expects two numbers".to_string())
            }
        } else if args.len() == 1 {
            if let crate::vm::Value::Number(e) = &args[0] {
                let mut list = Vec::new();
                for i in 0..(*e as i64) {
                    list.push(crate::vm::Value::Number(i as f64));
                }
                Ok(crate::vm::Value::List(list))
            } else {
                Err("range() expects a number".to_string())
            }
        } else {
            Err("range() expects 1 or 2 arguments".to_string())
        }
    }
    
    fn value_to_string(value: &crate::vm::Value) -> String {
        match value {
            crate::vm::Value::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            },
            crate::vm::Value::String(s) => s.clone(),
            crate::vm::Value::Boolean(b) => format!("{}", b),
            crate::vm::Value::List(list) => {
                let items: Vec<String> = list.iter().map(|v| value_to_string(v)).collect();
                format!("[{}]", items.join(", "))
            },
            crate::vm::Value::Map(map) => {
                let pairs: Vec<String> = map.iter()
                    .map(|(k, v)| format!("{}: {}", k, value_to_string(v)))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            },
            crate::vm::Value::Function { name, .. } => format!("<function {}>", name),
            crate::vm::Value::None => "None".to_string(),
        }
    }
}

pub mod math {
    // Mathematical functions
    
    pub fn abs(x: f64) -> f64 {
        x.abs()
    }
    
    pub fn min(a: f64, b: f64) -> f64 {
        a.min(b)
    }
    
    pub fn max(a: f64, b: f64) -> f64 {
        a.max(b)
    }
    
    pub fn sqrt(x: f64) -> f64 {
        x.sqrt()
    }
    
    pub fn pow(base: f64, exp: f64) -> f64 {
        base.powf(exp)
    }
    
    pub fn sin(x: f64) -> f64 {
        x.sin()
    }
    
    pub fn cos(x: f64) -> f64 {
        x.cos()
    }
    
    pub fn tan(x: f64) -> f64 {
        x.tan()
    }
}

pub mod string {
    // String manipulation functions
    
    pub fn len(s: &str) -> usize {
        s.len()
    }
    
    pub fn contains(haystack: &str, needle: &str) -> bool {
        haystack.contains(needle)
    }
    
    pub fn split(s: &str, delimiter: &str) -> Vec<String> {
        s.split(delimiter).map(|s| s.to_string()).collect()
    }
    
    pub fn join(parts: &[String], separator: &str) -> String {
        parts.join(separator)
    }
    
    pub fn trim(s: &str) -> String {
        s.trim().to_string()
    }
    
    pub fn to_upper(s: &str) -> String {
        s.to_uppercase()
    }
    
    pub fn to_lower(s: &str) -> String {
        s.to_lowercase()
    }
}

pub mod collections {
    // Collection utilities
    
    use std::collections::HashMap;
    use crate::vm::Value;
    
    pub fn len(list: &[Value]) -> usize {
        list.len()
    }
    
    pub fn push(list: &mut Vec<Value>, item: Value) {
        list.push(item);
    }
    
    pub fn pop(list: &mut Vec<Value>) -> Option<Value> {
        list.pop()
    }
    
    pub fn map_len(map: &HashMap<String, Value>) -> usize {
        map.len()
    }
    
    pub fn map_get(map: &HashMap<String, Value>, key: &str) -> Option<&Value> {
        map.get(key)
    }
    
    pub fn map_insert(map: &mut HashMap<String, Value>, key: String, value: Value) {
        map.insert(key, value);
    }
}

