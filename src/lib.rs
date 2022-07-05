mod lexer;
mod token;
mod token_type;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use lexer::Lexer;

#[derive(Parser, Debug)]
#[clap(version)]
pub struct Args {
    pub file: Option<PathBuf>,
}

pub fn run(source: String) -> Result<()> {
    let mut lexer = Lexer::new(&source);

    lexer.scan_tokens();
    dbg!(&lexer);

    Ok(())
}
