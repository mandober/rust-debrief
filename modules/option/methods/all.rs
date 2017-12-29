// ! Option methods

/**
is_some
is_none
as_ref

*/


// Returns true if the option is a Some value.
fn is_some(&self) -> bool;
// example:
let x: Option<u32> = Some(2);
assert_eq!(x.is_some(), true);
// example:
let x: Option<u32> = None;
assert_eq!(x.is_some(), false);



// Returns true if the option is a None value.
fn is_none(&self) -> bool
// example:
let x: Option<u32> = Some(2);
assert_eq!(x.is_none(), false);
// example:
let x: Option<u32> = None;
assert_eq!(x.is_none(), true);



// Converts from Option<T> to Option<&T>
fn as_ref(&self) -> Option<&T>;
/**
Convert an Option<String> into an Option<usize>, preserving the original.
The map method takes the self argument by value, consuming the original,
so this technique uses as_ref to first take an Option
to a reference to the value inside the original.
*/
let num_as_str: Option<String> = Some("10".to_string());
// First, cast Option<String> to Option<&String> with as_ref,
// then consume that with map, leaving num_as_str on the stack.
let num_as_int: Option<usize> = num_as_str.as_ref().map(|n| n.len());
println!("still can print num_as_str: {:?}", num_as_str);



// Converts from Option<T> to Option<&mut T>
fn as_mut(&mut self) -> Option<&mut T>
// Examples
let mut x = Some(2);
match x.as_mut() {
    Some(v) => *v = 42,
    None => {},
}
assert_eq!(x, Some(42));



// Unwraps an option, yielding the content of a Some.
fn expect(self, msg: &str) -> T
// Panics
// Panics if the value is a None with a custom panic message provided by msg.
// example
let x = Some("value");
assert_eq!(x.expect("the world is ending"), "value");
// example
let x: Option<&str> = None;
x.expect("the world is ending"); // panics with `the world is ending`Run



// Moves the value v out of the Option<T> if it is Some(v).
fn unwrap(self) -> T;
/**
In general, because this function may panic, its use is discouraged.
Instead, prefer to use pattern matching and handle the None case explicitly.

Panics
Panics if the self value equals None.
*/
// example
let x = Some("air");
assert_eq!(x.unwrap(), "air");
// example
let x: Option<&str> = None;
assert_eq!(x.unwrap(), "air"); // failsRun



// Returns the contained value or a default.
fn unwrap_or(self, def: T) -> T
// Examples
assert_eq!(Some("car").unwrap_or("bike"), "car");
assert_eq!(None.unwrap_or("bike"), "bike");



// Returns the contained value or computes it from a closure.
fn unwrap_or_else<F>(self, f: F) -> T
where F: FnOnce() -> T,
// Examples
let k = 10;
assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4);
assert_eq!(None.unwrap_or_else(|| 2 * k), 20);



// Maps an Option<T> to Option<U> by applying a function to a contained value.
fn map<U, F>(self, f: F) -> Option<U>
where F: FnOnce(T) -> U,
// Examples
// Convert an Option<String> into an Option<usize>, consuming the original:
let maybe_some_string = Some(String::from("Hello, World!"));
// `Option::map` takes self *by value*, consuming `maybe_some_string`
let maybe_some_len = maybe_some_string.map(|s| s.len());
assert_eq!(maybe_some_len, Some(13));



// Applies a function to the contained value (if any), or returns a default (if not).
fn map_or<U, F>(self, default: U, f: F) -> U
where F: FnOnce(T) -> U,
// Examples
let x = Some("foo");
assert_eq!(x.map_or(42, |v| v.len()), 3);
// Examples
let x: Option<&str> = None;
assert_eq!(x.map_or(42, |v| v.len()), 42);



// Applies a function to the contained value (if any), or computes a default (if not).
fn map_or_else<U, D, F>(self, default: D, f: F) -> U
where
    D: FnOnce() -> U,
    F: FnOnce(T) -> U,
// Examples
let k = 21;
let x = Some("foo");
assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 3);
let x: Option<&str> = None;
assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 42);




// Transforms the Option<T> into a Result<T, E>, mapping Some(v) to Ok(v) and None to Err(err).
fn ok_or<E>(self, err: E) -> Result<T, E>
// Examples
let x = Some("foo");
assert_eq!(x.ok_or(0), Ok("foo"));
let x: Option<&str> = None;
assert_eq!(x.ok_or(0), Err(0));



// Transforms the Option<T> into a Result<T, E>, mapping Some(v) to Ok(v) and None to Err(err()).
fn ok_or_else<E, F>(self, err: F) -> Result<T, E>
where F: FnOnce() -> E,
// Examples
let x = Some("foo");
assert_eq!(x.ok_or_else(|| 0), Ok("foo"));
let x: Option<&str> = None;
assert_eq!(x.ok_or_else(|| 0), Err(0));



