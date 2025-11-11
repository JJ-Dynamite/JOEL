#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    // Primitive types
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
    Str,
    Char,
    
    // Collection types
    List(Box<Type>),
    Map(Box<Type>, Box<Type>), // key type, value type
    
    // Special types
    Option(Box<Type>),
    Result(Box<Type>, Box<Type>), // Ok type, Err type
    Bytes,
    None,
    
    // Function type
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    
    // Generic/unknown
    Unknown,
    Any, // For interpreted mode
    
    // User-defined types
    Named(String),
}

impl Type {
    pub fn from_string(s: &str) -> Option<Type> {
        match s {
            "i8" => Some(Type::I8),
            "i16" => Some(Type::I16),
            "i32" => Some(Type::I32),
            "i64" => Some(Type::I64),
            "u8" => Some(Type::U8),
            "u16" => Some(Type::U16),
            "u32" => Some(Type::U32),
            "u64" => Some(Type::U64),
            "f32" => Some(Type::F32),
            "f64" => Some(Type::F64),
            "bool" => Some(Type::Bool),
            "str" => Some(Type::Str),
            "char" => Some(Type::Char),
            "Bytes" => Some(Type::Bytes),
            "None" => Some(Type::None),
            _ => {
                // Check for generic types like list[i32], map[str, i32], Option[i32]
                if s.starts_with("list[") && s.ends_with("]") {
                    let inner = &s[5..s.len()-1];
                    Type::from_string(inner).map(|t| Type::List(Box::new(t)))
                } else if s.starts_with("map[") && s.ends_with("]") {
                    let inner = &s[4..s.len()-1];
                    let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
                    if parts.len() == 2 {
                        if let (Some(k), Some(v)) = (Type::from_string(parts[0]), Type::from_string(parts[1])) {
                            Some(Type::Map(Box::new(k), Box::new(v)))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else if s.starts_with("Option[") && s.ends_with("]") {
                    let inner = &s[7..s.len()-1];
                    Type::from_string(inner).map(|t| Type::Option(Box::new(t)))
                } else if s.starts_with("Result[") && s.ends_with("]") {
                    let inner = &s[7..s.len()-1];
                    let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
                    if parts.len() == 2 {
                        if let (Some(ok), Some(err)) = (Type::from_string(parts[0]), Type::from_string(parts[1])) {
                            Some(Type::Result(Box::new(ok), Box::new(err)))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    Some(Type::Named(s.to_string()))
                }
            }
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            Type::I8 => "i8".to_string(),
            Type::I16 => "i16".to_string(),
            Type::I32 => "i32".to_string(),
            Type::I64 => "i64".to_string(),
            Type::U8 => "u8".to_string(),
            Type::U16 => "u16".to_string(),
            Type::U32 => "u32".to_string(),
            Type::U64 => "u64".to_string(),
            Type::F32 => "f32".to_string(),
            Type::F64 => "f64".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Str => "str".to_string(),
            Type::Char => "char".to_string(),
            Type::Bytes => "Bytes".to_string(),
            Type::None => "None".to_string(),
            Type::List(inner) => format!("list[{}]", inner.to_string()),
            Type::Map(k, v) => format!("map[{}, {}]", k.to_string(), v.to_string()),
            Type::Option(inner) => format!("Option[{}]", inner.to_string()),
            Type::Result(ok, err) => format!("Result[{}, {}]", ok.to_string(), err.to_string()),
            Type::Function { params, return_type } => {
                let param_strs: Vec<String> = params.iter().map(|t| t.to_string()).collect();
                format!("fn({}) -> {}", param_strs.join(", "), return_type.to_string())
            },
            Type::Unknown => "?".to_string(),
            Type::Any => "any".to_string(),
            Type::Named(name) => name.clone(),
        }
    }
    
    pub fn is_numeric(&self) -> bool {
        matches!(self, 
            Type::I8 | Type::I16 | Type::I32 | Type::I64 |
            Type::U8 | Type::U16 | Type::U32 | Type::U64 |
            Type::F32 | Type::F64
        )
    }
    
    pub fn is_integer(&self) -> bool {
        matches!(self,
            Type::I8 | Type::I16 | Type::I32 | Type::I64 |
            Type::U8 | Type::U16 | Type::U32 | Type::U64
        )
    }
    
    pub fn is_float(&self) -> bool {
        matches!(self, Type::F32 | Type::F64)
    }
    
    pub fn can_coerce_to(&self, target: &Type) -> bool {
        if self == target {
            return true;
        }
        
        match (self, target) {
            // Numeric coercions
            (Type::I8, Type::I16) | (Type::I8, Type::I32) | (Type::I8, Type::I64) => true,
            (Type::I16, Type::I32) | (Type::I16, Type::I64) => true,
            (Type::I32, Type::I64) => true,
            (Type::U8, Type::U16) | (Type::U8, Type::U32) | (Type::U8, Type::U64) => true,
            (Type::U16, Type::U32) | (Type::U16, Type::U64) => true,
            (Type::U32, Type::U64) => true,
            // Integer to float
            (t, Type::F32) if t.is_integer() => true,
            (t, Type::F64) if t.is_integer() => true,
            (Type::F32, Type::F64) => true,
            // Any type can be coerced to Any
            (_, Type::Any) => true,
            _ => false,
        }
    }
    
    pub fn infer_from_literal(value: &crate::ast::Expr) -> Type {
        match value {
            crate::ast::Expr::Number(n) => {
                if n.fract() == 0.0 {
                    Type::I32 // Default to i32 for integers
                } else {
                    Type::F64 // Default to f64 for floats
                }
            },
            crate::ast::Expr::String(_) => Type::Str,
            crate::ast::Expr::Boolean(_) => Type::Bool,
            crate::ast::Expr::List(_) => Type::List(Box::new(Type::Any)),
            crate::ast::Expr::Map(_) => Type::Map(Box::new(Type::Str), Box::new(Type::Any)),
            crate::ast::Expr::None => Type::None,
            _ => Type::Unknown,
        }
    }
}

