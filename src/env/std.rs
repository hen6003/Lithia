// Defining the standard functions and variables that exist in the language

use alloc::{
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};

use crate::{
    errors::*,
    lisp::{Lisp, LispBuilder},
    object::Object,
};

impl LispBuilder {
    pub fn add_env_std(self) -> Result<Self, LispError> {
        // Variables
        self.add_var("t", Rc::new(Object::True))?
            .add_var("f", Rc::new(Object::Nil))?
            .add_var("pi", Rc::new(Object::Number(core::f32::consts::PI)))?
            // Functions
            .add_func("quote", quote)?
            .add_func("eval", eval)?
            .add_func("while", lispwhile)?
            .add_func("if", lispif)?
            .add_func("func", func)?
            .add_func("car", car)?
            .add_func("cdr", cdr)?
            .add_func("cons", cons)?
            .add_func("=", set)?
            .add_func("def", define)?
            .add_func("defunc", defunc)?
            // Math functions
            .add_func("set", set)?
            .add_func("add", add)?
            .add_func("sub", minus)?
            .add_func("mul", times)?
            .add_func("div", divide)?
            .add_func("mod", modulus)?
            .add_func("eq", equal)?
            .add_func("ne", notequal)?
            // Symbol names
            .add_func("+", add)?
            .add_func("-", minus)?
            .add_func("*", times)?
            .add_func("/", divide)?
            .add_func("%", modulus)?
            .add_func("==", equal)?
            .add_func("!=", notequal)
    }
}

fn cons(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    let first;
    let second;

    match &*arg {
        Object::Pair(a, b) => {
            first = lisp.eval_object(Rc::clone(a))?;

            match &**b {
                Object::Pair(a, b) => {
                    second = lisp.eval_object(Rc::clone(a))?;

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

    Ok(Rc::new(Object::Pair(first, second)))
}

fn modulus(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    let mut sum;
    let mut cur_object = match &*arg {
        Object::Pair(a, b) => {
            sum = match *lisp.eval_object(Rc::clone(a))? {
                Object::Number(i) => i,
                _ => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
            };

            b
        }
        Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };

    loop {
        match &**cur_object {
            Object::Pair(a, b) => {
                sum %= match *lisp.eval_object(Rc::clone(a))? {
                    Object::Number(i) => i,
                    _ => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
                };

                cur_object = b
            }
            Object::Nil => break Ok(Rc::new(Object::Number(sum))),
            _ => break Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
        }
    }
}

fn divide(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    let mut sum;
    let mut cur_object = match &*arg {
        Object::Pair(a, b) => {
            sum = match *lisp.eval_object(Rc::clone(a))? {
                Object::Number(i) => i,
                _ => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
            };

            b
        }
        Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };

    loop {
        match &**cur_object {
            Object::Pair(a, b) => {
                sum /= match *lisp.eval_object(Rc::clone(a))? {
                    Object::Number(i) => i,
                    _ => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
                };

                cur_object = b
            }
            Object::Nil => break Ok(Rc::new(Object::Number(sum))),
            _ => break Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
        }
    }
}

fn times(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    let mut sum;
    let mut cur_object = match &*arg {
        Object::Pair(a, b) => {
            sum = match *lisp.eval_object(Rc::clone(a))? {
                Object::Number(i) => i,
                _ => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
            };

            b
        }
        Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };

    loop {
        match &**cur_object {
            Object::Pair(a, b) => {
                sum *= match *lisp.eval_object(Rc::clone(a))? {
                    Object::Number(i) => i,
                    _ => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
                };

                cur_object = b
            }
            Object::Nil => break Ok(Rc::new(Object::Number(sum))),
            _ => break Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
        }
    }
}

fn minus(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    let mut sum;
    let mut cur_object = match &*arg {
        Object::Pair(a, b) => {
            sum = match *lisp.eval_object(Rc::clone(a))? {
                Object::Number(i) => i,
                _ => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
            };

            b
        }
        Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };

    loop {
        match &**cur_object {
            Object::Pair(a, b) => {
                sum -= match *lisp.eval_object(Rc::clone(a))? {
                    Object::Number(i) => i,
                    _ => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
                };

                cur_object = b
            }
            Object::Nil => break Ok(Rc::new(Object::Number(sum))),
            _ => break Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
        }
    }
}

fn add(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    let mut sum;
    let mut cur_object = match &*arg {
        Object::Pair(a, b) => {
            sum = match *lisp.eval_object(Rc::clone(a))? {
                Object::Number(i) => i,
                _ => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
            };

            b
        }
        Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };

    loop {
        match &**cur_object {
            Object::Pair(a, b) => {
                sum += match *lisp.eval_object(Rc::clone(a))? {
                    Object::Number(i) => i,
                    _ => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
                };

                cur_object = b;
            }
            Object::Nil => break Ok(Rc::new(Object::Number(sum))),
            _ => break Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
        }
    }
}

// Set variable
fn set(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    let (symbol, data) = match &*arg {
        Object::Pair(a, b) => match &**b {
            Object::Pair(c, d) => {
                if **d != Object::Nil {
                    return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough));
                }

                (a, c)
            }
            Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
            _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
        },
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };

    if let Object::Symbol(symbol) = &**symbol {
        let data = lisp.eval_object(Rc::clone(data))?;
        lisp.set_var(symbol, data)?;
    } else {
        return Err(RustFuncError::new_args_error(ArgumentsError::WrongType));
    }

    Ok(Rc::new(Object::Nil))
}

// Define global
fn define(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    let (symbol, data) = match &*arg {
        Object::Pair(a, b) => match &**b {
            Object::Pair(c, d) => {
                if **d != Object::Nil {
                    return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough));
                }

                (a, c)
            }
            Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
            _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
        },
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };

    if let Object::Symbol(symbol) = &**symbol {
        let data = lisp.eval_object(Rc::clone(data))?;
        lisp.add_var(true, symbol, data)?;
    } else {
        return Err(RustFuncError::new_args_error(ArgumentsError::WrongType));
    }

    Ok(Rc::new(Object::Nil))
}

