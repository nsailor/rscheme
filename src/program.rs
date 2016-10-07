
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
        println!("Running program: {:?}", root);
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
            _ => Err("Can't evaluate user-defined functions yet.".to_string())
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
                    return Err("Failed to find the requested function.".to_string());
                }
                let procedure = procedure.unwrap();
                match procedure {
                    LResult::Procedure(ref p) => self.evaluate_call(p, &arg_values),
                    _ => Err("Identifier not a function.".to_string())
                }
            }
        }
    }
}