// Returns an iterator over the possibly contained value.
fn iter(&self) -> Iter<T>
// Examples
let x = Some(4);
assert_eq!(x.iter().next(), Some(&4));
let x: Option<u32> = None;
assert_eq!(x.iter().next(), None);



// Returns a mutable iterator over the possibly contained value.
fn iter_mut(&mut self) -> IterMut<T>
// Examples
let mut x = Some(4);
match x.iter_mut().next() {
    Some(v) => *v = 42,
    None => {},
}
assert_eq!(x, Some(42));
let mut x: Option<u32> = None;
assert_eq!(x.iter_mut().next(), None);




// Returns None if the option is None, otherwise returns optb.
fn and<U>(self, optb: Option<U>) -> Option<U>
// Examples
let x = Some(2);
let y: Option<&str> = None;
assert_eq!(x.and(y), None);
//
let x: Option<u32> = None;
let y = Some("foo");
assert_eq!(x.and(y), None);
//
let x = Some(2);
let y = Some("foo");
assert_eq!(x.and(y), Some("foo"));
//
let x: Option<u32> = None;
let y: Option<&str> = None;
assert_eq!(x.and(y), None);





// Returns None if the option is None, otherwise calls f with the wrapped value and returns the result.
// Some languages call this operation flatmap.
fn and_then<U, F>(self, f: F) -> Option<U>
where    F: FnOnce(T) -> Option<U>,
// Examples
fn sq(x: u32) -> Option<u32> { Some(x * x) }
fn nope(_: u32) -> Option<u32> { None }
//
assert_eq!(Some(2).and_then(sq).and_then(sq), Some(16));
assert_eq!(Some(2).and_then(sq).and_then(nope), None);
assert_eq!(Some(2).and_then(nope).and_then(sq), None);
assert_eq!(None.and_then(sq).and_then(sq), None);



// Returns the option if it contains a value, otherwise returns optb.
fn or(self, optb: Option<T>) -> Option<T>
// Examples
let x = Some(2);
let y = None;
assert_eq!(x.or(y), Some(2));
//
let x = None;
let y = Some(100);
assert_eq!(x.or(y), Some(100));
//
let x = Some(2);
let y = Some(100);
assert_eq!(x.or(y), Some(2));
//
let x: Option<u32> = None;
let y = None;
assert_eq!(x.or(y), None);





fn or_else<F>(self, f: F) -> Option<T>
where    F: FnOnce() -> Option<T>,
// Returns the option if it contains a value, otherwise calls f and returns the result.
fn nobody() -> Option<&'static str> { None }
fn vikings() -> Option<&'static str> { Some("vikings") }
assert_eq!(Some("barbarians").or_else(vikings), Some("barbarians"));
assert_eq!(None.or_else(vikings), Some("vikings"));
assert_eq!(None.or_else(nobody), None);



fn get_or_insert(&mut self, v: T) -> &mut T // 1.20.0
// Inserts v into the option if it is None, then returns a mutable reference to
// the contained value.
let mut x = None;
{
    let y: &mut u32 = x.get_or_insert(5);
    assert_eq!(y, &5);

    *y = 7;
}
assert_eq!(x, Some(7));



fn get_or_insert_with<F>(&mut self, f: F) -> &mut T
where    F: FnOnce() -> T // 1.20.0
// Inserts a value computed from f into the option if it is None, then returns
// a mutable reference to the contained value.
let mut x = None;
{
    let y: &mut u32 = x.get_or_insert_with(|| 5);
    assert_eq!(y, &5);
    *y = 7;
}
assert_eq!(x, Some(7));




fn take(&mut self) -> Option<T>;
// Takes the Some variant out of the option, leaving a None variant in its place.
let mut x: Option<u32> = Some(2);
let     z: Option<u32> = x.take();
assert_eq!(x, None);
assert_eq!(z, Some(2));

let mut x: Option<u32> = None;
x.take();
assert_eq!(x, None);





// * impl<'a, T> Option<&'a T> where T: Clone

fn cloned(self) -> Option<T>

Maps an Option<&T> to an Option<T> by cloning the contents of the option.

Examples

let x = 12;
let opt_x = Some(&x);
assert_eq!(opt_x, Some(&12));
let cloned = opt_x.cloned();
assert_eq!(cloned, Some(12));Run



// * impl<T> Option<T> where T: Default

fn unwrap_or_default(self) -> T

Returns the contained value or a default

Consumes the self argument then, if Some, returns the contained value, otherwise if None, returns the default value for that type.

Examples

Convert a string to an integer, turning poorly-formed strings into 0 (the default value for integers). parse converts a string to any other type that implements FromStr, returning None on error.

