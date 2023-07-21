use alloc::{
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};

use std::{fs::File, io::Read};

use crate::{
    errors::*,
    lisp::{Lisp, LispBuilder},
    object::Object,
};

impl LispBuilder {
    pub fn add_env_sys(self) -> Result<Self, LispError> {
        // System functions
        self.add_func("include", include)?
            .add_func("read", read)?
            .add_func("exit", exit)?
            .add_func("print", print)?
            .add_func("print-raw", print_raw)
    }
}

// Exit lisp interpreter, number may be provided for exit code
fn exit(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    let exit_code = match &*arg {
        Object::Pair(a, b) => {
            if **b != Object::Nil {
                return Err(RustFuncError::new_args_error(ArgumentsError::TooMany));
            }

            if let Object::Number(n) = *lisp.eval_object(Rc::clone(a))? {
                n
            } else {
                return Err(RustFuncError::new_args_error(ArgumentsError::WrongType));
            }
        }
        _ => 0.0,
    };

    std::process::exit(exit_code as i32);
}

// Display an object
fn print(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    let a = match &*arg {
        Object::Pair(a, b) => {
            if **b != Object::Nil {
                return Err(RustFuncError::new_args_error(ArgumentsError::TooMany));
            }
            lisp.eval_object(Rc::clone(a))?
        }
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };

    println!("{}", a);

    Ok(Rc::new(Object::Nil))
}

// Display an objects raw internals
fn print_raw(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    let a = match &*arg {
        Object::Pair(a, b) => {
            if **b != Object::Nil {
                return Err(RustFuncError::new_args_error(ArgumentsError::TooMany));
            }
            lisp.eval_object(Rc::clone(a))?
        }
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };

    println!("{:?}", a);

    Ok(Rc::new(Object::Nil))
}

// Reads a line into objects
fn read(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    use std::io::{stdin, stdout, Write};

    let promptobject = match &*arg {
        Object::Pair(a, b) => {
            if **b != Object::Nil {
                return Err(RustFuncError::new_args_error(ArgumentsError::TooMany));
            }

            Rc::clone(a)
        }
        Object::Nil => match lisp.eval_object(Rc::new(Object::Symbol("PROMPT".to_string()))) {
            Ok(o) => o,
            Err(_) => Rc::new(Object::Nil),
        },
        _ => return Err(RustFuncError::new_args_error(ArgumentsError::DottedPair)),
    };

    let prompt = match &*lisp.eval_object(promptobject)? {
        Object::Character(c) => format!("{} ", c),
        Object::Nil => "> ".to_string(),
        v => {
            if let Object::Pair(_, _) = v {
                v.pair_list_to_string().unwrap_or_else(|_| "> ".to_string())
            } else {
                return Err(RustFuncError::new_args_error(ArgumentsError::WrongType));
            }
        }
    };

    let stdin = stdin();
    let mut stdout = stdout();
    let mut input = String::new();

    stdout.write_all(prompt.as_bytes()).unwrap();
    stdout.flush().unwrap();

    stdin.read_line(&mut input).unwrap();
    let objects = Object::eval(&input)?; // Evaluate tokens into objects
    let objects: Vec<Rc<Object>> = objects.into_iter().map(Rc::new).collect(); // Store objects on the heap

    // Read cannot return multiple objects, even if multiple objects were evaluated
    if !objects.is_empty() {
        Ok(objects[0].clone())
    } else {
        Ok(Rc::new(Object::Nil))
    }
}

// Include another file, causing execution to switch to that file
fn include(lisp: &mut Lisp, arg: Rc<Object>) -> RustFuncResult {
    let file = match &*arg {
        Object::Pair(a, b) => {
            if **b != Object::Nil {
                return Err(RustFuncError::new_args_error(ArgumentsError::TooMany));
            }

            lisp.eval_object(Rc::clone(a))?
        }
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
    let objects: Vec<Rc<Object>> = objects.into_iter().map(Rc::new).collect(); // Store objects on the heap

    let mut scope = Lisp::new(lisp.globals.clone());

    let mut ret = Rc::new(Object::Nil);
    for o in objects {
        ret = scope.eval_object(o)?;
    }

    Ok(ret)
}
