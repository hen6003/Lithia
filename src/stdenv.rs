use crate::lisp::Lisp;
use crate::object::Object;

impl Lisp {
    pub fn add_stdenv(&mut self) -> &mut Self {
        // Variables
        self.add_var("nil", Object::Nil);
        self.add_var("t", Object::True);
        
        // Functions
        self.add_func("quote", quote);
        self.add_func("exit", exit);
        self.add_func("eval", eval);
        self.add_func("loop", lisploop);
        self.add_func("while", lispwhile);
        self.add_func("print", print);
        self.add_func("read", read);

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
        self.add_func("nq", notequal);

        self
    }
}

fn divide(lisp: &mut Lisp, arg: Object) -> Object {
    let mut sum;
    let mut cur_object: Object = match arg {
        Object::Pair(a, b) => {
            sum = match lisp.eval_object(*a) {
                Object::Number(i) => i,
                _ => panic!("/ requires number arguments")
            };

            *b
        },
        _ => panic!("/ requires multiple arguments"),
    };
    
    loop {
        match cur_object {
            Object::Pair(a, b) => {
                sum /= match lisp.eval_object(*a) {
                    Object::Number(i) => i,
                    _ => panic!("/ requires number arguments")
                };

                cur_object = *b
            },
            Object::Nil => break Object::Number(sum),
            _ => panic!("/ doesn't accept dotted arguments"),
        }
    }
}

fn times(lisp: &mut Lisp, arg: Object) -> Object {
    let mut sum;
    let mut cur_object: Object = match arg {
        Object::Pair(a, b) => {
            sum = match lisp.eval_object(*a) {
                Object::Number(i) => i,
                _ => panic!("* requires number arguments")
            };

            *b
        },
        _ => panic!("* requires multiple arguments"),
    };
    
    loop {
        match cur_object {
            Object::Pair(a, b) => {
                sum *= match lisp.eval_object(*a) {
                    Object::Number(i) => i,
                    _ => panic!("* requires number arguments")
                };

                cur_object = *b
            },
            Object::Nil => break Object::Number(sum),
            _ => panic!("* doesn't accept dotted arguments"),
        }
    }
}

fn minus(lisp: &mut Lisp, arg: Object) -> Object {
    let mut sum;
    let mut cur_object: Object = match arg {
        Object::Pair(a, b) => {
            sum = match lisp.eval_object(*a) {
                Object::Number(i) => i,
                _ => panic!("- requires number arguments")
            };

            *b
        },
        _ => panic!("- requires multiple arguments"),
    };
    
    loop {
        match cur_object {
            Object::Pair(a, b) => {
                sum -= match lisp.eval_object(*a) {
                    Object::Number(i) => i,
                    _ => panic!("- requires number arguments")
                };

                cur_object = *b
            },
            Object::Nil => break Object::Number(sum),
            _ => panic!("- doesn't accept dotted arguments"),
        }
    }
}

fn add(lisp: &mut Lisp, arg: Object) -> Object {
    let mut sum;
    let mut cur_object: Object = match arg {
        Object::Pair(a, b) => {
            sum = match lisp.eval_object(*a) {
                Object::Number(i) => i,
                _ => panic!("+ requires number arguments")
            };

            *b
        },
        _ => panic!("+ requires multiple arguments"),
    };
    
    loop {
        match cur_object {
            Object::Pair(a, b) => {
                sum += match lisp.eval_object(*a) {
                    Object::Number(i) => i,
                    _ => panic!("+ requires number arguments")
                };

                cur_object = *b
            },
            Object::Nil => break Object::Number(sum),
            _ => panic!("+ doesn't accept dotted arguments"),
        }
    }
}

// Set variable
fn set(lisp: &mut Lisp, arg: Object) -> Object {
    let (symbol, data) = match arg {
        Object::Pair(a, b) => {
            match *b {
                Object::Pair(c, d) => {
                    if *d != Object::Nil {
                        panic!("set requires two arguments")
                    }

                    (a, c)
                },
                Object::Nil => panic!("set requires two arguments"),
                _ => panic!("set doesn't accept dotted arguments"),
            }
        },
        _ => panic!("set doesn't accept dotted arguments"),
    };

    if let Object::Symbol(symbol) = *symbol {
        let data = lisp.eval_object(*data);
        lisp.set_var(&symbol, data);
    } else {
        panic!("set requires a symbol for first arguments");
    }

    Object::Nil
}

// Evaluate an object and what it returns
fn eval(lisp: &mut Lisp, arg: Object) -> Object {
    let mut objects = Vec::new();
    let mut cur_object: Object = match arg {
        Object::Pair(a, b) => {
            objects.push(lisp.eval_object(*a));

            *b
        },
        _ => panic!("eval requires multiple arguments"),
    };
    
    loop {
        match cur_object {
            Object::Pair(a, b) => {
                objects.push(lisp.eval_object(*a));

                cur_object = *b
            },
            Object::Nil => break,
            _ => panic!("eval doesn't accept dotted arguments"),
        }
    }

    lisp.eval_objects(objects)
}

