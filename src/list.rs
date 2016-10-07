
use parser::*;
use std::vec::*;

#[derive(Debug)]
pub enum ListNode {
    Node(Vec<Box<ListNode>>),
    Identifier(String),
    StringLiteral(String),
    NumericLiteral(f64)
}

impl ListNode {
    pub fn from_primitive_tokens(mut it:&mut IntoIter<PrimitiveToken>) -> ListNode {
        let mut children:Vec<Box<ListNode>> = Vec::new();
        loop {
            let mut token_option = it.next();
            if token_option.is_none() {
                break;
            }
            let token = token_option.take().unwrap();
            match token {
                PrimitiveToken::Word(s) => children.push(Box::new(ListNode::Identifier(s))),
                PrimitiveToken::StringLiteral(s) =>
                    children.push(Box::new(ListNode::StringLiteral(s))),
                PrimitiveToken::NumericLiteral(v) =>
                    children.push(Box::new(ListNode::NumericLiteral(v))),
                PrimitiveToken::LeftParen => {
                    children.push(Box::new(ListNode::from_primitive_tokens(&mut it)));
                },
                PrimitiveToken::RightParen => {
                    break
                }
            }
        }
        ListNode::Node(children)
    }
}
