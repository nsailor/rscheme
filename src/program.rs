
use std::collections::HashMap;
use list::ListNode;
use expression::*;

pub struct Program {
    stack:Vec<HashMap<String, LResult>>
}

impl Program {
    pub fn new() -> Program {
        Program { stack:Vec::new() }
    }

    pub fn run(&mut self, root: &ListNode) {
        // Add the basic functions.
        let mut basic_map:HashMap<String, LResult> = HashMap::new();
        basic_map.insert("+".to_string(), LResult::Procedure(Procedure::Sum));
        basic_map.insert("-".to_string(), LResult::Procedure(Procedure::Difference));
        basic_map.insert("*".to_string(), LResult::Procedure(Procedure::Product));
        basic_map.insert("/".to_string(), LResult::Procedure(Procedure::Division));
        self.stack.push(basic_map);
        match *root {
            ListNode::Node(ref v) => {
                for e in v {
                    match Expression::from_list(e) {
                        Ok(res) => {
                            match self.evaluate_expression(&res) {
                                Ok(result) => println!("Result: {:?}", result),
                                Err(s) => println!("Runtime error: {}", s)
                            }
                        }
                        Err(s) => {
                            println!("Syntax error: {}", s);
                            break
                        }
                    }
                }
            },
            _ => println!("Fatal error.")
        }
    }

    fn find_identifier(&self, name:&str) -> Option<&LResult> {
        for name_stack in &self.stack {
            match name_stack.get(name) {
                Some(lres) => return Option::Some(lres),
                None => continue
            }
        }
        Option::None
    }

    fn evaluate_call(&mut self, p:&Procedure, args:&Vec<LResult>) -> Result<LResult, String> {
        match *p {
            Procedure::Sum => {
                let mut sum:f64 = 0.0;
                for value in args {
                    if let &LResult::Value(ref v) = value {
                        if let &LValue::NumericalValue(ref x) = v {
                            sum += *x;
                        }
                    }
                }
                Ok(LResult::Value(LValue::NumericalValue(sum)))
            },
            Procedure::Product => {
                let mut product:f64 = 1.0;
                for value in args {
                    if let &LResult::Value(ref v) = value {
                        if let &LValue::NumericalValue(ref x) = v {
                            product *= *x;
                        }
                    }
                }
                Ok(LResult::Value(LValue::NumericalValue(product)))
            },
            Procedure::Difference => {
                let mut difference:f64 = 0.0;
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
            },
            Procedure::Division => {
                let mut ratio:f64 = 1.0;
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
            },
            Procedure::UserDefined { ref arguments, ref body } => {
                // Make sure that the arguments provided are enough.
                if arguments.len() != args.len() {
                    return Err("Invalid number of arguments provided.".to_string());
                }
                // Create an argument map.
                let mut arg_stack:HashMap<String,LResult> = HashMap::new();
                for i in 0..args.len() {
                    arg_stack.insert(arguments[i].clone(), args[i].clone());
                }
                self.stack.push(arg_stack);
                let mut lres:Result<LResult,String> = Err("Empty function body.".to_string());
                for e in body {
                    lres = self.evaluate_expression(e);
                }
                self.stack.pop();
                lres
            }
        }
    }

    fn evaluate_expression(&mut self, e:&Expression) -> Result<LResult, String> {
        match *e {
            Expression::Value(ref v) => Ok(LResult::Value(v.clone())),
            Expression::Call { ref name, arguments:ref args } => {
                let mut arg_values:Vec<LResult> = Vec::new();
                for e in args {
                    match self.evaluate_expression(e) {
                        Ok(lres) => arg_values.push(lres),
                        Err(s) => return Err(s)
                    }
                }
                let procedure = self.find_identifier(name).cloned();
                if procedure.is_none() {
                    return Err("Failed to find procedure '".to_string() + name + "'.");
                }
                let procedure = procedure.unwrap();
                match procedure {
                    LResult::Procedure(ref p) => self.evaluate_call(p, &arg_values),
                    _ => Err("Identifier not a procedure name in call statement.".to_string())
                }
            },
            Expression::Definition { ref name, ref value } => {
                // Check if the value exists anywhere but the last stack.
                if let Some((_, first)) = self.stack.split_last_mut() {
                    for name_stack in first {
                        if let Some(_) = name_stack.get(name) {
                            return Err("Local variable definition would shadow a global name.".to_string());
                        }
                    }
                } else {
                    return Err("Unknown error occured - empty variable stack.".to_string());
                }
                let lres;
                match self.evaluate_expression(value) {
                    Ok(result) => lres = result.clone(),
                    Err(s) => return Err(s)
                }
                *self.stack.last_mut().unwrap().entry(name.to_string()).or_insert(LResult::Undefined) = lres;
                Ok(LResult::Undefined)
            },
            Expression::Identifier(ref s) => {
                let result = self.find_identifier(s).cloned();
                match result {
                    Some(lres) => Ok(lres),
                    None => Err("Undefined identifier '".to_string() + s + "'.")
                }
            },
            Expression::Lambda(ref p) => {
                Ok(LResult::Procedure(p.clone()))
            }
        }
    }
}
