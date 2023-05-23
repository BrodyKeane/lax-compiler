#![allow(unused_parens)]

use std::{
    io::{self, Write},
    env,
    process,
    fs, 
};

use scanner::Scanner;
use error::ErrorStatus;
use interpreter::Interpreter;
use ast::parser::Parser;

mod scanner;
mod token;
mod ast;
mod interpreter;
mod error;
mod environment;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut session = Lax::new();
    match args.len() {
        0 => session.run_prompt(),
        1 => session.run_file(&args[0]),
        num_args => {
            eprintln!("Expected 1 argument but {} were given", num_args);
            process::exit(64);
        },
    };
    
}

struct Lax {
    interpreter: Interpreter,
    status: ErrorStatus,
}

impl Lax {
    pub fn new() -> Self {
        Lax {
            interpreter: Interpreter::new(),
            status: ErrorStatus::new() 
        }
    }
    pub fn run_file(&mut self, path: &String) {
        let source = fs::read_to_string(path).unwrap();
        self.run(source);
        if self.status.had_compile_error {process::exit(65)}
        if self.status.had_runtime_error {process::exit(70)}
    }

    pub fn run_prompt(&mut self) {
        let mut status = ErrorStatus::new();
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            if let Err(error) = io::stdin().read_line(&mut line) {
                eprintln!("Error reading line: {:?}", error);
            }
            line = line.trim().to_string();
            
            self.run(line);
            status.had_compile_error = false;
        }
    }

    fn run(&mut self, source: String) {
        let mut  scanner = Scanner::new(&mut self.status, source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(&mut self.status, tokens);
        let stmts = parser.parse();

        if self.status.had_compile_error {return};

        if let Err(error) = self.interpreter.interpret(&stmts) {
             self.status.report_runtime_error(error); 
        }
    }
}




