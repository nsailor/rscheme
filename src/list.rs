
use parser::*;
use std::vec::*;

#[derive(Debug,Clone)]
pub enum ListNode {
    Node(bool, Vec<ListNode>),
    Identifier(bool, String),
    StringLiteral(String),
    NumericLiteral(f64),
    BooleanLiteral(bool),
}

impl ListNode {
    pub fn from_primitive_tokens(mut it: &mut IntoIter<PrimitiveToken>, quoted: bool) -> ListNode {
        let mut children: Vec<ListNode> = Vec::new();
        let mut quote_next = false;
        loop {
            let mut token_option = it.next();
            if token_option.is_none() {
                break;
            }
            let token = token_option.take().unwrap();
            match token {
                PrimitiveToken::Word(s) => {
                    match s.as_str() {
                        "#t" => {
                            children.push(ListNode::BooleanLiteral(true));
                            quote_next = false
                        }
                        "#f" => {
                            children.push(ListNode::BooleanLiteral(false));
                            quote_next = false
                        }
                        _ => children.push(ListNode::Identifier(quote_next, s)),
                    }
                }
                PrimitiveToken::StringLiteral(s) => children.push(ListNode::StringLiteral(s)),
                PrimitiveToken::NumericLiteral(v) => children.push(ListNode::NumericLiteral(v)),
                PrimitiveToken::LeftParen => {
                    children.push(ListNode::from_primitive_tokens(&mut it, quote_next));
                    quote_next = false;
                }
                PrimitiveToken::RightParen => break,
                PrimitiveToken::Quote => quote_next = true,
            }
        }
        ListNode::Node(quoted, children)
    }
}