// Define global function
fn defunc(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    let (symbol, function) = match &*arg {
        Object::Pair(a, b) => (a, func(lisp, Rc::clone(b))?),
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };

    if let Object::Symbol(symbol) = &**symbol {
        lisp.add_var(true, symbol, function)?;
    } else {
        return Err(RustFuncError::new_args_error(ArgumentsError::WrongType));
    }

    Ok(Rc::new(Object::Nil))
}

// Evaluate an object and what it returns
fn eval(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    let mut objects = Vec::new();
    let mut cur_object = match &*arg {
        Object::Pair(a, b) => {
            objects.push(lisp.eval_object(Rc::clone(a))?);

            &**b
        }
        Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };

    loop {
        match cur_object {
            Object::Pair(a, b) => {
                objects.push(lisp.eval_object(Rc::clone(a))?);

                cur_object = &*b
            }
            Object::Nil => break,
            _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
        }
    }

    Ok(lisp.eval_objects(objects)?)
}

// Evaluates the given object forever
fn lispwhile(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    let cond;
    let mut objects = Vec::new();
    let mut cur_object = match &*arg {
        Object::Pair(a, b) => {
            cond = a;

            &*b
        }
        Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };

    loop {
        match &**cur_object {
            Object::Pair(a, b) => {
                objects.push(Rc::clone(a));

                cur_object = &*b
            }
            Object::Nil => break,
            _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
        }
    }

    lisp.scope_create();

    while *lisp.eval_object(cond.clone())? != Object::Nil {
        lisp.eval_objects(objects.clone())?;
    }

    lisp.scope_end();

    Ok(Rc::new(Object::Nil))
}

