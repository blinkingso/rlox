use std::env;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::process::exit;

use crate::error::LoxError;
use crate::scanner::Scanner;

pub struct Lox {
    had_error: bool,
}
impl Default for Lox {
    fn default() -> Self {
        Lox { had_error: false }
    }
}

impl Lox {
    pub fn parse(&mut self) -> io::Result<()> {
        let args = env::args();
        if args.len() > 2 {
            eprintln!("Usage: jlox [script]");
            exit(64);
        } else if args.len() == 2 {
            self.run_file(args.last().unwrap())?;
            if self.had_error {
                exit(65);
            }
        } else {
            self.run_prompt()?;
        }
        Ok(())
    }

    fn run_file(&mut self, path: impl AsRef<str>) -> io::Result<()> {
        let buf = std::fs::read_to_string(path.as_ref())?;
        if let Err(err) = self.run(buf) {
            err.report("".to_string());
            self.had_error = true;
        }
        Ok(())
    }

    fn run(&mut self, source: String) -> Result<(), LoxError> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;
        for token in tokens.iter() {
            println!("{}", token);
        }

        Ok(())
    }

    fn run_prompt(&mut self) -> io::Result<()> {
        print!("> ");
        io::stdout().flush()?;
        for line in io::stdin().lock().lines() {
            if let Ok(line) = line {
                if line.is_empty() {
                    break;
                }
                if let Err(err) = self.run(line) {
                    self.had_error = false;
                    err.report("".to_string());
                }
            } else {
                break;
            }
        }
        Ok(())
    }
}
