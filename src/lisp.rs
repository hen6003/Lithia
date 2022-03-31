use std::collections::HashMap;

use crate::object::Object;
use crate::errors::*;

pub struct LispScope<'a> {
    variables: HashMap<String, Box<Object>>,
    inherit: Option<&'a LispScope<'a>>,
}

impl<'a> LispScope<'a> {
    pub fn new(inherit: Option<&'a LispScope>) -> Self {
        Self {
            variables: HashMap::new(),
	    inherit,
        }
    }

    pub fn add_var(&mut self, name: &str, object: Box<Object>) -> &mut Self {
        self.variables.insert(name.to_string(), object);
        self
    }
    
    pub fn add_func(&mut self, name: &str, func: fn (&mut LispScope, Object) -> RustFuncResult) -> &mut Self {
        self.add_var(name, Box::new(Object::RustFunc(func)))
    } 
   
    fn eval_symbol(&self, symbol: &str) -> LispResult {
        if !symbol.is_empty() {
            match self.variables.get(symbol) {
                Some(s) => Ok(s.clone()),
		// Check inherited variables
                None => if let Some(i) = self.inherit {
		    i.eval_symbol(symbol)
		} else {
		    Err(LispError::new(LispErrorKind::Eval,
				       EvalError::UnknownSymbol(symbol.to_string())))
		}
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
            Object::Pair(ref f, ref a) => { // Execute expression
                match *self.eval_object(f.clone())? {
                    Object::RustFunc(f) => match f(self, *a.clone()) {
                        Ok(x) => Ok(x),
                        Err(e) => Err(LispError::new(LispErrorKind::RustFunc, e)),
                    },
		    Object::LispFunc(p, b) => {
			let mut args = Vec::new();
			let objects = b.into_iter().map(Box::new).collect(); // Store objects on the heap

			// Create args
			let mut cur_object = a;
			
			loop {
	    		    match &**cur_object {
	    			Object::Pair(a, b) => {
	    			    args.push(self.eval_object(a.clone())?);
				    
	    			    cur_object = b
	    			},
	    			Object::Nil => break,
	    			_ => return Err(LispError::new(LispErrorKind::RustFunc, RustFuncError::new_args_error(ArgumentsError::DottedPair))),
	    		    }
			}

			let mut scope = match self.inherit {
			    Some(_) => LispScope::new(self.inherit),
			    None => LispScope::new(Some(self)),
			};   

			for (i, p) in p.iter().enumerate() {
			    match args.get(i) {
				Some(a) => scope.add_var(p, a.clone()),
				None => return Err(LispError::new(LispErrorKind::RustFunc, RustFuncError::new_args_error(ArgumentsError::NotEnough))),
			    };
			}

			// Call function
			scope.eval_objects(objects)
		    },
		    Object::Character(_) => Ok(object),
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