// Evaluates the given object forever conditionally
fn lispif(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    let first;
    let second;
    let third;

    match &*arg {
        Object::Pair(a, b) => {
            first = a;

            match &**b {
                Object::Pair(a, b) => {
                    second = a;

                    match &**b {
                        Object::Pair(a, b) => {
                            third = Some(a);

                            match &**b {
                                Object::Nil => (),
                                _ => {
                                    return Err(RustFuncError::new_args_error(
                                        ArgumentsError::NotEnough,
                                    ))
                                }
                            }
                        }
                        Object::Nil => third = None,
                        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
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

    lisp.scope_create();

    let ret = if *lisp.eval_object(first.clone())? != Object::Nil {
        lisp.eval_object(second.clone())?
    } else if let Some(third) = third {
        lisp.eval_object(third.clone())?
    } else {
        Rc::new(Object::Nil)
    };

    lisp.scope_end();

    Ok(ret)
}

fn equal(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    let first;
    let second;

    match &*arg {
        Object::Pair(a, b) => {
            first = a;

            match &**b {
                Object::Pair(a, b) => {
                    second = a;

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

    if *lisp.eval_object(Rc::clone(first))? == *lisp.eval_object(Rc::clone(second))? {
        Ok(Rc::new(Object::True))
    } else {
        Ok(Rc::new(Object::Nil))
    }
}

fn notequal(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    let first;
    let second;

    match &*arg {
        Object::Pair(a, b) => {
            first = a;

            match &**b {
                Object::Pair(a, b) => {
                    second = a;

                    match **b {
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

    if *lisp.eval_object(Rc::clone(first))? != *lisp.eval_object(Rc::clone(second))? {
        Ok(Rc::new(Object::True))
    } else {
        Ok(Rc::new(Object::Nil))
    }
}

// Returns whatever its given, used for when you don't want to evaluate something
fn quote(_: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    match &*arg {
        Object::Pair(a, b) => {
            if **b != Object::Nil {
                Err(RustFuncError::new_args_error(ArgumentsError::TooMany))
            } else {
                Ok(Rc::clone(a))
            }
        }
        Object::Nil => Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    }
}

// For creating functions
fn func(_: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    let mut lisp_list_args;
    let mut func_body = Vec::new();

    match &*arg {
        Object::Pair(a, b) => {
            match **a {
                Object::Pair(_, _) => lisp_list_args = a,
                Object::Nil => lisp_list_args = a,
                _ => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
            }

            let mut cur_object = &*b;

            loop {
                match &**cur_object {
                    Object::Pair(a, b) => {
                        func_body.push(Rc::clone(a));

                        cur_object = &*b
                    }
                    Object::Nil => break,
                    _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
                }
            }
        }
        Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };

    let mut args: Vec<String> = Vec::new();

    loop {
        match &**lisp_list_args {
            Object::Pair(a, b) => {
                if let Object::Symbol(s) = &**a {
                    args.push(s.to_string());
                } else {
                    return Err(RustFuncError::new_args_error(ArgumentsError::WrongType));
                }

                lisp_list_args = b
            }
            Object::Nil => break,
            _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
        }
    }

    Ok(Rc::new(Object::LispFunc(args, func_body)))
}

// Get first item in a list
fn car(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    match &*arg {
        Object::Pair(a, b) => {
            if **b != Object::Nil {
                return Err(RustFuncError::new_args_error(ArgumentsError::TooMany));
            }

            match &*lisp.eval_object(Rc::clone(a))? {
                Object::Pair(a, _) => Ok(Rc::clone(a)),
                _ => Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
            }
        }
        Object::Nil => Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    }
}

// Get next item in a list
fn cdr(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    match &*arg {
        Object::Pair(a, b) => {
            if **b != Object::Nil {
                return Err(RustFuncError::new_args_error(ArgumentsError::TooMany));
            }

            match &*lisp.eval_object(Rc::clone(a))? {
                Object::Pair(_, b) => Ok(Rc::clone(b)),
                _ => Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
            }
        }
        Object::Nil => Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    }
}
