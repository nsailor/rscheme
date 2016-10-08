
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
    Lambda(Procedure),
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
                            "lambda" => Expression::process_lambda(&v[1..]),
                            "define" => Expression::process_define(&v[1..]),
                            _ => Expression::process_call(&v)
                        }
                    },
                    _ => Err("Expected identifier or keyword.".to_string())
                }
            },
            ListNode::Identifier(ref s) => {
                Ok(Expression::Identifier(s.to_string()))
            }
        }
    }

    fn process_define(params:&[ListNode]) -> Result<Expression,String> {
        if params.len() != 2 {
            Err("A definition statement needs exactly 2 arguments.".to_string())
        } else {
            if let ListNode::Identifier(ref s) = params[0] {
                match Expression::from_list(&params[1]) {
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
    }

    fn process_lambda(params:&[ListNode]) -> Result<Expression,String> {
        let mut args:Vec<String> = Vec::new();
        let arg_list = &params[0];
        if let ListNode::Node(ref v) = *arg_list {
            for a in v {
                if let ListNode::Identifier(ref s) = *a {
                    args.push(s.to_string());
                } else {
                    return Err("The argument list must only contain identifiers.".to_string());
                }
            }
        }
        let mut body:Vec<Expression> = Vec::new();
        for v in &params[1..] {
            match Expression::from_list(v) {
                Ok(exp) => body.push(exp),
                Err(s) => return Err(s)
            }
        }
        Ok(Expression::Lambda(Procedure::UserDefined { arguments:args, body:body }))
    }

    fn process_call(params:&[ListNode]) -> Result<Expression,String> {
        let mut args:Vec<Expression> = Vec::new();
        for node in &params[1..] {
            match Expression::from_list(node) {
                Ok(e) => args.push(e),
                Err(s) => return Err(s)
            }
        }
        let name:String;
        match params[0] {
            ListNode::Identifier(ref s) => name = s.to_string(),
            _ => return Err("Expected function name.".to_string())
        }
        Ok(Expression::Call { name:name, arguments:args })
    }
}
