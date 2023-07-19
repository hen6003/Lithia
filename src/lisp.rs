use alloc::{
    rc::Rc,
    string::{String, ToString},
    vec,
    vec::Vec,
};

use core::cell::RefCell;

#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(not(feature = "std"))]
use hashbrown::HashMap;

use crate::{errors::*, object::Object};

#[derive(Clone)]
pub struct LispBuilder {
    lisp: Lisp,
}

impl LispBuilder {
    pub fn new() -> Self {
        Self {
            lisp: Lisp::new(Rc::new(RefCell::new(HashMap::new()))),
        }
    }

    pub fn add_var(
        mut self,
        global: bool,
        name: &str,
        object: Rc<Object>,
    ) -> Result<Self, LispError> {
        self.lisp.add_var(global, name, object).map(|_| self)
    }

    pub fn add_func(
        mut self,
        global: bool,
        name: &str,
        func: fn(&mut Lisp, Rc<Object>) -> RustFuncResult,
    ) -> Result<Self, LispError> {
        self.lisp.add_func(global, name, func).map(|_| self)
    }

    pub fn build(self) -> Lisp {
        self.lisp
    }
}

#[derive(Clone)]
pub struct Lisp {
    scope: Vec<HashMap<String, Rc<Object>>>,
    pub globals: Rc<RefCell<HashMap<String, Rc<Object>>>>,
}

impl Lisp {
    pub(crate) fn new(globals: Rc<RefCell<HashMap<String, Rc<Object>>>>) -> Self {
        Self {
            scope: vec![HashMap::new()],
            globals,
        }
    }

    // New scope
    pub(crate) fn scope_create(&mut self) {
        self.scope.push(HashMap::new());
    }

    // End scope
    pub(crate) fn scope_end(&mut self) {
        self.scope.pop();
    }

    pub(crate) fn add_var(
        &mut self,
        global: bool,
        name: &str,
        object: Rc<Object>,
    ) -> Result<(), LispError> {
        if global {
            let mut globals = self.globals.borrow_mut();

            match globals.get(name) {
                Some(_) => {
                    return Err(LispError::new(
                        LispErrorKind::Eval,
                        EvalError::GlobalExists(name.to_string()),
                    ))
                }
                None => globals.insert(name.to_string(), object),
            }
        } else {
            let len = self.scope.len();
            self.scope[len - 1].insert(name.to_string(), object)
        };

        Ok(())
    }

    pub(crate) fn add_func(
        &mut self,
        global: bool,
        name: &str,
        func: fn(&mut Lisp, Rc<Object>) -> RustFuncResult,
    ) -> Result<(), LispError> {
        self.add_var(global, name, Rc::new(Object::RustFunc(func)))
    }

    fn eval_symbol(&self, symbol: &str) -> LispResult {
        if !symbol.is_empty() {
            for s in self.scope.iter().rev() {
                if let Some(o) = s.get(symbol) {
                    return Ok(o.clone());
                }
            }

            // Check globals
            match self.globals.borrow_mut().get(symbol) {
                Some(o) => Ok(o.clone()),
                None => Err(LispError::new(
                    LispErrorKind::Eval,
                    EvalError::UnknownSymbol(symbol.to_string()),
                )),
            }
        } else {
            Ok(Rc::new(Object::Nil))
        }
    }

    pub(crate) fn set_var(&mut self, symbol: &str, data: Rc<Object>) -> Result<(), LispError> {
        // Check for variable, going up scope if it can't find it
        for v in self.scope.iter_mut().rev() {
            if let Some(s) = v.get_mut(symbol) {
                *s = data;
                return Ok(());
            }
        }

        // Check for variable in globals
        if let Some(s) = self.globals.borrow_mut().get_mut(symbol) {
            *s = data;
            return Ok(());
        }

        // If variable not found, create it
        self.add_var(false, symbol, data)?;

        Ok(())
    }

    pub(crate) fn eval_object(&mut self, object: Rc<Object>) -> LispResult {
        match &*object {
            Object::Pair(ref f, ref a) => {
                // Execute expression
                match &*self.eval_object(f.clone())? {
                    Object::RustFunc(f) => match f(self, Rc::clone(a)) {
                        Ok(x) => Ok(x),
                        Err(e) => Err(LispError::new(LispErrorKind::RustFunc, e)),
                    },
                    Object::LispFunc(p, b) => {
                        let mut args = Vec::new();
                        let objects = b.iter().map(Rc::clone).collect(); // Store objects on the heap

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

                        let mut scope = Lisp::new(self.globals.clone());

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
                    Object::Character(_) => Ok(Rc::clone(&object)),
                    _ => Err(LispError::new(
                        LispErrorKind::Eval,
                        EvalError::NonFunction(Rc::clone(&object)),
                    )),
                }
            }
            Object::Symbol(s) => Ok(self.eval_symbol(s)?),
            Object::Quoted(o) => Ok(Rc::clone(o)),
            _ => Ok(object),
        }
    }

    pub fn eval_objects(&mut self, objects: Vec<Rc<Object>>) -> LispResult {
        let mut ret = Rc::new(Object::Nil);

        for o in objects {
            ret = self.eval_object(o)?
        }

        Ok(ret)
    }

    pub fn eval(&mut self, input: &str) -> LispResult {
        let objects = Object::eval(input)?; // Evaluate tokens into objects
        let objects = objects.into_iter().map(Rc::new).collect(); // Store objects on the heap

        self.eval_objects(objects)
    }
}