// Evaluates the given object forever
fn lisploop(lisp: &mut Lisp, arg: Object) -> Object {
    let mut objects = Vec::new();
    let mut cur_object: Object = match arg {
        Object::Pair(a, b) => {
            objects.push(*a);

            *b
        },
        _ => panic!("loop requires multiple arguments"),
    };
    
    loop {
        match cur_object {
            Object::Pair(a, b) => {
                objects.push(*a);

                cur_object = *b
            },
            Object::Nil => break,
            _ => panic!("loop doesn't accept dotted arguments"),
        }
    }

    loop {
        lisp.eval_objects(objects.clone());
    }
}

// Evaluates the given object forever
fn lispwhile(lisp: &mut Lisp, arg: Object) -> Object {
    let cond;
    let mut objects = Vec::new();
    let mut cur_object: Object = match arg {
        Object::Pair(a, b) => {
            cond = a;

            *b
        },
        _ => panic!("while requires multiple arguments"),
    };
    
    loop {
        match cur_object {
            Object::Pair(a, b) => {
                objects.push(*a);

                cur_object = *b
            },
            Object::Nil => break,
            _ => panic!("while doesn't accept dotted arguments"),
        }
    }

    while lisp.eval_object(*cond.clone()) != Object::Nil {
        lisp.eval_objects(objects.clone());
    }

    Object::Nil
}

// Evaluates the given object forever
fn equal(lisp: &mut Lisp, arg: Object) -> Object {
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
                        _ => panic!("== requires two arguments"),
                    }
                },
                Object::Nil => panic!("== requires two arguments"),
                _ => panic!("== doesn't accept dotted arguments"),
            }
        },
        _ => panic!("== requires two arguments"),
    }; 

    if lisp.eval_object(*first) == lisp.eval_object(*second) {
        Object::True
    } else {
        Object::Nil
    }
}

// Evaluates the given object forever
fn notequal(lisp: &mut Lisp, arg: Object) -> Object {
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
                        _ => panic!("!= requires two arguments"),
                    }
                },
                Object::Nil => panic!("!= requires two arguments"),
                _ => panic!("!= doesn't accept dotted arguments"),
            }
        },
        _ => panic!("!= requires two arguments"),
    }; 

    if lisp.eval_object(*first) != lisp.eval_object(*second) {
        Object::True
    } else {
        Object::Nil
    }
}

// Returns whatever its given, used for when you don't want to evaluate something
fn quote(_: &mut Lisp, arg: Object) -> Object {
    match arg {
        Object::Pair(a, b) => {
            if *b != Object::Nil {
                panic!("quote doesn't accept multiple arguments")
            }
            *a
        },
        _ => panic!("quote doesn't accept dotted arguments"),
    }
}

// Exit lisp interpreter, number may be provided for exit code
fn exit(lisp: &mut Lisp, arg: Object) -> Object {
    let exit_code = match arg {
        Object::Pair(a, b) => {
            if *b != Object::Nil {
                panic!("exit doesn't accept multiple arguments")
            }

            if let Object::Number(n) = lisp.eval_object(*a) {
                n
            } else {
                panic!("exit requires number arguments");
            }
        },
        _ => 0.0,
    };

    std::process::exit(exit_code as i32);
}

// Display an object
fn print(lisp: &mut Lisp, arg: Object) -> Object {
    let a = match arg {
        Object::Pair(a, b) => {
            if *b != Object::Nil {
                panic!("print doesn't accept multiple arguments")
            }
            lisp.eval_object(*a)
        },
        _ => panic!("print doesn't accept dotted arguments"),
    };

    println!("{}", a);

    Object::Nil
}

// Reads a line into objects
fn read(lisp: &mut Lisp, arg: Object) -> Object {
    use std::io::{stdin, stdout, Write};

    let c = match arg {
        Object::Pair(a, b) => {
            if *b != Object::Nil {
                panic!("read doesn't accept multiple arguments")
            }
            match lisp.eval_object(*a) {
                Object::Character(c) => c,
                _ => panic!("read doesn't accept non-characters")
            }
        },
        Object::Nil => '>',
        _ => panic!("read doesn't accept dotted arguments"),
    };

    let stdin = stdin();
    let mut stdout = stdout();
    let mut input = String::new();

    print!("{} ", c);
    stdout.flush().unwrap();
    
    stdin.read_line(&mut input).unwrap();
    let objects = Object::eval(&input);

    // Read cannot return multiple objects, even if multiple objects were evaluated
    if objects.len() > 0 {
        objects[0].clone()
    } else {
        Object::Nil
    }
}
