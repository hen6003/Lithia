mod lisp;
mod object;
mod stdenv;

use lisp::Lisp;

fn main() {
    let mut lisp = Lisp::new();
    lisp.add_stdenv();

    //print_prompt();
    //for line in stdin.lock().lines() {
    //    println!("{}", lisp.eval(&line.unwrap()));

    //    print_prompt();
    //}

    lisp.eval("(loop (print (eval (read))))");
}
