use std::io::{self, BufRead};
use std::{fs::read_to_string, io::Write};

use anyhow::Result;
use clap::Parser;
use glad::{run, Args};

fn main() -> Result<()> {
    let args = Args::parse();

    match args.file {
        Some(file_path) => {
            let file_contents = read_to_string(file_path)?;
            run(&file_contents)?;
        }
        None => {
            let mut input = String::new();
            let mut stdin = io::stdin().lock();
            let mut stdout = io::stdout();

            loop {
                print!("> ");
                stdout.flush()?;

                input.clear();
                match stdin.read_line(&mut input) {
                    Ok(bytes_read) => {
                        if bytes_read == 0 {
                            std::process::exit(0);
                        } else {
                            run(&input)?
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        std::process::exit(70);
                    }
                }
            }
        }
    }

    Ok(())
}
