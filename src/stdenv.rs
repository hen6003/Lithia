use crate::lisp::Lisp;
use crate::object::Object;

impl Lisp {
    pub fn add_stdenv(&mut self) {
        // Variables
        self.add_var("nil", Object::Nil);
        
        // Functions
        self.add_func("quote", quote);
        self.add_func("+", add);

        #[cfg(debug_assertions)]
        self.add_func("internal", internal);
    }
}

// Adds...
fn add(arg: Object) -> Object {
    let mut sum;
    let mut cur_object: Object = match arg {
        Object::Pair(a, b) => {
            sum = match *a {
                Object::Integer(i) => i,
                _ => panic!("+ requires integer arguments")
            };

            *b
        },
        _ => panic!("+ requires multiple arguments {:?}", arg),
    };
    
    loop {
        match cur_object {
            Object::Pair(a, b) => {
                sum += match *a {
                    Object::Integer(i) => i,
                    _ => panic!("+ requires integer arguments")
                };

                cur_object = *b
            },
            Object::Nil => break Object::Integer(sum),
            _ => panic!("+ doesn't accept dotted arguments"),
        }
    }
}

// Returns whatever its given, used for when you don't want to evaluate something
fn quote(arg: Object) -> Object {
    arg
}

// Display internal version of an object
#[cfg(debug_assertions)]
fn internal(arg: Object) -> Object {
    let a = match arg {
        Object::Pair(a, n) => {
            if *n != Object::Nil {
                panic!("internal doesn't accept multiple arguments")
            }
            a
        },
        _ => panic!("internal doesn't accept dotted arguments"),
    };

    println!("{:?}", a);

    Object::Nil
}
