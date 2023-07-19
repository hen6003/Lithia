/// Example runner for lithia - running either a file or a repl
use std::{collections::HashMap, env, fs};

use lithia::Lisp;

fn main() {
    let args: Vec<String> = env::args().collect();

    let code = if args.len() > 1 {
        fs::read_to_string(&args[1]).unwrap()
    } else {
        "(while t (print (eval (read))))".to_string()
    };

    let mut globals = HashMap::new();
    let ret = Lisp::new(&mut globals)
        .add_stdenv()
        .unwrap()
        .add_sysenv()
        .unwrap()
        .eval(&code);

    match ret {
        Ok(_) => (),
        Err(e) => println!("\x1b[31m{}\x1b[0m", e),
    }
}
