
use std::rc::{Weak};
use parser;

pub enum ListToken<'a> {
    Identifier { text: &'a str },
    StringLiteral { text: &'a str },
    NumericLiteral { value: f64 }
}

pub struct Node<'a> {
    children: Vec<Node<'a>>,
    data: Option<ListToken<'a>>
}

fn create_tree<'a>(tokens: &Vec<parser::PrimitiveToken>) -> Node<'a> {
    let root = Node { children: Vec::new(), data: None };
    root
}
