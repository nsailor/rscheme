
use std::collections::HashMap;
use parser;
use list::ListNode;
use expression::*;
use std::cmp::Ordering;

pub struct Program {
    stack: Vec<HashMap<String, LResult>>,
}

impl Program {
    pub fn new() -> Program {
        let mut p = Program { stack: Vec::new() };
        p.initialize();
        p
    }

    pub fn run_code(&mut self, code: String, silent: bool) {
        let tokens = parser::parse_primitives(&code);
        let mut token_iter = tokens.into_iter();
        let list_tree = ListNode::from_primitive_tokens(&mut token_iter);
        match list_tree {
            ListNode::Node(ref v) => {
                for e in v {
                    match Expression::from_list(e) {
                        Ok(res) => {
                            match self.evaluate_expression(&res) {
                                Ok(result) => {
                                    if !silent {
                                        println!("{}", result)
                                    }
                                }
                                Err(s) => println!("Runtime error: {}", s),
                            }
                        }
                        Err(s) => {
                            println!("Syntax error: {}", s);
                            break;
                        }
                    }
                }
            }
            _ => println!("Fatal error."),
        }
    }

    pub fn initialize(&mut self) {
        // Add the basic functions.
        let mut basic_map: HashMap<String, LResult> = HashMap::new();
        basic_map.insert("+".to_string(), LResult::Procedure(Procedure::Sum));
        basic_map.insert("-".to_string(), LResult::Procedure(Procedure::Difference));
        basic_map.insert("*".to_string(), LResult::Procedure(Procedure::Product));
        basic_map.insert("/".to_string(), LResult::Procedure(Procedure::Division));
        basic_map.insert("=".to_string(), LResult::Procedure(Procedure::Equal));
        basic_map.insert("<".to_string(), LResult::Procedure(Procedure::Less));
        basic_map.insert(">".to_string(), LResult::Procedure(Procedure::Greater));
        basic_map.insert("and".to_string(), LResult::Procedure(Procedure::And));
        basic_map.insert("or".to_string(), LResult::Procedure(Procedure::Or));
        basic_map.insert("not".to_string(), LResult::Procedure(Procedure::Not));
        self.stack.push(basic_map);
    }

    fn find_identifier(&self, name: &str) -> Option<&LResult> {
        let stack_ref = &self.stack;
        for name_stack in stack_ref.into_iter().rev() {
            match name_stack.get(name) {
                Some(lres) => return Some(lres),
                None => continue,
            }
        }
        None
    }

