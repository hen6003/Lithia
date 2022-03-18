use std::collections::HashMap;
use regex::Regex;

use crate::object::Object;

pub struct Lisp {
    variables: HashMap<String, Object>,
}

impl Lisp {
    pub fn new() -> Self {
        let mut variables = HashMap::new();

        variables.insert("nil".to_string(), Object::Nil);

        Self {
            variables
        }
    }
   
    fn split_into_strings(input: &str) -> Vec<String> {
        let regex = Regex::new(r"(?m)\(|\)|[^\s()]*").unwrap();
 
        regex.captures_iter(input)
            .filter_map(|x| {
                let s = x.get(0).unwrap().as_str().to_string();

                if s.is_empty() {
                    None
                } else {
                    Some(s)
                }
            })
            .collect()
    }

    fn eval_symbol(&mut self, symbol: &str) -> &mut Object {
        self.variables.get_mut(symbol).expect(&format!("Undefined variable '{}'", symbol))
    }

    pub fn eval(&mut self, input: &str) -> String {
        let strings = Self::split_into_strings(input); // Split code into seperate tokens
        let object = Object::eval(strings); // Evaluate tokens into objects

        match object {
            Object::Pair(_,_) => String::new(), // Execute function
            _ => format!("{}", object),
        }
    }
}
