mod lisp;
mod object;
mod stdenv;

use std::io;
use std::io::prelude::*;
use lisp::Lisp;

fn print_prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn main() {
    let stdin = io::stdin();
    let mut lisp = Lisp::new();
    lisp.add_stdenv();

    print_prompt();
    for line in stdin.lock().lines() {
        println!("{}", lisp.eval(&line.unwrap()));

        print_prompt();
    }
}
