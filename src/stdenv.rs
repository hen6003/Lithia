use crate::lisp::LispScope;
use crate::object::Object;
use crate::errors::*;

impl<'a> LispScope<'a> {
    pub fn add_stdenv(&mut self) -> &mut Self {
        // Variables
        self.add_var("nil", Box::new(Object::Nil));
        self.add_var("t", Box::new(Object::True));
        self.add_var("pi", Box::new(Object::Number(std::f32::consts::PI)));
        
        // Functions
        self.add_func("quote", quote);
        self.add_func("exit", exit);
        self.add_func("eval", eval);
        self.add_func("while", lispwhile);
        self.add_func("print", print);
        self.add_func("read", read);
        self.add_func("func", func);

        // Math functions
        self.add_func("=", set);
        self.add_func("+", add);
        self.add_func("-", minus);
        self.add_func("*", times);
        self.add_func("/", divide);
        self.add_func("==", equal);
        self.add_func("!=", notequal);
        
        // Non-symbol names
        self.add_func("set", set);
        self.add_func("add", add);
        self.add_func("sub", minus);
        self.add_func("mul", times);
        self.add_func("div", divide);
        self.add_func("eq", equal);
        self.add_func("ne", notequal);

        self
    }
}

fn divide(lisp: &mut LispScope, arg: Object) -> RustFuncResult {
    let mut sum;
    let mut cur_object: Object = match arg {
        Object::Pair(a, b) => {
            sum = match *lisp.eval_object(a)? {
                Object::Number(i) => i,
                _ => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
            };

            *b
        },
        Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };
    
    loop {
        match cur_object {
            Object::Pair(a, b) => {
                sum /= match *lisp.eval_object(a)? {
                    Object::Number(i) => i,
                    _ => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
                };

                cur_object = *b
            },
            Object::Nil => break Ok(Box::new(Object::Number(sum))),
            _ => break Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
        }
    }
}

fn times(lisp: &mut LispScope, arg: Object) -> RustFuncResult {
    let mut sum;
    let mut cur_object: Object = match arg {
        Object::Pair(a, b) => {
            sum = match *lisp.eval_object(a)? {
                Object::Number(i) => i,
                _ => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
            };

            *b
        },
        Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };
    
    loop {
        match cur_object {
            Object::Pair(a, b) => {
                sum *= match *lisp.eval_object(a)? {
                    Object::Number(i) => i,
                    _ => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
                };

                cur_object = *b
            },
            Object::Nil => break Ok(Box::new(Object::Number(sum))),
            _ => break Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
        }
    }
}

fn minus(lisp: &mut LispScope, arg: Object) -> RustFuncResult {
    let mut sum;
    let mut cur_object: Object = match arg {
        Object::Pair(a, b) => {
            sum = match *lisp.eval_object(a)? {
                Object::Number(i) => i,
                _ => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
            };

            *b
        },
        Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };
    
    loop {
        match cur_object {
            Object::Pair(a, b) => {
                sum -= match *lisp.eval_object(a)? {
                    Object::Number(i) => i,
                    _ => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
                };

                cur_object = *b
            },
            Object::Nil => break Ok(Box::new(Object::Number(sum))),
            _ => break Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
        }
    }
}

fn add(lisp: &mut LispScope, arg: Object) -> RustFuncResult {
    let mut sum;
    let mut cur_object: Object = match arg {
        Object::Pair(a, b) => {
            sum = match *lisp.eval_object(a)? {
                Object::Number(i) => i,
                _ => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
            };

            *b
        },
        Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };
    
    loop {
        match cur_object {
            Object::Pair(a, b) => {
                sum += match *lisp.eval_object(a)? {
                    Object::Number(i) => i,
                    _ => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
                };

                cur_object = *b
            },
            Object::Nil => break Ok(Box::new(Object::Number(sum))),
            _ => break Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
        }
    }
}

// Set variable
fn set(lisp: &mut LispScope, arg: Object) -> RustFuncResult {
    let (symbol, data) = match arg {
        Object::Pair(a, b) => {
            match *b {
                Object::Pair(c, d) => {
                    if *d != Object::Nil {
                        return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough))
                    }

                    (a, c)
                },
                Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
                _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
            }
        },
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };

    if let Object::Symbol(symbol) = *symbol {
        let data = lisp.eval_object(data)?;
        lisp.set_var(&symbol, data);
    } else {
        return Err(RustFuncError::new_args_error(ArgumentsError::WrongType))
    }

    Ok(Box::new(Object::Nil))
}

// Evaluate an object and what it returns
fn eval(lisp: &mut LispScope, arg: Object) -> RustFuncResult {
    let mut objects = Vec::new();
    let mut cur_object: Object = match arg {
        Object::Pair(a, b) => {
            objects.push(lisp.eval_object(a)?);

            *b
        },
        Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };
    
    loop {
        match cur_object {
            Object::Pair(a, b) => {
                objects.push(lisp.eval_object(a)?);

                cur_object = *b
            },
            Object::Nil => break,
            _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
        }
    }

    Ok(lisp.eval_objects(objects)?)
}

