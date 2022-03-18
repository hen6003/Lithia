use std::collections::HashMap;
use regex::Regex;

use crate::object::Object;

pub struct Lisp {
    variables: HashMap<String, Object>,
}

impl Lisp {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn add_var(&mut self, name: &str, object: Object) {
        self.variables.insert(name.to_string(), object);
    }
    
    pub fn add_func(&mut self, name: &str, func: fn (&mut Self, Object) -> Object) {
        self.add_var(name, Object::RustFunc(func));
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
        self.variables.get_mut(symbol).unwrap_or_else(|| panic!("Undefined variable '{}'", symbol))
    }

    pub fn eval_object(&mut self, object: Object) -> Object {
        match object {
            Object::Pair(f, a) => { // Execute function
                if let Object::Symbol(s) = *f {
                    match self.eval_symbol(&s) {
                        Object::RustFunc(f) => f(self, *a),
                        _ => panic!("Symbol was not a function")
                    }
                } else {
                    panic!("Expected symbol for function name")
                }
            },
            a => a,
        }
    }

    pub fn eval(&mut self, input: &str) -> String {
        let strings = Self::split_into_strings(input); // Split code into seperate tokens
        let object = Object::eval(strings); // Evaluate tokens into objects

        match object {
            Object::Pair(f, a) => { // Execute function
                if let Object::Symbol(s) = *f {
                    match self.eval_symbol(&s) {
                        Object::RustFunc(f) => f(self, *a).to_string(),
                        _ => panic!("Symbol was not a function")
                    }
                } else {
                    panic!("Expected symbol for function name")
                }
            },
            _ => format!("{}", object),
        }
    }
}
