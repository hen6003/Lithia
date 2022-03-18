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
        
        #[cfg(debug_assertions)]
        variables.insert("internal".to_string(), Object::RustFunc(internal));

        Self {
            variables
        }
    }

    pub fn add_func(&mut self, name: &str, func: fn (Object) -> Object) {
        self.variables.insert(name.to_string(), Object::RustFunc(func));
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

    pub fn eval(&mut self, input: &str) -> String {
        let strings = Self::split_into_strings(input); // Split code into seperate tokens
        let object = Object::eval(strings); // Evaluate tokens into objects

        match object {
            Object::Pair(f, a) => { // Execute function
                if let Object::Symbol(s) = *f {
                    match self.eval_symbol(&s) {
                        Object::RustFunc(f) => {
                            let a = match *a {
                                Object::Pair(a, n) => {
                                    if *n == Object::Nil {
                                        a
                                    } else {
                                        panic!("Arguments to function must not be dotted")
                                    }
                                },
                                _ => panic!("Arguments to function must not be dotted"),
                            };

                            f(*a).to_string()
                        },
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

// Display internal version of object
#[cfg(debug_assertions)]
fn internal(arg: Object) -> Object {
    println!("{:?}", arg);
    Object::Nil
}
