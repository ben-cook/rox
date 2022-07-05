use std::io;
use std::{fs::read_to_string, io::Write};

use anyhow::Result;
use clap::Parser;
use glad::{run, Args};

fn main() -> Result<()> {
    let args = Args::parse();

    match args.file {
        Some(file_path) => {
            let file_contents = read_to_string(file_path)?;
            run(file_contents)?;
        }
        None => {
            let stdin = io::stdin();
            let mut stdout = io::stdout();

            'repl: loop {
                print!("> ");
                stdout.flush()?;

                let mut input = String::new();
                match stdin.read_line(&mut input) {
                    Ok(_) => run(input)?,
                    Err(e) => {
                        println!("Error: {}", e);
                        break 'repl;
                    }
                }
            }
        }
    }

    Ok(())
}
