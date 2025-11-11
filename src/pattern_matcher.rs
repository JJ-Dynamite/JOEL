use crate::ast::*;
use crate::vm::Value;

/// Pattern matching engine
pub struct PatternMatcher;

/// Nested pattern matching support
impl PatternMatcher {
    /// Match nested patterns (patterns within patterns)
    pub fn matches_nested(pattern: &Pattern, value: &Value) -> bool {
        Self::matches_nested_recursive(pattern, value, 0)
    }
    
    fn matches_nested_recursive(pattern: &Pattern, value: &Value, depth: usize) -> bool {
        if depth > 100 {
            return false; // Prevent infinite recursion
        }
        
        match (pattern, value) {
            (Pattern::Wildcard, _) => true,
            (Pattern::Identifier(_), _) => true,
            (Pattern::Number(n), Value::Number(v)) => (n - v).abs() < f64::EPSILON,
            (Pattern::String(s), Value::String(v)) => s == v,
            (Pattern::Boolean(b), Value::Boolean(v)) => b == v,
            (Pattern::Tuple(patterns), Value::List(values)) => {
                if patterns.len() != values.len() {
                    return false;
                }
                patterns.iter().zip(values.iter())
                    .all(|(p, v)| Self::matches_nested_recursive(p, v, depth + 1))
            },
            (Pattern::List(patterns), Value::List(values)) => {
                // Support for list patterns with rest patterns (future)
                if patterns.len() != values.len() {
                    return false;
                }
                patterns.iter().zip(values.iter())
                    .all(|(p, v)| Self::matches_nested_recursive(p, v, depth + 1))
            },
            (Pattern::Struct { fields, .. }, Value::Map(map)) => {
                for (field_name, field_pattern) in fields {
                    if let Some(field_value) = map.get(field_name) {
                        if !Self::matches_nested_recursive(field_pattern, field_value, depth + 1) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                true
            },
            (Pattern::Or(patterns), value) => {
                patterns.iter().any(|p| Self::matches_nested_recursive(p, value, depth + 1))
            },
            (Pattern::Binding { pattern, .. }, value) => {
                Self::matches_nested_recursive(pattern, value, depth + 1)
            },
            (Pattern::Guard { pattern, .. }, value) => {
                Self::matches_nested_recursive(pattern, value, depth + 1)
            },
            _ => false,
        }
    }
}

impl PatternMatcher {
    /// Match a value against a pattern
    pub fn matches(pattern: &Pattern, value: &Value) -> bool {
        match (pattern, value) {
            (Pattern::Wildcard, _) => true,
            (Pattern::Identifier(_), _) => true, // Always matches, binds variable
            (Pattern::Number(n), Value::Number(v)) => (n - v).abs() < f64::EPSILON,
            (Pattern::String(s), Value::String(v)) => s == v,
            (Pattern::Boolean(b), Value::Boolean(v)) => b == v,
            (Pattern::Tuple(patterns), Value::List(values)) => {
                if patterns.len() != values.len() {
                    return false;
                }
                patterns.iter().zip(values.iter())
                    .all(|(p, v)| Self::matches(p, v))
            },
            (Pattern::List(patterns), Value::List(values)) => {
                // List pattern matching (simplified - would need more sophisticated matching)
                if patterns.len() != values.len() {
                    return false;
                }
                patterns.iter().zip(values.iter())
                    .all(|(p, v)| Self::matches(p, v))
            },
            (Pattern::Or(patterns), value) => {
                patterns.iter().any(|p| Self::matches(p, value))
            },
            (Pattern::Binding { pattern, .. }, value) => {
                Self::matches(pattern, value)
            },
            (Pattern::Guard { pattern, .. }, _) => {
                // Guard evaluation would need VM context
                Self::matches(pattern, value)
            },
            (Pattern::Struct { fields, .. }, Value::Map(map)) => {
                // Match struct pattern against map
                for (field_name, field_pattern) in fields {
                    if let Some(field_value) = map.get(field_name) {
                        if !Self::matches(field_pattern, field_value) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                true
            },
            (Pattern::Binding { pattern, .. }, value) => {
                Self::matches(pattern, value)
            },
            _ => false,
        }
    }
    
    /// Extract bindings from a pattern match
    pub fn extract_bindings(pattern: &Pattern, value: &Value) -> Vec<(String, Value)> {
        let mut bindings = Vec::new();
        Self::extract_bindings_recursive(pattern, value, &mut bindings);
        bindings
    }
    
    fn extract_bindings_recursive(pattern: &Pattern, value: &Value, bindings: &mut Vec<(String, Value)>) {
        match pattern {
            Pattern::Identifier(name) => {
                bindings.push((name.clone(), value.clone()));
            },
            Pattern::Binding { name, pattern } => {
                bindings.push((name.clone(), value.clone()));
                Self::extract_bindings_recursive(pattern, value, bindings);
            },
            Pattern::Tuple(patterns) => {
                if let Value::List(values) = value {
                    for (p, v) in patterns.iter().zip(values.iter()) {
                        Self::extract_bindings_recursive(p, v, bindings);
                    }
                }
            },
            Pattern::List(patterns) => {
                if let Value::List(values) = value {
                    for (p, v) in patterns.iter().zip(values.iter()) {
                        Self::extract_bindings_recursive(p, v, bindings);
                    }
                }
            },
            Pattern::Struct { fields, .. } => {
                if let Value::Map(map) = value {
                    for (field_name, field_pattern) in fields {
                        if let Some(field_value) = map.get(field_name) {
                            Self::extract_bindings_recursive(field_pattern, field_value, bindings);
                        }
                    }
                }
            },
            Pattern::Binding { name, pattern } => {
                bindings.push((name.clone(), value.clone()));
                Self::extract_bindings_recursive(pattern, value, bindings);
            },
            Pattern::Guard { pattern, .. } => {
                Self::extract_bindings_recursive(pattern, value, bindings);
            },
            Pattern::Or(patterns) => {
                // Extract bindings from first matching pattern
                for pattern in patterns {
                    if Self::matches(pattern, value) {
                        Self::extract_bindings_recursive(pattern, value, bindings);
                        break;
                    }
                }
            },
            _ => {}
        }
    }
}

