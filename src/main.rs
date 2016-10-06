
use std::fs::File;
use std::io::prelude::*;
mod parser;

fn main() {
    let mut source_file = File::open("test-code.scm").unwrap();
    let mut code = String::new();
    source_file.read_to_string(&mut code).unwrap();
    let tokens = parser::parse_primitives(&mut code);
    println!("Tokens: {:?}", tokens);
}
