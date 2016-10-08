
use list::*;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug,Clone)]
pub enum LValue {
    StringValue(String),
    NumericalValue(f64),
    BooleanValue(bool),
}

impl fmt::Display for LValue {
    fn fmt(&self, f:&mut Formatter) -> fmt::Result {
        match *self {
            LValue::StringValue(ref s) => write!(f, "\"{}\"", s),
            LValue::NumericalValue(v) => write!(f, "{}", v),
            LValue::BooleanValue(v) => if v { write!(f, "#t") } else { write!(f, "#f") }
        }
    }
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

impl fmt::Display for Procedure {
    fn fmt(&self, f:&mut Formatter) -> fmt::Result {
        match *self {
            Procedure::UserDefined { arguments:_, body:_ } => {
                 write!(f, "#procedure:user-defined")
            }
            Procedure::Sum => write!(f, "#procedure:+"),
            Procedure::Difference => write!(f, "#procedure:-"),
            Procedure::Product => write!(f, "#procedure:*"),
            Procedure::Division => write!(f, "#procedure:/")
        }
    }
}

#[derive(Debug,Clone)]
pub enum LResult {
    Value(LValue),
    Procedure(Procedure),
    Undefined
}

impl fmt::Display for LResult {
    fn fmt(&self, f:&mut Formatter) -> fmt::Result {
        match *self {
            LResult::Value(ref v) => write!(f, "{}", v),
            LResult::Procedure(ref p) => write!(f, "{}", p),
            LResult::Undefined => write!(f, "#undefined")
        }
    }
}

#[derive(Debug,Clone)]
pub enum Expression {
    Call { fun:Box<Expression>, arguments:Vec<Expression> },
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
                match v[0] {
                    ListNode::Identifier(ref s) => {
                        match s.as_str() {
                            "lambda" => Expression::process_lambda(&v[1..]),
                            "define" => Expression::process_define(&v[1..]),
                            _ => Expression::process_call(&v)
                        }
                    },
                    _ => Expression::process_call(&v)
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
        match Expression::from_list(&params[0]) {
            Ok(e) => Ok(Expression::Call { fun:Box::new(e), arguments:args }),
            Err(s) => Err(s)
        }
    }
}
