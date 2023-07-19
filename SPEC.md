# Lithia
lisp implemented in rust

## Name
Name comes from another name for Lithium Oxide

## Functions
* `quote` 
Returns whatever its given, used for when you don't want to evaluate something
    ```lisp
    (quote pi)
    ```
    ```
    => pi
    ```
* `exit`
Exit lisp interpreter, number may be provided for exit code
    ```lisp
    (exit 1)
    ```
* `=`, `set`
Sets a variable
    ```lisp
    (set foo "bar")
    ```
* `def`
Define a global
    ```lisp
    (def foo "bar")
    ```
* `defunc`
Define a global function
    ```lisp
	(defunc name (arg1 arg2)
		(body)
		(body)
		return_value)
    ```
* `eval`
Evaluates the given object and what it returns
    ```lisp
    (eval (quote pi))
    ```
    ```
    => 3.1415927
    ```
* `print`
Display an object
    ```lisp
    (print "hello world")
    ```
* `read`
Reads a line into objects
    ```lisp
    (read "> ")
    ```
* `include`
Reads a file and evaluates it, returning the last object
    ```lisp
    (include "hello.lisp")
    ```
* `while`
While first argument isn't nil, evaluates the rest
    ```lisp
    (def x 0)
    (while (!= x 3)
        (print x)
        (= x (+ x 1)))
    ```
    ```
    0
    1
    2
    ```
* `func`
Creates a function
	* Use:
	```lisp
	(func (arg1 arg2)
		(body)
		(body)
		return_value)
	```
* `car`
Gets the first element in a dot-pair
    ```lisp
    (car '(foo . bar))
    ```
    ```
    => foo
    ```
* `cdr`
Gets the second element in a dot-pair
    ```lisp
    (cdr '(foo . bar))
    ```
    ```
    => bar
    ```

### Maths functions
* `+`, `add`
* `-`, `sub`
* `*`, `mul`
* `/`, `div`
* `%`, `mod`
* `==`, `eq`
* `!=`, `ne`
