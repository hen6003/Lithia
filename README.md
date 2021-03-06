# Lithia
lisp implemented in rust

## Name
Name comes from another name for Lithium Oxide

## Functions
* `quote` 
Returns whatever its given, used for when you don't want to evaluate something
* `exit`
Exit lisp interpreter, number may be provided for exit code
* `=`, `set`
Sets a variable
* `def`
Define a global
* `defunc`
Define a global function
* `eval`
Evaluates the given object and what it returns
* `print`
Display an object
* `read`
Reads a line into objects
* `include`
Reads a file and evaluates it, returning the last object
* `while`
While first argument isn't nil, evaluates the rest
* `read`
Prompts for input and converts it to objects
* `func`
Creates a function
	* Use:
	```lisp
	(func (arg1 arg2)
		(body)
		(body)
		return_value)
	```
* `car`, `first`
Gets the first element in a dot-pair
* `cdr`, `next`
Gets the second element in a dot-pair

### Math functions
* `+`, `add`
* `-`, `sub`
* `*`, `mul`
* `/`, `div`
* `%`, `mod`
* `==`, `eq`
* `!=`, `ne`
