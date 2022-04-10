// Defining the standard functions and variables that exist in the language

#[cfg(feature = "system")]
use std::fs::File;
#[cfg(feature = "system")]
use std::io::Read;

use crate::lisp::Lisp;
use crate::object::Object;
use crate::errors::*;

impl<'a> Lisp<'a> {
    #[cfg(feature = "system")]
    pub fn add_sysenv(&mut self) -> Result<&mut Self, LispError> {
	// System functions
        self.add_func(true, "include", include)?;
        self.add_func(true, "read", read)?;
	self.add_func(true, "exit", exit)?;
        self.add_func(true, "print", print)?;

	Ok(self)
    }

    pub fn add_stdenv(&mut self) -> Result<&mut Self, LispError> {
        // Variables
        self.add_var(true, "nil", Box::new(Object::Nil))?;
        self.add_var(true, "t", Box::new(Object::True))?;
        self.add_var(true, "pi", Box::new(Object::Number(std::f32::consts::PI)))?;
        
        // Functions
        self.add_func(true, "quote", quote)?; 
        self.add_func(true, "eval", eval)?;
        self.add_func(true, "while", lispwhile)?;
        self.add_func(true, "func", func)?;

        self.add_func(true, "first", car)?;
        self.add_func(true, "next", cdr)?;
        self.add_func(true, "car", car)?;
        self.add_func(true, "cdr", cdr)?;
	
        self.add_func(true, "=", set)?;
        self.add_func(true, "def", define)?;
        self.add_func(true, "defunc", defunc)?;

        // Math functions
        self.add_func(true, "+", add)?;
        self.add_func(true, "-", minus)?;
        self.add_func(true, "*", times)?;
        self.add_func(true, "/", divide)?;
        self.add_func(true, "%", modulus)?;
        self.add_func(true, "==", equal)?;
        self.add_func(true, "!=", notequal)?;
        
        // Non-symbol names
        self.add_func(true, "set", set)?;
        self.add_func(true, "add", add)?;
        self.add_func(true, "sub", minus)?;
        self.add_func(true, "mul", times)?;
        self.add_func(true, "div", divide)?;
        self.add_func(true, "mod", modulus)?;
        self.add_func(true, "eq", equal)?;
        self.add_func(true, "ne", notequal)?;

	#[cfg(feature = "system")]
	self.add_sysenv()?;

	Ok(self)
    }
}

