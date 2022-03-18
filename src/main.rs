mod lisp;
mod object;
mod stdenv;

use lisp::Lisp;

fn main() {
    Lisp::new()
        .add_stdenv()
        .eval("(loop (print (eval (read))))");
}
