
mod parser;
mod list;
mod expression;
mod program;

use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::env;
use program::Program;

fn get_file_list() -> Vec<String> {
    let mut args = env::args();
    args.next(); // Skip the program name.
    let mut files:Vec<String> = Vec::new();
    for arg in args {
        files.push(arg.clone());
    }
    files
}

fn main() {
    println!("rScheme - A minimal Scheme intepreter written in Rust (v0.0.1 Alpha)");
    println!("Copyright (C) 2016 School of Engineering - Aristotle University of Thessaloniki");
    println!("---------------");
    println!("Type `exit` to quit the REPL environment. Statements can't span multiple lines.");
    println!("Loading the standard library...");

    let mut program = Program::new();
    let mut files = vec!["stdlib.scm".to_string()];
    files.append(&mut get_file_list());
    for path in files {
        match File::open(path.clone()) {
            Ok(mut file) => {
                let mut code = String::new();
                file.read_to_string(&mut code).unwrap();
                program.run_code(code, true);
            }
            Err(_) => println!("Failed to open file {}.", path)
        }
    }

    println!("---------------");
    loop {
        print!("]=> ");
        io::stdout().flush().unwrap();
        let mut accum = String::new();
        io::stdin().read_line(&mut accum).unwrap();
        if accum.trim() == "exit" {
            println!("Goodbye!");
            break;
        }
        program.run_code(accum, false);
        println!("");
    }
}
