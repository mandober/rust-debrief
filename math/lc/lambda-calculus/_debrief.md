
- The first concept in lambda calculus is abstraction: λx.M
- It is analogous to defining a function
- This defines a function which takes an argument x and returns a term M.
- Functions in lambda calculus have no name.
- The second concept is that of invocation: M N
- It is essentially a function call.
- This binds the argument N to the term M
- A term is a defined function. A term may contain variables.
- A variable, may represent any term.
- A term may contain free variables or bound variables
- A bound variable is a variable which will be bound to some term passed as an argument. Eg, In λx.xy, x is a bound variable.
- A free variable is any variable in a term which is not bound to some other term. Eg, In λx.xy, y is a free variable.

