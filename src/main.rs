use clap::Parser;
use std::fs;
use std::path::PathBuf;

mod utils;
mod lexer;
mod parser;
mod compile;

#[derive(Parser)]
#[command(name = "crabby")]
#[command(about = "Crabby programming language compiler")]
struct Cli {
    #[arg(help = "Input .crab or .cb file")]
    input: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if !cli.input.exists() {
        return Err("Input file does not exist".into());
    }

    let ext = cli.input.extension().unwrap_or_default();
    if ext != "crab" && ext != "cb" {
        return Err("Input file must have .crab or .cb extension".into());
    }

    let source = fs::read_to_string(&cli.input)?;

    // Lexical analysis
    let tokens = lexer::tokenize(&source)?;

    // Parsing
    let ast = parser::parse(tokens)?;

    // Compilation
    compile::compile(&ast)?;

    Ok(())
}
