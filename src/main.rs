use std::io;
use std::io::prelude::*;

mod lisp;

fn print_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn main() {
    let stdin = io::stdin();
    let mut lisp = lisp::Lisp::new();

    print_prompt();
    for line in stdin.lock().lines() {
        println!("{}", lisp.eval(&line.unwrap()));

        print_prompt();
    }
}
