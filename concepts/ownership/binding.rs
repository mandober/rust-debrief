// ! variables

// variable v of type T: v: T
let v: &'static str = "vars have type, the type of their value";

// must be declared and cannot be used without binding
let var = true;

// var name that begins with _ supresses the warning about unused variables
let _energy = 5;

// type can be explicitlly annotated
let v: u8 = 255;

// type inferred as i32 (default for integers)
let v = 255;

// variable bound to the unit type
let empty = ();

// overshadow a variable; change its type while at it
let empty = 10u8;

// `size_of_val(&T)` returns the size of a variable in bytes
println!("size: {} bytes", std::mem::size_of_val(&v));

// -------

// or turn off unused code detection with:
#![allow(unused_code)]

let x:u8 = 255;
let empty = (); // the value of the unit type ()


// var name that begins with _ supresses the warning about unused variables.
let _energy = 5;


// overshadow a variable
let x = 10u8;

// cannot change its type
//x = 305; //305 is u16 at least, not u8

// variables are mmutable. use mut
let mut y = 234_u8; // use _ to separate/group digits. it is ignored in numeric values.

//let n; // error: unable to infer enough type information about `_`; type annotations required

// assignment cannot be chained
let mut a = 5;
let mut b = 6;
let n = 7;
let a = b = n;
// here, b would be assign the value of n, but that expression
// would return () which would than be assigned to a.

// braces are expression and they return the last value (if not terminated by ;)
let n1 = {
    let a = 2;
    let b = 5;
    a + b   // <-- no semicolon!
};
println!("n1 is: {}", n1);  // prints: n1 is 7
