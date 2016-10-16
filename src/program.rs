
use std::collections::HashMap;
use parser;
use list::ListNode;
use expression::*;
use std::cmp::Ordering;

pub struct Program {
    stack: Vec<HashMap<String, LValue>>,
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
        let list_tree = ListNode::from_primitive_tokens(&mut token_iter, false);
        match list_tree {
            ListNode::Node(_, ref v) => {
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
        let mut basic_map: HashMap<String, LValue> = HashMap::new();
        basic_map.insert("+".to_string(), LValue::Procedure(Procedure::Sum));
        basic_map.insert("-".to_string(), LValue::Procedure(Procedure::Difference));
        basic_map.insert("*".to_string(), LValue::Procedure(Procedure::Product));
        basic_map.insert("/".to_string(), LValue::Procedure(Procedure::Division));
        basic_map.insert("=".to_string(), LValue::Procedure(Procedure::Equal));
        basic_map.insert("<".to_string(), LValue::Procedure(Procedure::Less));
        basic_map.insert(">".to_string(), LValue::Procedure(Procedure::Greater));
        basic_map.insert("and".to_string(), LValue::Procedure(Procedure::And));
        basic_map.insert("or".to_string(), LValue::Procedure(Procedure::Or));
        basic_map.insert("not".to_string(), LValue::Procedure(Procedure::Not));
        self.stack.push(basic_map);
    }

    fn find_identifier(&self, name: &str) -> Option<&LValue> {
        let stack_ref = &self.stack;
        for name_stack in stack_ref.into_iter().rev() {
            match name_stack.get(name) {
                Some(lres) => return Some(lres),
                None => continue,
            }
        }
        None
    }

    fn evaluate_call(&mut self, p: &Procedure, args: &Vec<LValue>) -> Result<LValue, String> {
        match *p {
            Procedure::Sum => {
                let mut sum: f64 = 0.0;
                for value in args {
                    if let &LValue::NumericalValue(ref x) = value {
                        sum += *x;
                    }
                }
                Ok(LValue::NumericalValue(sum))
            }
            Procedure::Product => {
                let mut product: f64 = 1.0;
                for value in args {
                    if let &LValue::NumericalValue(ref x) = value {
                        product *= *x;
                    }
                }
                Ok(LValue::NumericalValue(product))
            }
            Procedure::Difference => {
                let mut difference: f64 = 0.0;
                let mut index = 0;
                for value in args {
                    if let &LValue::NumericalValue(ref x) = value {
                        if index == 0 {
                            difference += *x;
                        } else {
                            difference -= *x;
                        }
                        index += 1;
                    }
                }
                if index == 1 {
                    // Only one number was given, find the additive inverse.
                    difference *= -1.0;
                }
                Ok(LValue::NumericalValue(difference))
            }
            Procedure::Division => {
                let mut ratio: f64 = 1.0;
                let mut index = 0;
                for value in args {
                    if let &LValue::NumericalValue(ref x) = value {
                        if index == 0 {
                            ratio *= *x;
                        } else {
                            ratio /= *x;
                        }
                        index += 1;
                    }
                }
                if index == 1 {
                    // Only one number was given, find the inverse.
                    ratio = 1.0 / ratio;
                }
                Ok(LValue::NumericalValue(ratio))
            }
            Procedure::Equal => {
                if args.len() != 2 {
                    return Err("Equality test needs two arguments.".to_string());
                }
                match args[0].compare(&args[1]) {
                    Ok(ord) => {
                        match ord {
                            Ordering::Equal => Ok(LValue::BooleanValue(true)),
                            _ => Ok(LValue::BooleanValue(false)),
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
                            Ordering::Less => Ok(LValue::BooleanValue(true)),
                            _ => Ok(LValue::BooleanValue(false)),
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
                            Ordering::Greater => Ok(LValue::BooleanValue(true)),
                            _ => Ok(LValue::BooleanValue(false)),
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
                Ok(LValue::BooleanValue(value))
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
                Ok(LValue::BooleanValue(value))
            }
            Procedure::Not => {
                if args.len() != 1 {
                    return Err("'Not' requires a single argument.".to_string());
                }
                match args[0].to_boolean() {
                    Ok(b) => Ok(LValue::BooleanValue(!b)),
                    Err(s) => return Err(s),
                }
            }
            Procedure::UserDefined { ref arguments, ref body } => {
                // Make sure that the arguments provided are enough.
                if arguments.len() != args.len() {
                    return Err("Invalid number of arguments provided.".to_string());
                }
                // Create an argument map.
                let mut arg_stack: HashMap<String, LValue> = HashMap::new();
                for i in 0..args.len() {
                    arg_stack.insert(arguments[i].clone(), args[i].clone());
                }
                self.stack.push(arg_stack);
                let mut lres: Result<LValue, String> = Err("Empty function body.".to_string());
                for e in body {
                    lres = self.evaluate_expression(e);
                }
                self.stack.pop();
                lres
            }
        }
    }

    fn evaluate_expression(&mut self, e: &Expression) -> Result<LValue, String> {
        match *e {
            Expression::Value(ref v) => Ok(v.clone()),
            Expression::List(ref children) => {
                if children.len() == 0 {
                    return Err("Can't evaluate an empty list.".to_string());
                }
                let p: Procedure;
                match self.evaluate_expression(&children[0]) {
                    Ok(res) => {
                        match res {
                            LValue::Procedure(pval) => p = pval,
                            _ => return Err("First element of list not a procedure.".to_string()),
                        }
                    }
                    Err(s) => return Err(s),
                }
                let mut args: Vec<LValue> = Vec::new();
                for arg in &children[1..] {
                    match self.evaluate_expression(arg) {
                        Ok(lres) => args.push(lres),
                        Err(s) => return Err(s),
                    }
                }
                self.evaluate_call(&p, &args)
            }
            Expression::Eval(ref exp) => {
                // Evaluate the argument.
                match self.evaluate_expression(exp) {
                    Ok(val) => {
                        if let LValue::Quoted(e) = val {
                            self.evaluate_expression(e.as_ref())
                        } else {
                            Ok(val)
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
                Ok(LValue::Undefined)
            }
            Expression::Identifier(ref s) => {
                let result = self.find_identifier(s).cloned();
                match result {
                    Some(lres) => Ok(lres),
                    None => Err("Undefined identifier '".to_string() + s + "'."),
                }
            }
            Expression::Lambda(ref p) => Ok(LValue::Procedure(p.clone())),
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
                        None => Ok(LValue::Undefined),
                    }
                }
            }
        }
    }
}
