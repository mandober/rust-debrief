// * is_ok
// fn is_ok(&self) -> bool
// Returns true if the result is Ok.
let x: Result<i32, &str> = Ok(-3);
assert_eq!(x.is_ok(), true);
let x: Result<i32, &str> = Err("Some error message");
assert_eq!(x.is_ok(), false);

// * is_err
// fn is_err(&self) -> bool
// Returns true if the result is Err.
let x: Result<i32, &str> = Ok(-3);
assert_eq!(x.is_err(), false);
let x: Result<i32, &str> = Err("Some error message");
assert_eq!(x.is_err(), true);


// * unwrap_or
// fn unwrap_or(self, optb: T) -> T
// Unwraps a result, yielding the content of an Ok. Else, it returns optb.
let optb = 2;

let x: Result<u32, &str> = Ok(9);
assert_eq!(x.unwrap_or(optb), 9);

let x: Result<u32, &str> = Err("error");
assert_eq!(x.unwrap_or(optb), optb);


// * unwrap_or_else
// fn unwrap_or_else<F>(self, op: F) -> T
// where  F: FnOnce(E) -> T,
// Unwraps a result, yielding the content of an Ok.
// If the value is an Err then it calls op with its value.
fn count(x: &str) -> usize { x.len() }
assert_eq!(Ok(2).unwrap_or_else(count), 2);
assert_eq!(Err("foo").unwrap_or_else(count), 3);


// * unwrap
// fn unwrap(self) -> T
// Unwraps a result, yielding the content of an Ok.
// Panics if the value is an Err, with a panic message provided by the Err's value.
let x: Result<u32, &str> = Ok(2);
assert_eq!(x.unwrap(), 2);

let x: Result<u32, &str> = Err("emergency failure");
x.unwrap(); // panics with `emergency failure`


// * expect
// fn expect(self, msg: &str) -> T
// Unwraps a result, yielding the content of an Ok.
// Panics if the value is an Err, with a panic message including the passed
// message, and the content of the Err.
let x: Result<u32, &str> = Err("emergency failure");
x.expect("Testing expect");
//: panics with `Testing expect: emergency failure`
