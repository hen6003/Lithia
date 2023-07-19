use alloc::rc::Rc;

use crate::{
    errors::*,
    lisp::{Lisp, LispBuilder},
    object::Object,
};

impl LispBuilder {
    pub fn add_env_maths(self) -> Result<Self, LispError> {
        self.add_func("sqrt", sqrt)?
            .add_func("pow", pow)?
            .add_func("exp", exp)?
            // Symbol names
            .add_func("^", pow)
    }
}

fn sqrt(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    match &*arg {
        Object::Pair(a, b) => {
            let value = match *lisp.eval_object(Rc::clone(a))? {
                Object::Number(i) => Ok(i),
                _ => Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
            };

            match **b {
                Object::Nil => value.map(|v| Rc::new(Object::Number(v))),
                _ => Err(RustFuncError::InvalidArguments(ArgumentsError::TooMany)),
            }
        }
        Object::Nil => Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    }
}

fn exp(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    match &*arg {
        Object::Pair(a, b) => {
            let value = match *lisp.eval_object(Rc::clone(a))? {
                Object::Number(i) => Ok(i),
                _ => Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
            };

            match **b {
                Object::Nil => value.map(|v| Rc::new(Object::Number(v.exp()))),
                _ => Err(RustFuncError::InvalidArguments(ArgumentsError::TooMany)),
            }
        }
        Object::Nil => Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    }
}

fn pow(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    let first;
    let second;

    match &*arg {
        Object::Pair(a, b) => {
            first = match *lisp.eval_object(Rc::clone(a))? {
                Object::Number(i) => Ok(i),
                _ => Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
            }?;

            match &**b {
                Object::Pair(a, b) => {
                    second = match *lisp.eval_object(Rc::clone(a))? {
                        Object::Number(i) => Ok(i),
                        _ => Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
                    }?;

                    match &**b {
                        Object::Nil => (),
                        _ => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
                    }
                }
                Object::Nil => {
                    return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough))
                }
                _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
            }
        }
        Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };

    Ok(Rc::new(Object::Number(first.powf(second))))
}
