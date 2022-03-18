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

    fn eval_symbol(&mut self, symbol: &str) -> &Object {
        self.variables.get(symbol).unwrap_or_else(|| panic!("Undefined variable '{}'", symbol))
    }

    pub fn set_var(&mut self, symbol: &str, data: Object) {
        if let Some(s) = self.variables.get_mut(symbol) {
            *s = data;
        } else {
            self.add_var(symbol, data);
        }
    }

    pub fn eval_object(&mut self, object: Object) -> Object {
        match object {
            Object::Pair(f, a) => { // Execute function
                match self.eval_object(*f) {
                    Object::RustFunc(f) => f(self, *a),
                    _ => panic!("Object was not a function")
                }
            },
            Object::Symbol(s) => self.eval_symbol(&s).clone(),
            a => a,
        }
    }

    pub fn eval(&mut self, input: &str) -> String {
        let strings = Self::split_into_strings(input); // Split code into seperate tokens
        let object = Object::eval(strings); // Evaluate tokens into objects

        format!("{}", self.eval_object(object))
    }
}
