mod lexer;
mod parser;
mod ast;
mod vm;

use clap::{Parser as ClapParser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[derive(ClapParser)]
#[command(name = "joel")]
#[command(about = "JOEL Language - A polymodal programming language", long_about = None)]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a JOEL file in interpreted mode
    Run {
        /// Path to the JOEL source file
        file: PathBuf,
    },
    /// Build a JOEL file for a specific target
    Build {
        /// Path to the JOEL source file
        file: PathBuf,
        /// Target platform (native, wasm32, evm, wasm-solana)
        #[arg(short, long, default_value = "native")]
        target: String,
    },
    /// Show version information
    Version,
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Run { file } => {
            run_file(&file);
        },
        Commands::Build { file, target } => {
            build_file(&file, &target);
        },
        Commands::Version => {
            println!("JOEL Language v0.1.0");
            println!("A polymodal programming language");
        },
    }
}

fn run_file(file: &PathBuf) {
    let source = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Error reading file: {}", e);
            return;
        },
    };
    
    // Check for header
    if source.trim_start().starts_with("[Interpreted]") {
        // Tokenize
        let mut lexer = lexer::Lexer::new(&source);
        let tokens = lexer.tokenize();
        
        // Parse
        let mut parser = parser::Parser::new(tokens);
        let program = parser.parse();
        
        // Interpret
        let mut vm = vm::VM::new();
        if let Err(e) = vm.interpret(&program) {
            eprintln!("❌ Runtime error: {}", e);
        }
    } else if source.trim_start().starts_with("[Compiled]") {
        println!("⚙️  Compiling (AOT) ...");
        println!("⚠️  Compiled mode not yet implemented - use [Interpreted] for now");
    } else {
        eprintln!("❌ Error: missing [Compiled] or [Interpreted] header");
        eprintln!("   Add [Interpreted] or [Compiled] at the top of your file");
    }
}

fn build_file(file: &PathBuf, target: &str) {
    println!("🔨 Building: {} for target: {}\n", file.display(), target);
    
    let source = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Error reading file: {}", e);
            return;
        },
    };
    
    // Tokenize
    let mut lexer = lexer::Lexer::new(&source);
    let tokens = lexer.tokenize();
    
    // Parse
    let mut parser = parser::Parser::new(tokens);
    let program = parser.parse();
    
    match target {
        "native" => {
            println!("⚠️  Native compilation not yet implemented");
            println!("   Parsed {} statements", program.statements.len());
        },
        "wasm32" => {
            println!("⚠️  WASM compilation not yet implemented");
            println!("   Parsed {} statements", program.statements.len());
        },
        "evm" => {
            println!("⚠️  EVM compilation not yet implemented");
            println!("   Parsed {} statements", program.statements.len());
        },
        _ => {
            eprintln!("❌ Unknown target: {}", target);
        },
    }
}

