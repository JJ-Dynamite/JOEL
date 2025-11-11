use crate::lexer::Token;

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub location: Option<SourceLocation>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub line: usize,
    pub col: usize,
    pub file: Option<String>,
}

impl Diagnostic {
    pub fn error(message: String, location: Option<SourceLocation>) -> Self {
        Self {
            level: DiagnosticLevel::Error,
            message,
            location,
            notes: Vec::new(),
        }
    }
    
    pub fn warning(message: String, location: Option<SourceLocation>) -> Self {
        Self {
            level: DiagnosticLevel::Warning,
            message,
            location,
            notes: Vec::new(),
        }
    }
    
    pub fn with_note(mut self, note: String) -> Self {
        self.notes.push(note);
        self
    }
    
    pub fn from_token(level: DiagnosticLevel, message: String, token: &Token) -> Self {
        Self {
            level,
            message,
            location: Some(SourceLocation {
                line: token.line,
                col: token.col,
                file: None,
            }),
            notes: Vec::new(),
        }
    }
    
    pub fn format(&self, source_lines: &[String]) -> String {
        let mut output = String::new();
        
        let prefix = match self.level {
            DiagnosticLevel::Error => "❌ Error",
            DiagnosticLevel::Warning => "⚠️  Warning",
            DiagnosticLevel::Info => "ℹ️  Info",
        };
        
        if let Some(ref loc) = self.location {
            output.push_str(&format!("{}: {} (line {}, col {})\n", 
                prefix, self.message, loc.line, loc.col));
            
            // Show source line with caret
            if loc.line > 0 && loc.line <= source_lines.len() {
                let line_idx = loc.line - 1;
                let line = &source_lines[line_idx];
                output.push_str(&format!("  {} | {}\n", loc.line, line));
                
                // Show caret pointing to the error
                if loc.col > 0 {
                    let spaces = " ".repeat(format!("{}", loc.line).len() + 3 + (loc.col - 1).min(line.len()));
                    output.push_str(&format!("  {} | {}^\n", " ".repeat(format!("{}", loc.line).len()), spaces));
                }
            }
        } else {
            output.push_str(&format!("{}: {}\n", prefix, self.message));
        }
        
        for note in &self.notes {
            output.push_str(&format!("  💡 {}\n", note));
        }
        
        output
    }
}

pub struct DiagnosticReporter {
    diagnostics: Vec<Diagnostic>,
    source_lines: Vec<String>,
}

impl DiagnosticReporter {
    pub fn new(source: &str) -> Self {
        Self {
            diagnostics: Vec::new(),
            source_lines: source.lines().map(|s| s.to_string()).collect(),
        }
    }
    
    pub fn error(&mut self, message: String, location: Option<SourceLocation>) {
        self.diagnostics.push(Diagnostic::error(message, location));
    }
    
    pub fn warning(&mut self, message: String, location: Option<SourceLocation>) {
        self.diagnostics.push(Diagnostic::warning(message, location));
    }
    
    pub fn error_from_token(&mut self, message: String, token: &Token) {
        self.diagnostics.push(Diagnostic::from_token(
            DiagnosticLevel::Error,
            message,
            token,
        ));
    }
    
    pub fn warning_from_token(&mut self, message: String, token: &Token) {
        self.diagnostics.push(Diagnostic::from_token(
            DiagnosticLevel::Warning,
            message,
            token,
        ));
    }
    
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.level == DiagnosticLevel::Error)
    }
    
    pub fn print_all(&self) {
        for diag in &self.diagnostics {
            print!("{}", diag.format(&self.source_lines));
        }
    }
    
    pub fn get_errors(&self) -> Vec<&Diagnostic> {
        self.diagnostics.iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .collect()
    }
    
    pub fn get_warnings(&self) -> Vec<&Diagnostic> {
        self.diagnostics.iter()
            .filter(|d| d.level == DiagnosticLevel::Warning)
            .collect()
    }
}

