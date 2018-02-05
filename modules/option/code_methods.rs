// Option method examples

/**
is_some
is_none
as_ref

*/


// is_some
fn is_some(&self) -> bool;
// returns true if the option is present.
let x: Option<u32> = Some(2);
assert_eq!(x.is_some(), true);
let y: Option<u32> = None;
assert_eq!(y.is_some(), false);


// is_none
fn is_none(&self) -> bool
// returns true if the option is absent.
let x: Option<u32> = Some(2);
let y: Option<u32> = None;
assert_eq!(y.is_none(), true);


// as_ref
// Converts from Option<T> to Option<&T>. Weakens the type.
fn as_ref(&self) -> Option<&T>;
// Convert Option<String> into Option<usize>, preserving the original.
// Since map takes self by value, as_ref is used to make an Option containing
// a ref to the value inside self.
let num_as_str: Option<String> = Some("10".to_string());
// First, cast Option<String> to Option<&String> with as_ref,
// then consume that with map, leaving num_as_str on the stack.
let num_as_int: Option<usize> = num_as_str.as_ref().map(|n| n.len());
println!("still can print num_as_str: {:?}", num_as_str);


// as_mut
fn as_mut(&mut self) -> Option<&mut T>;
// Converts from Option<T> to Option<&mut T>
let mut x = Some(2);
match x.as_mut() {
    Some(v) => *v = 42,
    None => {},
}
assert_eq!(x, Some(42));


// expect
fn expect(self, msg: &str) -> T;
// Unwraps an option, yielding the content of a Some. Panics if None.
let x = Some("value");
assert_eq!(x.expect("the world is ending"), "value");
let x: Option<&str> = None;
x.expect("the world is ending"); // panic


// unwrap
fn unwrap(self) -> T;
// Moves the value out of Option<T>. Panics if None.
let x = Some("air");
assert_eq!(x.unwrap(), "air");
let x: Option<&str> = None;
assert_eq!(x.unwrap(), "air"); // panic


// unwrap_or
fn unwrap_or(self, def: T) -> T;
// Returns the contained value or a default.
assert_eq!(Some("car").unwrap_or("bike"), "car");
assert_eq!(None.unwrap_or("bike"), "bike");


// unwrap_or_else
fn unwrap_or_else<F>(self, f: F) -> T where F: FnOnce() -> T;
// Returns the contained value or computes it from a closure.
let k = 10;
assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4);
assert_eq!(None.unwrap_or_else(|| 2 * k), 20);


// map
// Maps an Option<T> to Option<U> by applying a function to a contained value.
fn map<U, F>(self, f: F) -> Option<U> where F: FnOnce(T) -> U;
// Convert an Option<String> into an Option<usize>, consuming the original:
let maybe_some_string = Some(String::from("Hello, World!"));
// `Option::map` takes self by value, consuming `maybe_some_string`
let maybe_some_len = maybe_some_string.map(|s| s.len());
assert_eq!(maybe_some_len, Some(13));


// map_or
fn map_or<U, F>(self, default: U, f: F) -> U where F: FnOnce(T) -> U;
// Applies a function to the contained value (if any), or returns a default.
let x = Some("foo");
assert_eq!(x.map_or(42, |v| v.len()), 3);
let x: Option<&str> = None;
assert_eq!(x.map_or(42, |v| v.len()), 42);


// map_or_else
fn map_or_else<U, D, F>(self, default: D, f: F) -> U
  where D: FnOnce() -> U, F: FnOnce(T) -> U;
// Applies a function to the contained value, or computes a default.
let k = 21;
let x = Some("foo");
assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 3);
let x: Option<&str> = None;
assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 42);


// ok_or
fn ok_or<E>(self, err: E) -> Result<T, E>;
// Transforms the Option<T> into a Result<T, E>
// mapping Some(v) to Ok(v) and None to Err(err).
let x = Some("foo");
assert_eq!(x.ok_or(0), Ok("foo"));
let x: Option<&str> = None;
assert_eq!(x.ok_or(0), Err(0));


// ok_or_else
fn ok_or_else<E, F>(self, err: F) -> Result<T, E> where F: FnOnce() -> E;
// Transforms the Option<T> into a Result<T, E> (Some to Ok, None to Err)
let x = Some("foo");
assert_eq!(x.ok_or_else(|| 0), Ok("foo"));
let x: Option<&str> = None;
assert_eq!(x.ok_or_else(|| 0), Err(0));


// iter
fn iter(&self) -> Iter<T>;
// Returns an iterator over the possibly contained value.
let x = Some(4);
assert_eq!(x.iter().next(), Some(&4));
let x: Option<u32> = None;
assert_eq!(x.iter().next(), None);


