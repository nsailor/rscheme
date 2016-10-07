
use std::fs::File;
use std::io::prelude::*;
mod parser;
mod list;
use list::*;

fn main() {
    let mut source_file = File::open("test-code.scm").unwrap();
    let mut code = String::new();
    source_file.read_to_string(&mut code).unwrap();
    let tokens = parser::parse_primitives(&mut code);
    let mut token_iter = tokens.into_iter();
    let list_tree = ListNode::from_primitive_tokens(&mut token_iter);
    println!("{:?}", list_tree);
}