    fn evaluate_call(&mut self, p: &Procedure, args: &Vec<LResult>) -> Result<LResult, String> {
        match *p {
            Procedure::Sum => {
                let mut sum: f64 = 0.0;
                for value in args {
                    if let &LResult::Value(ref v) = value {
                        if let &LValue::NumericalValue(ref x) = v {
                            sum += *x;
                        }
                    }
                }
                Ok(LResult::Value(LValue::NumericalValue(sum)))
            }
            Procedure::Product => {
                let mut product: f64 = 1.0;
                for value in args {
                    if let &LResult::Value(ref v) = value {
                        if let &LValue::NumericalValue(ref x) = v {
                            product *= *x;
                        }
                    }
                }
                Ok(LResult::Value(LValue::NumericalValue(product)))
            }
            Procedure::Difference => {
                let mut difference: f64 = 0.0;
                let mut index = 0;
                for value in args {
                    if let &LResult::Value(ref v) = value {
                        if let &LValue::NumericalValue(ref x) = v {
                            if index == 0 {
                                difference += *x;
                            } else {
                                difference -= *x;
                            }
                            index += 1;
                        }
                    }
                }
                if index == 1 {
                    // Only one number was given, find the additive inverse.
                    difference *= -1.0;
                }
                Ok(LResult::Value(LValue::NumericalValue(difference)))
            }
            Procedure::Division => {
                let mut ratio: f64 = 1.0;
                let mut index = 0;
                for value in args {
                    if let &LResult::Value(ref v) = value {
                        if let &LValue::NumericalValue(ref x) = v {
                            if index == 0 {
                                ratio *= *x;
                            } else {
                                ratio /= *x;
                            }
                            index += 1;
                        }
                    }
                }
                if index == 1 {
                    // Only one number was given, find the inverse.
                    ratio = 1.0 / ratio;
                }
                Ok(LResult::Value(LValue::NumericalValue(ratio)))
            }
            Procedure::Equal => {
                if args.len() != 2 {
                    return Err("Equality test needs two arguments.".to_string());
                }
                match args[0].compare(&args[1]) {
                    Ok(ord) => {
                        match ord {
                            Ordering::Equal => Ok(LResult::Value(LValue::BooleanValue(true))),
                            _ => Ok(LResult::Value(LValue::BooleanValue(false))),
                        }
                    }
                    Err(s) => Err(s),
                }
            }
            Procedure::Less => {
                if args.len() != 2 {
                    return Err("Comparison test needs two arguments.".to_string());
                }
                match args[0].compare(&args[1]) {
                    Ok(ord) => {
                        match ord {
                            Ordering::Less => Ok(LResult::Value(LValue::BooleanValue(true))),
                            _ => Ok(LResult::Value(LValue::BooleanValue(false))),
                        }
                    }
                    Err(s) => Err(s),
                }
            }
            Procedure::Greater => {
                if args.len() != 2 {
                    return Err("Comparison test needs two arguments.".to_string());
                }
                match args[0].compare(&args[1]) {
                    Ok(ord) => {
                        match ord {
                            Ordering::Greater => Ok(LResult::Value(LValue::BooleanValue(true))),
                            _ => Ok(LResult::Value(LValue::BooleanValue(false))),
                        }
                    }
                    Err(s) => Err(s),
                }
            }
            Procedure::And => {
                if args.len() < 2 {
                    return Err("'And' requires at least two operands.".to_string());
                }
                let mut value = true;
                for v in args {
                    match v.to_boolean() {
                        Ok(b) => value = value && b,
                        Err(s) => return Err(s),
                    }
                }
                Ok(LResult::Value(LValue::BooleanValue(value)))
            }
            Procedure::Or => {
                if args.len() < 2 {
                    return Err("'Or' requires at least two operands.".to_string());
                }
                let mut value = false;
                for v in args {
                    match v.to_boolean() {
                        Ok(b) => value = value || b,
                        Err(s) => return Err(s),
                    }
                }
                Ok(LResult::Value(LValue::BooleanValue(value)))
            }
            Procedure::Not => {
                if args.len() != 1 {
                    return Err("'Not' requires a single argument.".to_string());
                }
                match args[0].to_boolean() {
                    Ok(b) => Ok(LResult::Value(LValue::BooleanValue(!b))),
                    Err(s) => return Err(s),
                }
            }
            Procedure::UserDefined { ref arguments, ref body } => {
                // Make sure that the arguments provided are enough.
                if arguments.len() != args.len() {
                    return Err("Invalid number of arguments provided.".to_string());
                }
                // Create an argument map.
                let mut arg_stack: HashMap<String, LResult> = HashMap::new();
                for i in 0..args.len() {
                    arg_stack.insert(arguments[i].clone(), args[i].clone());
                }
                self.stack.push(arg_stack);
                let mut lres: Result<LResult, String> = Err("Empty function body.".to_string());
                for e in body {
                    lres = self.evaluate_expression(e);
                }
                self.stack.pop();
                lres
            }
        }
    }

    fn evaluate_expression(&mut self, e: &Expression) -> Result<LResult, String> {
        match *e {
            Expression::Value(ref v) => Ok(LResult::Value(v.clone())),
            Expression::Call { ref fun, arguments: ref args } => {
                let mut arg_values: Vec<LResult> = Vec::new();
                for e in args {
                    match self.evaluate_expression(e) {
                        Ok(lres) => arg_values.push(lres),
                        Err(s) => return Err(s),
                    }
                }
                let procedure = self.evaluate_expression(fun).clone();
                match procedure {
                    Ok(p) => {
                        match p {
                            LResult::Procedure(ref p) => self.evaluate_call(p, &arg_values),
                            _ => Err("First expression not a procedure.".to_string()),
                        }
                    }
                    Err(s) => Err(s),
                }
            }
            Expression::Definition { ref name, ref value } => {
                let lres;
                match self.evaluate_expression(value) {
                    Ok(result) => lres = result.clone(),
                    Err(s) => return Err(s),
                }
                let ref mut active_stack = *self.stack
                    .last_mut()
                    .unwrap();
                active_stack.insert(name.to_string(), lres.clone());
                Ok(LResult::Undefined)
            }
            Expression::Identifier(ref s) => {
                let result = self.find_identifier(s).cloned();
                match result {
                    Some(lres) => Ok(lres),
                    None => Err("Undefined identifier '".to_string() + s + "'."),
                }
            }
            Expression::Lambda(ref p) => Ok(LResult::Procedure(p.clone())),
            Expression::IfCondition { ref cond, ref yes_expr, ref no_expr } => {
                let result: bool;
                match self.evaluate_expression(cond) {
                    Ok(lres) => {
                        match lres.to_boolean() {
                            Ok(b) => {
                                result = b;
                            }
                            Err(s) => return Err(s),
                        }
                    }
                    Err(s) => return Err(s),
                }
                if result == true {
                    self.evaluate_expression(yes_expr)
                } else {
                    match *no_expr {
                        Some(ref no_e) => self.evaluate_expression(no_e),
                        None => Ok(LResult::Undefined),
                    }
                }
            }
        }
    }
}