// Evaluates the given object forever
fn lispwhile(lisp: &mut LispScope, arg: Object) -> RustFuncResult {
    let cond;
    let mut objects = Vec::new();
    let mut cur_object: Object = match arg {
        Object::Pair(a, b) => {
            cond = a;

            *b
        },
        Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };
    
    loop {
        match cur_object {
            Object::Pair(a, b) => {
                objects.push(a);

                cur_object = *b
            },
            Object::Nil => break,
            _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
        }
    }

    while *lisp.eval_object(cond.clone())? != Object::Nil {
        lisp.eval_objects(objects.clone())?;
    }

    Ok(Box::new(Object::Nil))
}

// Evaluates the given object forever
fn equal(lisp: &mut LispScope, arg: Object) -> RustFuncResult {
    let first;
    let second;
    
    match arg {
        Object::Pair(a, b) => {
            first = a;

            match *b {
                Object::Pair(a, b) => {
                    second = a;

                    match *b {
                        Object::Nil => (),
                        _ => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
                    }
                },
                Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
                _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
            }
        },
        Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    }; 

    if *lisp.eval_object(first)? == *lisp.eval_object(second)? {
        Ok(Box::new(Object::True))
    } else {
        Ok(Box::new(Object::Nil))
    }
}

// Evaluates the given object forever
fn notequal(lisp: &mut LispScope, arg: Object) -> RustFuncResult {
    let first;
    let second;
    
    match arg {
        Object::Pair(a, b) => {
            first = a;

            match *b {
                Object::Pair(a, b) => {
                    second = a;

                    match *b {
                        Object::Nil => (),
                        _ => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
                    }
                },
                Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
                _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
            }
        },
        Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    }; 

    if *lisp.eval_object(first)? != *lisp.eval_object(second)? {
        Ok(Box::new(Object::True))
    } else {
        Ok(Box::new(Object::Nil))
    }
}

// Returns whatever its given, used for when you don't want to evaluate something
fn quote(_: &mut LispScope, arg: Object) -> RustFuncResult {
    match arg {
        Object::Pair(a, b) => {
            if *b != Object::Nil {
                Err(RustFuncError::new_args_error(ArgumentsError::TooMany))
            } else {
                Ok(a)
            }
        },
        _ => Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    }
}

// Exit lisp interpreter, number may be provided for exit code
fn exit(lisp: &mut LispScope, arg: Object) -> RustFuncResult {
    let exit_code = match arg {
        Object::Pair(a, b) => {
            if *b != Object::Nil {
                return Err(RustFuncError::new_args_error(ArgumentsError::TooMany))
            }

            if let Object::Number(n) = *lisp.eval_object(a)? {
                n
            } else {
                return Err(RustFuncError::new_args_error(ArgumentsError::WrongType))
            }
        },
        _ => 0.0,
    };

    std::process::exit(exit_code as i32);
}

// Display an object
fn print(lisp: &mut LispScope, arg: Object) -> RustFuncResult {
    let a = match arg {
        Object::Pair(a, b) => {
            if *b != Object::Nil {
                return Err(RustFuncError::new_args_error(ArgumentsError::TooMany))
            }
            lisp.eval_object(a)?
        },
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };

    println!("{}", a);

    Ok(Box::new(Object::Nil))
}

// Reads a line into objects
fn read(lisp: &mut LispScope, arg: Object) -> RustFuncResult {
    use std::io::{stdin, stdout, Write};

    let c = match arg {
        Object::Pair(a, b) => {
            if *b != Object::Nil {
                return Err(RustFuncError::new_args_error(ArgumentsError::TooMany))
            }
            match *lisp.eval_object(a)? {
                Object::Character(c) => c,
                _ => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
            }
        },
        Object::Nil => '>',
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };

    let stdin = stdin();
    let mut stdout = stdout();
    let mut input = String::new();

    print!("{} ", c);
    stdout.flush().unwrap();
    
    stdin.read_line(&mut input).unwrap();
    let objects = Object::eval(&input)?; // Evaluate tokens into objects
    let objects: Vec<Box<Object>> = objects.into_iter().map(Box::new).collect(); // Store objects on the heap

    // Read cannot return multiple objects, even if multiple objects were evaluated
    if !objects.is_empty() {
        Ok(objects[0].clone())
    } else {
        Ok(Box::new(Object::Nil))
    }
}

// For creating functions
fn func(_: &mut LispScope, arg: Object) -> RustFuncResult {
    let mut lisp_list_args;
    let mut func_body = Vec::new();

    match arg {
        Object::Pair(a, b) => {

	    match *a {
		Object::Pair(_,_) => lisp_list_args = a,
		Object::Nil => lisp_list_args = a,
		_ => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
	    }

	    let mut cur_object = *b;
	    
	    loop {
	    	match cur_object {
	    	    Object::Pair(a, b) => {
	    		func_body.push(*a);
	    
	    		cur_object = *b
	    	    },
	    	    Object::Nil => break,
	    	    _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
	    	}
	    }
        },
        Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    }; 

    let mut args: Vec<String> = Vec::new();

    loop {
        match *lisp_list_args {
            Object::Pair(a, b) => {
		if let Object::Symbol(s) = *a {
                    args.push(s);
		} else {
	    	    return Err(RustFuncError::new_args_error(ArgumentsError::WrongType));
		}

                lisp_list_args = b
            },
            Object::Nil => break,
            _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
        }
    }

    Ok(Box::new(Object::LispFunc(args, func_body)))
}
