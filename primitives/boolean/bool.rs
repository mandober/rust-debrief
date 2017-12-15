// ! bool
/**
The bool represents a value, which could only be either true or false.

If you cast a bool into an integer, true will be 1 and false will be 0.

bool implements various traits, such as BitAnd, BitOr, Not, etc., which
allow us to perform boolean operations using &, | and !.

`if` always demands a bool value.

assert!, being an important macro in testing,
checks whether an expression returns true.
*/

let b = true & false | false;
assert!(!b);
