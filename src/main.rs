mod lexer;
mod parser;
mod ast;
mod vm;
mod types;
mod diagnostics;
mod type_checker;
mod compiler;
mod ownership;
mod stdlib;

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
        /// Target platform (native, wasm32, evm, wasm-solana, ios, android, cosmos, polkadot)
        #[arg(short, long, default_value = "native")]
        target: String,
        /// Enable optimizations
        #[arg(short, long)]
        optimize: bool,
        /// Enable debug symbols
        #[arg(short, long)]
        debug: bool,
        /// Target architecture (x86_64, arm64, riscv64)
        #[arg(long)]
        arch: Option<String>,
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
        Commands::Build { file, target, optimize, debug, arch } => {
            build_file(&file, &target, optimize, debug, arch);
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
        
        // Tokenize
        let mut lexer = lexer::Lexer::new(&source);
        let tokens = lexer.tokenize();
        
        // Parse
        let mut parser = parser::Parser::new(tokens);
        let program = parser.parse();
        
        // Type check
        println!("🔍 Type checking...");
        let mut checker = type_checker::TypeChecker::new(&source);
        if !checker.check(&program) {
            println!("\n");
            checker.print_diagnostics();
            eprintln!("\n❌ Type checking failed. Execution aborted.");
            return;
        }
        println!("✅ Type checking passed\n");
        
        // For now, still use VM for compiled mode (until LLVM backend is ready)
        let mut vm = vm::VM::new();
        if let Err(e) = vm.interpret(&program) {
            eprintln!("❌ Runtime error: {}", e);
        }
    } else {
        eprintln!("❌ Error: missing [Compiled] or [Interpreted] header");
        eprintln!("   Add [Interpreted] or [Compiled] at the top of your file");
    }
}

fn build_file(file: &PathBuf, target: &str, optimize: bool, debug: bool, arch: Option<String>) {
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
    
    // Type check (for compiled mode)
    if program.mode == ast::ExecutionMode::Compiled {
        println!("🔍 Type checking...");
        let mut checker = type_checker::TypeChecker::new(&source);
        if !checker.check(&program) {
            println!("\n");
            checker.print_diagnostics();
            eprintln!("\n❌ Type checking failed. Build aborted.");
            return;
        }
        println!("✅ Type checking passed");
        
        // Ownership checking
        println!("🔒 Ownership checking...");
        let mut borrow_checker = ownership::BorrowChecker::new(&source);
        if !borrow_checker.check(&program) {
            println!("\n");
            borrow_checker.print_diagnostics();
            eprintln!("\n❌ Ownership checking failed. Build aborted.");
            return;
        }
        println!("✅ Ownership checking passed\n");
    }
    
    // Determine output path
    let output_path = file.with_extension(match target {
        "native" => "ll",
        "wasm32" => "wasm",
        "evm" => "evm",
        "wasm-solana" => "so",
        "ios" => "ll",
        "android" => "ll",
        "cosmos" => "wasm",
        "polkadot" => "wasm",
        _ => {
            eprintln!("❌ Unknown target: {}", target);
            return;
        },
    });
    
    // Compile
    let compilation_target = match target {
        "native" => compiler::CompilationTarget::Native,
        "wasm32" => compiler::CompilationTarget::Wasm32,
        "evm" => compiler::CompilationTarget::Evm,
        "wasm-solana" => compiler::CompilationTarget::Solana,
        "ios" => compiler::CompilationTarget::Ios,
        "android" => compiler::CompilationTarget::Android,
        "cosmos" => compiler::CompilationTarget::Cosmos,
        "polkadot" => compiler::CompilationTarget::Polkadot,
        _ => unreachable!(),
    };
    
    // Create compiler with options
    let options = compiler::CompilerOptions {
        optimize,
        debug,
        arch,
    };
    let comp = compiler::Compiler::with_options(compilation_target, options);
    match comp.compile(&program, &output_path) {
        Ok(_) => {
            println!("✅ Compilation successful!");
            println!("   Output: {}", output_path.display());
        },
        Err(e) => {
            eprintln!("❌ Compilation failed: {}", e);
        },
    }
}