// iter_mut
fn iter_mut(&mut self) -> IterMut<T>;
// Returns a mutable iterator over the possibly contained value.
let mut x = Some(4);
match x.iter_mut().next() {
    Some(v) => *v = 42,
    None => {},
}
assert_eq!(x, Some(42));
let mut x: Option<u32> = None;
assert_eq!(x.iter_mut().next(), None);


// and
fn and<U>(self, optb: Option<U>) -> Option<U>;
// Returns None if the option is None, otherwise returns optb.
let x = Some(2);
let y: Option<&str> = None;
assert_eq!(x.and(y), None);


// and_then (flatmap)
fn and_then<U, F>(self, f: F) -> Option<U> where F: FnOnce(T) -> Option<U>;
// Returns None if the option is None, otherwise calls
// fn with the wrapped value and returns the result
fn sq(x: u32) -> Option<u32> { Some(x * x) }
fn nope(_: u32) -> Option<u32> { None }
assert_eq!(Some(2).and_then(sq).and_then(sq), Some(16));
assert_eq!(Some(2).and_then(sq).and_then(nope), None);
assert_eq!(Some(2).and_then(nope).and_then(sq), None);
assert_eq!(None.and_then(sq).and_then(sq), None);


// or
fn or(self, optb: Option<T>) -> Option<T>;
// Returns the option if it contains a value, otherwise returns optb.
let x = Some(2);
let y = None;
assert_eq!(x.or(y), Some(2));
let x = None;
let y = Some(100);
assert_eq!(x.or(y), Some(100));
let x = Some(2);
let y = Some(100);
assert_eq!(x.or(y), Some(2));
let x: Option<u32> = None;
let y = None;
assert_eq!(x.or(y), None);


// or_else
fn or_else<F>(self, f: F) -> Option<T> where F: FnOnce() -> Option<T>;
// Returns the option if it contains a value, otherwise calls f and returns the result.
fn nobody() -> Option<&'static str> { None }
fn vikings() -> Option<&'static str> { Some("vikings") }
assert_eq!(Some("barbarians").or_else(vikings), Some("barbarians"));
assert_eq!(None.or_else(vikings), Some("vikings"));
assert_eq!(None.or_else(nobody), None);


// get_or_insert
fn get_or_insert(&mut self, v: T) -> &mut T // 1.20
// Inserts v into the option if it is None,
// then returns a mutable reference to the contained value.
let mut x = None;
{   let y: &mut u32 = x.get_or_insert(5);
    assert_eq!(y, &5);
    *y = 7;
}
assert_eq!(x, Some(7));


// get_or_insert_with
fn get_or_insert_with<F>(&mut self, f: F) -> &mut T where F: FnOnce()->T //1.20
// Inserts a value computed from f into the option if it is None, then returns
// a mutable reference to the contained value.
let mut x = None;
{   let y: &mut u32 = x.get_or_insert_with(|| 5);
    assert_eq!(y, &5);
    *y = 7;
}
assert_eq!(x, Some(7));


// take
fn take(&mut self) -> Option<T>;
// Takes the Some variant out of the option, leaving a None variant in its place.
let mut x: Option<u32> = Some(2);
let z: Option<u32> = x.take();
assert_eq!(x, None);
assert_eq!(z, Some(2));
let mut x: Option<u32> = None;
x.take();
assert_eq!(x, None);



// cloned
impl<'a, T> Option<&'a T> where T: Clone {
  fn cloned(self) -> Option<T>;
  // Maps an Option<&T> to an Option<T> by cloning the contents of option.
  let x = 12;
  let opt_x = Some(&x);
  assert_eq!(opt_x, Some(&12));
  let cloned = opt_x.cloned();
  assert_eq!(cloned, Some(12));
}


// cloned
// NIGHTLY: https://github.com/rust-lang/rust/issues/43738
impl<'a, T> Option<&'a mut T> where T: Clone {
  fn cloned(self) -> Option<T>;
  // Maps an Option<&mut T> to an Option<T> by cloning the contents of option.
  let x = 12;
  let opt_x = Some(&x);
  assert_eq!(opt_x, Some(&12));
  let cloned = opt_x.cloned();
  assert_eq!(cloned, Some(12));
}


// unwrap_or_default
impl<'a, T> Option<&'a T> where T: Default {
  // Returns the contained value or a default
  fn unwrap_or_default(self) -> T;
  // Consumes the self argument then, if Some, returns the contained value,
  // otherwise if None, returns the default value for that type.
  // Convert a string to an integer, turning poorly-formed strings into 0
  // (the default value for integers). parse converts a string to any other type
  // that implements FromStr, returning None on error.
  let good_year_from_input = "1909";
  let bad_year_from_input = "190blarg";
  let good_year = good_year_from_input.parse().ok().unwrap_or_default();
  let bad_year = bad_year_from_input.parse().ok().unwrap_or_default();
  assert_eq!(1909, good_year);
  assert_eq!(0, bad_year);
}
