
use list::*;

#[derive(Debug,Clone)]
pub enum LValue {
    StringValue(String),
    NumericalValue(f64),
    BooleanValue(bool),
}

#[derive(Debug,Clone)]
pub enum Procedure {
    UserDefined {
        arguments:Vec<String>,
        body:Vec<Expression>
    },
    Sum,
    Difference,
    Product,
    Division
}

#[derive(Debug,Clone)]
pub enum LResult {
    Value(LValue),
    Procedure(Procedure),
    Undefined
}

#[derive(Debug,Clone)]
pub enum Expression {
    Call { name:String, arguments:Vec<Expression> },
    Definition { name:String, value:Box<Expression> },
    Identifier(String),
    Value(LValue)
}

impl Expression {
    pub fn from_list(l: &ListNode) -> Result<Expression, String> {
        match *l {
            ListNode::StringLiteral(ref s) => Ok(Expression::Value(LValue::StringValue(s.clone()))),
            ListNode::BooleanLiteral(b) => Ok(Expression::Value(LValue::BooleanValue(b))),
            ListNode::NumericLiteral(v) => Ok(Expression::Value(LValue::NumericalValue(v))),
            ListNode::Node(ref v) => {
                // Check if this is a "define" or a lambda expression.
                match v[0] {
                    ListNode::Identifier(ref s) => {
                        match s.as_str() {
                            "define" => {
                                if v.len() != 3 {
                                    Err("A definition statement needs exactly 2 arguments.".to_string())
                                } else {
                                    if let ListNode::Identifier(ref s) = v[1] {
                                        match Expression::from_list(&v[2]) {
                                            Ok(e) => {
                                                Ok(Expression::Definition { name:s.to_string(),
                                                     value:Box::new(e) })
                                            }
                                            Err(s) => Err(s)
                                        }
                                    } else {
                                        Err("First argument must be a valid identifier.".to_string())
                                    }
                                }
                            },
                            "lambda" => Err("Lambda expressions not supported yet.".to_string()),
                            _ => {
                                // Function call.
                                let mut args:Vec<Expression> = Vec::new();
                                if let Some((_, rest)) = v.split_first() {
                                    for node in rest {
                                        match Expression::from_list(node) {
                                            Ok(e) => args.push(e),
                                            Err(s) => return Err(s) // Forward the error.
                                        }
                                    }
                                }
                                Ok(Expression::Call { name:s.clone(), arguments:args })
                            }
                        }
                    },
                    // Todo: Could also be a lambda expression.
                    _ => Err("Expected identifier or keyword.".to_string())
                }
            },
            ListNode::Identifier(ref s) => {
                Ok(Expression::Identifier(s.to_string()))
            }
        }
    }
}
