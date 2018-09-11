// ! Functions

// Swaps the values at two mutable locations,
// without deinitializing either one.
use std::mem::swap;
pub fn swap<T>(x: &mut T, y: &mut T){}



// simple
fn main() {} // takes nothing, returns nothing
fn noop() -> () {} // returns unit type (nothing)
fn simple(num: f32) {}
fn simple(num: u8) -> u8 {}
fn simple(num: i64) -> bool {}
fn max_i32(list: &[i32]) -> i32 {} // borrows slice
fn max_char(list: &[char]) -> char {} // borrows slice

// takes ownership and returns it
let _i: i32 = 314;
let _i = take_int(_i);
fn take_int (num: i32) -> i32 { num }

// borrows int
let _i: i16 = 314;
let _i = take_int(&_i);
fn take_int (num: &i16) -> i16 { num }

// vec of Strings
// let args: Vec<String> = env::args().collect();
let args: Vec<String> = vec!["prog", "p1", "p2"]
let _config = parse(&args);
// param is slice into String view
fn parse(args: &[String]) -> Result<Config, &str> { }
// param is ref to a vec of Strings (same as above)
fn parse(args: &Vec<String>) -> Result<Config, &str> { }


// takes tuple, returns tuple
fn tuple_pair(pair: (i32, bool)) -> (bool, i32) {}
// borrows a String, ret Result<T,E>
fn fh(file: &String) -> Result<String, std::io::Error> {}
// mut borrow
fn change(somestr: &mut String){}
// takes and returns, Some(i32) value or None
fn plus_one(x: Option<i32>) -> Option<i32> {}
fn twice(f: &Fn(i32) -> i32, x:i32) -> i32 {}
// takes a slice of u8's, returns u8
fn fx (list: &[u8]) -> u8 {}



// METHODS
// takes instance
fn call(self) {}
// borrows instance immutably
fn call(&self) {}
// borrows instance mutably
fn call(&mut self) {}
// takes instance and a string slice
fn add(self, s: &str) -> String {}
// borrows instance and a custom type ref
fn hld(&self, rect: &Custom) -> bool {}


// GENERIC TYPE PARAMETERS
fn fx<T>(param: T) -> T {}
fn fx<T>(list: &[T]) -> T {}


// TRAIT BOUNDS
fn fx<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 { }
// Trait bounds with `where` clause
fn fx<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}

fn largest<T: PartialOrd>(list: &[T]) -> T {}

// lifetimes
fn skip_prefix<'a, 'b>(line: &'a str, prefix: &'b mut str) -> &'a str {}