fn modulus(lisp: &mut Lisp, arg: Object) -> RustFuncResult {
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
                sum %= match *lisp.eval_object(a)? {
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


fn divide(lisp: &mut Lisp, arg: Object) -> RustFuncResult {
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

fn times(lisp: &mut Lisp, arg: Object) -> RustFuncResult {
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

fn minus(lisp: &mut Lisp, arg: Object) -> RustFuncResult {
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

fn add(lisp: &mut Lisp, arg: Object) -> RustFuncResult {
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
fn set(lisp: &mut Lisp, arg: Object) -> RustFuncResult {
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
        lisp.set_var(&symbol, data)?;
    } else {
        return Err(RustFuncError::new_args_error(ArgumentsError::WrongType))
    }

    Ok(Box::new(Object::Nil))
}

// Define global
fn define(lisp: &mut Lisp, arg: Object) -> RustFuncResult {
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
        lisp.add_var(true, &symbol, data)?;
    } else {
        return Err(RustFuncError::new_args_error(ArgumentsError::WrongType))
    }

    Ok(Box::new(Object::Nil))
}

// Define global function
fn defunc(lisp: &mut Lisp, arg: Object) -> RustFuncResult {
    let (symbol, function) = match arg {
        Object::Pair(a, b) => (a, func(lisp, *b)?),
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };

    if let Object::Symbol(symbol) = *symbol {
        lisp.add_var(true, &symbol, function)?;
    } else {
        return Err(RustFuncError::new_args_error(ArgumentsError::WrongType))
    }

    Ok(Box::new(Object::Nil))
}

// Evaluate an object and what it returns
fn eval(lisp: &mut Lisp, arg: Object) -> RustFuncResult {
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
fn lispwhile(lisp: &mut Lisp, arg: Object) -> RustFuncResult {
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

    lisp.scope_create();

    while *lisp.eval_object(cond.clone())? != Object::Nil {
        lisp.eval_objects(objects.clone())?;
    }

    lisp.scope_end();

    Ok(Box::new(Object::Nil))
}

// Evaluates the given object forever
fn equal(lisp: &mut Lisp, arg: Object) -> RustFuncResult {
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
fn notequal(lisp: &mut Lisp, arg: Object) -> RustFuncResult {
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
fn quote(_: &mut Lisp, arg: Object) -> RustFuncResult {
    match arg {
        Object::Pair(a, b) => {
            if *b != Object::Nil {
                Err(RustFuncError::new_args_error(ArgumentsError::TooMany))
            } else {
                Ok(a)
            }
        },
	Object::Nil => Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    }
}

// Exit lisp interpreter, number may be provided for exit code
#[cfg(feature = "system")]
fn exit(lisp: &mut Lisp, arg: Object) -> RustFuncResult {
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
#[cfg(feature = "system")]
fn print(lisp: &mut Lisp, arg: Object) -> RustFuncResult {
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
#[cfg(feature = "system")]
fn read(lisp: &mut Lisp, arg: Object) -> RustFuncResult {
    use std::io::{stdin, stdout, Write};

    let promptobject = match arg {
        Object::Pair(a, b) => {
            if *b != Object::Nil {
                return Err(RustFuncError::new_args_error(ArgumentsError::TooMany))
            }

	    a
        },
        Object::Nil => match lisp.eval_object(Box::new(Object::Symbol("PROMPT".to_string()))) {
	    Ok(o) => o,
	    Err(_) => Box::new(Object::Nil),
	},
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };

    let prompt = match *lisp.eval_object(promptobject)? {
	//TODO handle string
        Object::Character(c) => format!("{} ", c),
	Object::Nil => "> ".to_string(),
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
    };

    let stdin = stdin();
    let mut stdout = stdout();
    let mut input = String::new();

    stdout.write_all(prompt.as_bytes()).unwrap();
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
fn func(_: &mut Lisp, arg: Object) -> RustFuncResult {
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

// Get first item in a list
fn car(lisp: &mut Lisp, arg: Object) -> RustFuncResult {
    match arg {
        Object::Pair(a, b) => {
	    if *b != Object::Nil {
		return Err(RustFuncError::new_args_error(ArgumentsError::TooMany))
	    }

	    match *lisp.eval_object(a)? {
		Object::Pair(a, _) => Ok(a),
		_ => Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
	    }
	},
	Object::Nil => Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    }
}

// Get next item in a list
fn cdr(lisp: &mut Lisp, arg: Object) -> RustFuncResult {
    match arg {
        Object::Pair(a, b) => {
	    if *b != Object::Nil {
		return Err(RustFuncError::new_args_error(ArgumentsError::TooMany))
	    }

	    match *lisp.eval_object(a)? {
		Object::Pair(_, b) => Ok(b),
		_ => Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
	    }
	},
	Object::Nil => Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    }
}

// Include another file, causing execution to switch to that file
#[cfg(feature = "system")]
fn include(lisp: &mut Lisp, arg: Object) -> RustFuncResult {
    let file = match arg {
        Object::Pair(a, b) => {
	    if *b != Object::Nil {
		return Err(RustFuncError::new_args_error(ArgumentsError::TooMany))
	    }

	    lisp.eval_object(a)?
	},
	Object::Nil => return Err(RustFuncError::new_args_error(ArgumentsError::NotEnough)),
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };

    let file = match file.pair_list_to_string() {
	Ok(s) => s,
	Err(_) => return Err(RustFuncError::new_args_error(ArgumentsError::WrongType)),
    };

    let mut file = match File::open(&file) {
        Err(why) => panic!("couldn't open: {}", why),
        Ok(file) => file,
    };

    let mut data = String::new();
    if let Err(why) = file.read_to_string(&mut data) {
        panic!("couldn't read: {}", why);
    }
  
    let objects = Object::eval(&data)?; // Evaluate tokens into objects
    let objects: Vec<Box<Object>> = objects.into_iter().map(Box::new).collect(); // Store objects on the heap

    let mut scope = Lisp::new(lisp.globals);

    let mut ret = Box::new(Object::Nil);
    for o in objects {
	ret = scope.eval_object(o)?;
    }

    Ok(ret)
}
