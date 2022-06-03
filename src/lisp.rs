use std::collections::HashMap;

use crate::errors::*;
use crate::object::Object;

pub struct Lisp<'a> {
    scope: Vec<HashMap<String, Box<Object>>>,
    pub globals: &'a mut HashMap<String, Box<Object>>,
}

impl<'a> Lisp<'a> {
    pub fn new(globals: &'a mut HashMap<String, Box<Object>>) -> Self {
        Self {
            scope: vec![HashMap::new()],
            globals,
        }
    }

    // New scope
    pub fn scope_create(&mut self) {
        self.scope.push(HashMap::new());
    }

    // End scope
    pub fn scope_end(&mut self) {
        self.scope.pop();
    }

    pub fn add_var(
        &mut self,
        global: bool,
        name: &str,
        object: Box<Object>,
    ) -> Result<&mut Self, LispError> {
        if global {
            match self.globals.get(name) {
                Some(_) => {
                    return Err(LispError::new(
                        LispErrorKind::Eval,
                        EvalError::GlobalExists(name.to_string()),
                    ))
                }
                None => self.globals.insert(name.to_string(), object),
            }
        } else {
            let len = self.scope.len();
            self.scope[len - 1].insert(name.to_string(), object)
        };

        Ok(self)
    }

    pub fn add_func(
        &mut self,
        global: bool,
        name: &str,
        func: fn(&mut Lisp, Object) -> RustFuncResult,
    ) -> Result<&mut Self, LispError> {
        self.add_var(global, name, Box::new(Object::RustFunc(func)))
    }

    fn eval_symbol(&self, symbol: &str) -> LispResult {
        if !symbol.is_empty() {
            for s in self.scope.iter().rev() {
                if let Some(o) = s.get(symbol) {
                    return Ok(o.clone());
                }
            }

            // Check globals
            match self.globals.get(symbol) {
                Some(o) => Ok(o.clone()),
                None => Err(LispError::new(
                    LispErrorKind::Eval,
                    EvalError::UnknownSymbol(symbol.to_string()),
                )),
            }
        } else {
            Ok(Box::new(Object::Nil))
        }
    }

    pub fn set_var(&mut self, symbol: &str, data: Box<Object>) -> Result<(), LispError> {
        // Check for variable, going up scope if it can't find it
        for v in self.scope.iter_mut().rev() {
            if let Some(s) = v.get_mut(symbol) {
                *s = data;
                return Ok(());
            }
        }

        // If variable not found, create it
        self.add_var(false, symbol, data)?;

        Ok(())
    }

    pub fn eval_object(&mut self, object: Box<Object>) -> LispResult {
        match *object {
            Object::Pair(ref f, ref a) => {
                // Execute expression
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
                                }
                                Object::Nil => break,
                                _ => {
                                    return Err(LispError::new(
                                        LispErrorKind::RustFunc,
                                        RustFuncError::new_args_error(ArgumentsError::DottedPair),
                                    ))
                                }
                            }
                        }

                        let mut scope = Lisp::new(self.globals);

                        for (i, p) in p.iter().enumerate() {
                            match args.get(i) {
                                Some(a) => scope.add_var(false, p, a.clone())?,
                                None => {
                                    return Err(LispError::new(
                                        LispErrorKind::RustFunc,
                                        RustFuncError::new_args_error(ArgumentsError::NotEnough),
                                    ))
                                }
                            };
                        }

                        // Call function
                        scope.eval_objects(objects)
                    }
                    Object::Character(_) => Ok(object),
                    o => Err(LispError::new(
                        LispErrorKind::Eval,
                        EvalError::NonFunction(o),
                    )),
                }
            }
            Object::Symbol(s) => Ok(self.eval_symbol(&s)?),
            Object::Quoted(o) => Ok(o),
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
