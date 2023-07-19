/// Example runner for lithia - running either a file or a repl
use std::{env, fs};

use lithia::lisp::LispBuilder;

fn main() {
    let args: Vec<String> = env::args().collect();

    let code = if args.len() > 1 {
        fs::read_to_string(&args[1]).unwrap()
    } else {
        "(while t (print (eval (read))))".to_string()
    };

    let mut lisp = LispBuilder::new().add_default_envs().unwrap().build();

    let ret = lisp.eval(&code);

    match ret {
        Ok(_) => (),
        Err(e) => println!("\x1b[31m{}\x1b[0m", e),
    }
}