let good_year_from_input = "1909";
let bad_year_from_input = "190blarg";
let good_year = good_year_from_input.parse().ok().unwrap_or_default();
let bad_year = bad_year_from_input.parse().ok().unwrap_or_default();

assert_eq!(1909, good_year);
assert_eq!(0, bad_year);Run



// ! Trait Implementations

impl<T> From<T> for Option<T>;
// 1.12.0

fn from(val: T) -> Option<T>
// Performs the conversion.


// * impl<T> Clone for Option<T> where T: Clone

fn clone(&self) -> Option<T>
// Returns a copy of the value.

fn clone_from(&mut self, source: &Self)
// Performs copy-assignment from source.

// * impl<T> PartialOrd<Option<T>> for Option<T>
where T: PartialOrd<T>,

fn partial_cmp(&self, __arg_0: &Option<T>) -> Option<Ordering>

This method returns an ordering between self and other values if one exists. Read more

fn lt(&self, __arg_0: &Option<T>) -> bool

This method tests less than (for self and other) and is used by the < operator. Read more

fn le(&self, __arg_0: &Option<T>) -> bool

This method tests less than or equal to (for self and other) and is used by the <= operator. Read more

fn gt(&self, __arg_0: &Option<T>) -> bool

This method tests greater than (for self and other) and is used by the > operator. Read more

fn ge(&self, __arg_0: &Option<T>) -> bool

This method tests greater than or equal to (for self and other) and is used by the >= operator. Read more

impl<T> Eq for Option<T>
where
    T: Eq,

impl<T> Default for Option<T>
fn default() -> Option<T>

Returns None.

impl<T> Copy for Option<T>
where
    T: Copy,

impl<T> Debug for Option<T>
where
    T: Debug,

fn fmt(&self, __arg_0: &mut Formatter) -> Result<(), Error>

Formats the value using the given formatter.

impl<A, V> FromIterator<Option<A>> for Option<V>
where
    V: FromIterator<A>,

fn from_iter<I>(iter: I) -> Option<V>
where
    I: IntoIterator<Item = Option<A>>,


Takes each element in the Iterator: if it is None, no further elements are taken, and the None is returned. Should no None occur, a container with the values of each Option is returned.

Here is an example which increments every integer in a vector, checking for overflow:

use std::u16;

let v = vec![1, 2];
let res: Option<Vec<u16>> = v.iter().map(|&x: &u16|
    if x == u16::MAX { None }
    else { Some(x + 1) }
).collect();
assert!(res == Some(vec![2, 3]));Run
impl<T> Ord for Option<T>
where
    T: Ord,

fn cmp(&self, __arg_0: &Option<T>) -> Ordering

This method returns an Ordering between self and other. Read more

fn max(self, other: Self) -> Self1.21.0

Compares and returns the maximum of two values. Read more

fn min(self, other: Self) -> Self1.21.0

Compares and returns the minimum of two values. Read more

impl<T> PartialEq<Option<T>> for Option<T>
where
    T: PartialEq<T>,

fn eq(&self, __arg_0: &Option<T>) -> bool

This method tests for self and other values to be equal, and is used by ==. Read more

fn ne(&self, __arg_0: &Option<T>) -> bool

This method tests for !=.

impl<T> Hash for Option<T>
where
    T: Hash,

fn hash<__HT>(&self, __arg_0: &mut __HT)
where
    __HT: Hasher,


Feeds this value into the given [Hasher]. Read more

fn hash_slice<H>(data: &[Self], state: &mut H)
where
    H: Hasher,
1.3.0

Feeds a slice of this type into the given [Hasher]. Read more

impl<T> IntoIterator for Option<T>
type Item = T

The type of the elements being iterated over.

type IntoIter = IntoIter<T>

Which kind of iterator are we turning this into?

fn into_iter(self) -> IntoIter<T>

Returns a consuming iterator over the possibly contained value.

Examples

let x = Some("string");
let v: Vec<&str> = x.into_iter().collect();
assert_eq!(v, ["string"]);

let x = None;
let v: Vec<&str> = x.into_iter().collect();
assert!(v.is_empty());Run
impl<'a, T> IntoIterator for &'a Option<T>1.4.0
type Item = &'a T

The type of the elements being iterated over.

type IntoIter = Iter<'a, T>

Which kind of iterator are we turning this into?

fn into_iter(self) -> Iter<'a, T>

Creates an iterator from a value. Read more

impl<'a, T> IntoIterator for &'a mut Option<T>1.4.0
type Item = &'a mut T

The type of the elements being iterated over.

type IntoIter = IterMut<'a, T>

Which kind of iterator are we turning this into?

fn into_iter(self) -> IterMut<'a, T>

Creates an iterator from a value. Read more