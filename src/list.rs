
use parser::*;
use std::vec::*;

#[derive(Debug)]
pub enum ListNode {
    Node(Vec<ListNode>),
    Identifier(String),
    StringLiteral(String),
    NumericLiteral(f64),
    BooleanLiteral(bool)
}

impl ListNode {
    pub fn from_primitive_tokens(mut it:&mut IntoIter<PrimitiveToken>) -> ListNode {
        let mut children:Vec<ListNode> = Vec::new();
        loop {
            let mut token_option = it.next();
            if token_option.is_none() {
                break;
            }
            let token = token_option.take().unwrap();
            match token {
                PrimitiveToken::Word(s) => {
                    match s.as_str() {
                        "#t" => children.push(ListNode::BooleanLiteral(true)),
                        "#f" => children.push(ListNode::BooleanLiteral(false)),
                        _ => children.push(ListNode::Identifier(s))
                    }
                },
                PrimitiveToken::StringLiteral(s) =>
                    children.push(ListNode::StringLiteral(s)),
                PrimitiveToken::NumericLiteral(v) =>
                    children.push(ListNode::NumericLiteral(v)),
                PrimitiveToken::LeftParen => {
                    children.push(ListNode::from_primitive_tokens(&mut it));
                },
                PrimitiveToken::RightParen => {
                    break
                }
            }
        }
        ListNode::Node(children)
    }
}
