use std::collections::HashMap;

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

    pub fn add_var(&mut self, name: &str, object: Object) -> &mut Self {
        self.variables.insert(name.to_string(), object);
        self
    }
    
    pub fn add_func(&mut self, name: &str, func: fn (&mut Self, Object) -> Object) -> &mut Self {
        self.add_var(name, Object::RustFunc(func))
    } 

    pub fn eval(&mut self, input: &str) -> Object {
        let object = Object::eval(input); // Evaluate tokens into objects

        self.eval_object(object)
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
}
