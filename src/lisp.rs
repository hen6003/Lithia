use std::collections::HashMap;

use crate::object::Object;
use crate::errors::*;

pub struct Lisp {
    variables: HashMap<String, Box<Object>>,
}

impl Lisp {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn add_var(&mut self, name: &str, object: Box<Object>) -> &mut Self {
        self.variables.insert(name.to_string(), object);
        self
    }
    
    pub fn add_func(&mut self, name: &str, func: fn (&mut Self, Object) -> RustFuncResult) -> &mut Self {
        self.add_var(name, Box::new(Object::RustFunc(func)))
    } 
   
    fn eval_symbol(&mut self, symbol: &str) -> LispResult {
        if !symbol.is_empty() {
            match self.variables.get(symbol) {
                Some(s) => Ok(s.clone()),
                None => Err(LispError::new(LispErrorKind::Eval,
                                           EvalError::UnknownSymbol(symbol.to_string())))
            }
        } else {
            Ok(Box::new(Object::Nil))
        }
    }

    pub fn set_var(&mut self, symbol: &str, data: Box<Object>) {
        if let Some(s) = self.variables.get_mut(symbol) {
            *s = data;
        } else {
            self.add_var(symbol, data);
        }
    }

    pub fn eval_object(&mut self, object: Box<Object>) -> LispResult {
        match *object {
            Object::Pair(f, a) => { // Execute function
                match *self.eval_object(f)? {
                    Object::RustFunc(f) => match f(self, *a) {
                        Ok(x) => Ok(x),
                        Err(e) => Err(LispError::new(LispErrorKind::RustFunc, e)),
                    },
                    o => Err(LispError::new(LispErrorKind::Eval,
                                            EvalError::NonFunction(o))) 
                }
            },
            Object::Symbol(s) => Ok(self.eval_symbol(&s)?),
            _ => Ok(object),
        }
    }
    
    pub fn eval_objects(&mut self, objects: Vec<Box<Object>>) -> LispResult {
        let mut ret = Box::new(Object::Nil);

        for o in objects {
            ret = self.eval_object(o)?
        }

        Ok(ret)
    }

    pub fn eval(&mut self, input: &str) -> LispResult {
        let objects = Object::eval(input)?; // Evaluate tokens into objects
        let objects = objects.into_iter().map(Box::new).collect(); // Store objects on the heap

        self.eval_objects(objects)
    }
}
