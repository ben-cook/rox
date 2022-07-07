mod expr;
mod interpreter;
mod lexer;
mod parser;
mod token;
mod token_type;
mod stmt;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser as ClapParser;

#[derive(ClapParser, Debug)]
#[clap(version)]
pub struct Args {
    pub file: Option<PathBuf>,
}

pub fn run(source: &str) -> Result<()> {
    let mut lexer = Lexer::new(&source);

    let tokens = lexer.scan_tokens();
    // dbg!(&lexer);

    let mut parser = Parser::new(tokens);

    match parser.parse() {
        Ok(expression) => {
            // dbg!(&expression);
            println!("{}", expression);
            let mut interpreter = Interpreter::new();
            interpreter.interpret(&expression)?;
        }
        Err(_) => println!(),
    }

    Ok(())
}
