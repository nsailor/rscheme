
use list::*;
use std::fmt;
use std::fmt::Formatter;
use std::cmp::Ordering;

#[derive(Debug,Clone)]
pub enum LValue {
    StringValue(String),
    NumericalValue(f64),
    BooleanValue(bool),
}

impl fmt::Display for LValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            LValue::StringValue(ref s) => write!(f, "\"{}\"", s),
            LValue::NumericalValue(v) => write!(f, "{}", v),
            LValue::BooleanValue(v) => if v { write!(f, "#t") } else { write!(f, "#f") },
        }
    }
}

#[derive(Debug,Clone)]
pub enum Procedure {
    UserDefined {
        arguments: Vec<String>,
        body: Vec<Expression>,
    },
    Sum,
    Difference,
    Product,
    Division,
    Equal,
    Less,
    Greater,
    And,
    Or,
    Not,
}

impl fmt::Display for Procedure {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Procedure::UserDefined { arguments: _, body: _ } => {
                write!(f, "#<procedure>:user-defined")
            }
            Procedure::Sum => write!(f, "#<procedure>:+"),
            Procedure::Difference => write!(f, "#<procedure>:-"),
            Procedure::Product => write!(f, "#<procedure>:*"),
            Procedure::Division => write!(f, "#<procedure>:/"),
            Procedure::Equal => write!(f, "#<procedure>:="),
            Procedure::Less => write!(f, "#<procedure>:<"),
            Procedure::Greater => write!(f, "#<procedure>:>"),
            Procedure::And => write!(f, "#<procedure>:and"),
            Procedure::Or => write!(f, "#<procedure>:or"),
            Procedure::Not => write!(f, "#<procedure>:not"),
        }
    }
}

#[derive(Debug,Clone)]
pub enum LResult {
    Value(LValue),
    Procedure(Procedure),
    Undefined,
}

impl fmt::Display for LResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            LResult::Value(ref v) => write!(f, "{}", v),
            LResult::Procedure(ref p) => write!(f, "{}", p),
            LResult::Undefined => write!(f, "#<undefined>"),
        }
    }
}

impl LResult {
    pub fn compare(&self, v: &LResult) -> Result<Ordering, String> {
        let lhs: LValue;
        let rhs: LValue;
        match *self {
            LResult::Value(ref lv) => lhs = lv.clone(),
            _ => return Err("Can't compare procedures or undefined results.".to_string()),
        }
        match *v {
            LResult::Value(ref lv) => rhs = lv.clone(),
            _ => return Err("Can't compare procedures or undefined results.".to_string()),
        }
        match lhs {
            LValue::StringValue(ref s1) => {
                match rhs {
                    LValue::StringValue(ref s2) => Ok(s1.cmp(s2)),
                    _ => Err("Expected string expression as the second argument.".to_string()),
                }
            }
            LValue::BooleanValue(b1) => {
                match rhs {
                    LValue::BooleanValue(b2) => Ok(b1.cmp(&b2)),
                    _ => Err("Expected boolean expression as the second argument.".to_string()),
                }
            }
            LValue::NumericalValue(x1) => {
                match rhs {
                    LValue::NumericalValue(x2) => {
                        Ok(if x1 > x2 {
                            Ordering::Greater
                        } else if x1 == x2 {
                            Ordering::Equal
                        } else {
                            Ordering::Less
                        })
                    }
                    _ => Err("Expected numerical expression as the second argument.".to_string()),
                }
            }
        }
    }

    pub fn to_boolean(&self) -> Result<bool, String> {
        match *self {
            LResult::Value(ref lv) => {
                match *lv {
                    LValue::NumericalValue(x) => Ok(x >= 0.0),
                    LValue::BooleanValue(b) => Ok(b),
                    LValue::StringValue(_) => Err("Can't convert string to boolean.".to_string()),
                }
            }
            _ => Err("Can't convert procedures and #undefined's to booleans.".to_string()),
        }
    }
}

#[derive(Debug,Clone)]
pub enum Expression {
    Call {
        fun: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Definition {
        name: String,
        value: Box<Expression>,
    },
    Lambda(Procedure),
    Identifier(String),
    Value(LValue),
    IfCondition {
        cond: Box<Expression>,
        yes_expr: Box<Expression>,
        no_expr: Option<Box<Expression>>,
    },
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
                            "if" => Expression::process_if(&v[1..]),
                            _ => Expression::process_call(&v),
                        }
                    }
                    _ => Expression::process_call(&v),
                }
            }
            ListNode::Identifier(ref s) => Ok(Expression::Identifier(s.to_string())),
        }
    }

    fn process_define(params: &[ListNode]) -> Result<Expression, String> {
        if params.len() != 2 {
            Err("A definition statement needs exactly 2 arguments.".to_string())
        } else {
            if let ListNode::Identifier(ref s) = params[0] {
                match Expression::from_list(&params[1]) {
                    Ok(e) => {
                        Ok(Expression::Definition {
                            name: s.to_string(),
                            value: Box::new(e),
                        })
                    }
                    Err(s) => Err(s),
                }
            } else {
                Err("First argument must be a valid identifier.".to_string())
            }
        }
    }

    fn process_lambda(params: &[ListNode]) -> Result<Expression, String> {
        let mut args: Vec<String> = Vec::new();
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
        let mut body: Vec<Expression> = Vec::new();
        for v in &params[1..] {
            match Expression::from_list(v) {
                Ok(exp) => body.push(exp),
                Err(s) => return Err(s),
            }
        }
        Ok(Expression::Lambda(Procedure::UserDefined {
            arguments: args,
            body: body,
        }))
    }

    fn process_if(params: &[ListNode]) -> Result<Expression, String> {
        if params.len() < 2 || params.len() > 3 {
            return Err("'if' statement requires two or three expressions.".to_string());
        }
        let condition: Box<Expression>;
        let yes_expr: Box<Expression>;
        let no_expr: Option<Box<Expression>>;
        match Expression::from_list(&params[0]) {
            Ok(ref e) => condition = Box::new(e.clone()),
            Err(s) => return Err(s),
        }
        match Expression::from_list(&params[1]) {
            Ok(ref e) => yes_expr = Box::new(e.clone()),
            Err(s) => return Err(s),
        }
        if params.len() < 3 {
            no_expr = Option::None;
        } else {
            match Expression::from_list(&params[2]) {
                Ok(ref e) => no_expr = Some(Box::new(e.clone())),
                Err(s) => return Err(s),
            }
        }
        Ok(Expression::IfCondition {
            cond: condition,
            yes_expr: yes_expr,
            no_expr: no_expr,
        })
    }

    fn process_call(params: &[ListNode]) -> Result<Expression, String> {
        let mut args: Vec<Expression> = Vec::new();
        for node in &params[1..] {
            match Expression::from_list(node) {
                Ok(e) => args.push(e),
                Err(s) => return Err(s),
            }
        }
        match Expression::from_list(&params[0]) {
            Ok(e) => {
                Ok(Expression::Call {
                    fun: Box::new(e),
                    arguments: args,
                })
            }
            Err(s) => Err(s),
        }
    }
}
