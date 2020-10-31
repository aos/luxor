use std::io::{self, Write};
use std::{fs, process};

mod ast;
mod scanner;
mod token;

use scanner::Scanner;

fn main() {
    let mut args = std::env::args();
    let mut luxor = Luxor::new();

    if args.len() > 2 {
        println!("Usage: ./luxor [script]");
        process::exit(64);
    } else if args.len() == 2 {
        let filename = args.next().unwrap();
        luxor.run_file(&filename).unwrap();
    } else {
        luxor.run_prompt().unwrap();
    }
}

#[derive(Default)]
struct Luxor {
    had_error: bool,
}

impl Luxor {
    fn new() -> Self {
        Default::default()
    }

    fn run_file(&mut self, f: &str) -> Result<(), io::Error> {
        let src = fs::read_to_string(f)?;
        self.run(&src);
        Ok(())
    }

    fn run_prompt(&mut self) -> Result<(), io::Error> {
        let mut input = String::new();

        loop {
            print!("> ");
            let _ = io::stdout().flush();

            match io::stdin().read_line(&mut input) {
                Ok(n) => {
                    if n == 0 {
                        // EOF
                        return Ok(());
                    } else {
                        self.run(&input.trim());
                    };
                }
                Err(e) => return Err(e),
            }

            input.clear();
        }
    }

    // Scanner here
    fn run(&mut self, src: &str) {
        let mut sc = Scanner::new(src);
        let tokens = sc.scan_tokens();

        for t in tokens {
            if t.is_error() {
                if let token::TokenType::Error(err) = &t.kind {
                    self.error(t.line, &err);
                }
            } else {
                println!("{:?}", t);
            }
        }
    }

    pub fn error(&mut self, line: u32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: u32, whe: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, whe, message);
        self.had_error = true;
    }
}
