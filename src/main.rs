mod lisp;
mod object;
mod stdenv;

use std::env;
use std::fs;
use lisp::Lisp;

fn main() {
    let args: Vec<String> = env::args().collect();

    let code = if args.len() > 1 {
        fs::read_to_string(&args[1]).unwrap()
    } else {
        "(loop (print (eval (read))))".to_string()
    };

    Lisp::new()
        .add_stdenv()
        .eval(&code);
}
