# Boolean


- name: The Boolean type
- type group: numeric
- type: `bool`
- cardinality: 2
- values: `true`, `false`
- literals: ✔  
- ref: `&bool` (&true)
- mut ref: `&mut bool` (&mut true)
- kind: primitive, scalar, concrete, fixed  
- sized: ✔    
- size: 1b  
- storage: stack (`Copy` type)  
- std::module: ✗  
- sample: `let b: bool = true`  
- traits: `Copy`, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Debug, Display, Default, Hash, Eq, PartialEq, Ord, PartialOrd, FromStr  


https://doc.rust-lang.org/std/primitive.bool.html

Primitive Type `bool` 1.0.0

The boolean type.

The `bool` represents a value, which could only be either true or false.
If you cast a `bool` into an integer, true will be 1 and false will be 0.


bool implements various traits, such as BitAnd, BitOr, Not, etc., which
allow us to perform boolean operations using &, | and !.
`if` always demands a bool value.
assert!, being an important macro in testing, checks whether an expression returns true.


## Basic usage

`bool` implements various traits, such as `BitAnd`, `BitOr`, `Not`, etc.,
which allow us to perform boolean operations using `&`, `|` and `!`.

`if` always demands a `bool` value.

`assert!`, being an important macro in testing, 
checks whether an expression returns true.

```rust
let bool_val = true & false | false;
assert!(!bool_val);
```

## Examples

A trivial example of the usage of `bool`

```rust
let praise_the_borrow_checker = true;

// using the `if` conditional
if praise_the_borrow_checker {
    println!("oh, yeah!");
} else {
    println!("what?!!");
}

// ... or, a match pattern
match praise_the_borrow_checker {
    true => println!("keep praising!"),
    false => println!("you should praise!"),
}
```

Also, since `bool` implements the `Copy` trait, we don't have to worry about 
the move semantics (just like the integer and float primitives).

Now an example of `bool` cast to integer type:

```rust
assert_eq!(true as i32, 1);
assert_eq!(false as i32, 0);
```


## Trait Implementations

```rust
impl PartialOrd<bool> for bool;
```
```rust
    fn partial_cmp(&self, other: &bool) -> Option<Ordering>;
    // This method returns an ordering between 
    // self and other values if one exists.

    fn lt(&self, other: &Rhs) -> bool;
    // This method tests less than (for self and other) 
    // and is used by the < operator.

    fn le(&self, other: &Rhs) -> bool;
    // This method tests less than or equal to (for 
    // self and other) and is used by the <= operator.

    fn gt(&self, other: &Rhs) -> bool;
    // This method tests greater than (for self 
    // and other) and is used by the > operator.

    fn ge(&self, other: &Rhs) -> bool;
    // This method tests greater than or equal to 
    // (for self and other) and is used by the >= operator.
```

```rust
impl BitXorAssign<bool> for bool; // 1.8.0
```
```rust
    fn bitxor_assign(&mut self, other: bool)
    // Performs the ^= operation.
```

```rust
impl BitAndAssign<bool> for bool; // 1.8.0
```
```rust
    fn bitand_assign(&mut self, other: bool)
    // Performs the &= operation.
```

```rust
impl Eq for bool
```

```rust
impl BitOr<bool> for bool;
type Output = bool;
// The resulting type after applying the | operator.
```
```rust
    fn bitor(self, rhs: bool) -> bool
    // Performs the | operation.
```

```rust
impl<'a> BitOr<bool> for &'a bool
type Output = <bool as BitOr<bool>>::Output
// The resulting type after applying the | operator.
```
```rust
    fn bitor(self, other: bool) -> <bool as BitOr<bool>>::Output
    // Performs the | operation.
```

```rust
impl<'a> BitOr<&'a bool> for bool
type Output = <bool as BitOr<bool>>::Output
// The resulting type after applying the | operator.
```

```rust
fn bitor(self, other: &'a bool) -> <bool as BitOr<bool>>::Output
// Performs the | operation.

impl<'a, 'b> BitOr<&'a bool> for &'b bool
type Output = <bool as BitOr<bool>>::Output

// The resulting type after applying the | operator.

fn bitor(self, other: &'a bool) -> <bool as BitOr<bool>>::Output

// Performs the | operation.

impl Default for bool
fn default() -> bool

// Returns the default value of false

impl Debug for bool
fn fmt(&self, f: &mut Formatter) -> Result<(), Error>

// Formats the value using the given formatter.

impl BitAnd<bool> for bool
type Output = bool

// The resulting type after applying the & operator.

fn bitand(self, rhs: bool) -> bool

// Performs the & operation.

impl<'a> BitAnd<&'a bool> for bool
type Output = <bool as BitAnd<bool>>::Output

// The resulting type after applying the & operator.

fn bitand(self, other: &'a bool) -> <bool as BitAnd<bool>>::Output

// Performs the & operation.

impl<'a, 'b> BitAnd<&'a bool> for &'b bool
type Output = <bool as BitAnd<bool>>::Output

// The resulting type after applying the & operator.

fn bitand(self, other: &'a bool) -> <bool as BitAnd<bool>>::Output

// Performs the & operation.

impl<'a> BitAnd<bool> for &'a bool
type Output = <bool as BitAnd<bool>>::Output

// The resulting type after applying the & operator.

fn bitand(self, other: bool) -> <bool as BitAnd<bool>>::Output

// Performs the & operation.

impl BitOrAssign<bool> for bool1.8.0	
fn bitor_assign(&mut self, other: bool)

// Performs the |= operation.

impl Ord for bool
fn cmp(&self, other: &bool) -> Ordering

// This method returns an Ordering between self and other. Read more

fn max(self, other: Self) -> Self1.21.0	

// Compares and returns the maximum of two values. Read more

fn min(self, other: Self) -> Self1.21.0	

// Compares and returns the minimum of two values. Read more

impl PartialEq<bool> for bool
fn eq(&self, other: &bool) -> bool

// This method tests for self and other values to be equal, and is used by ==. Read more

fn ne(&self, other: &bool) -> bool

// This method tests for !=.

impl Display for bool
fn fmt(&self, f: &mut Formatter) -> Result<(), Error>

// Formats the value using the given formatter. Read more

impl Hash for bool
fn hash<H>(&self, state: &mut H) 
where H: Hasher, 
// Feeds this value into the given [Hasher]. Read more

fn hash_slice<H>(data: &[Self], state: &mut H) 
where H: Hasher // 1.3.0	
// Feeds a slice of this type into the given [Hasher]. Read more

impl FromStr for bool
type Err = ParseBoolError

// The associated error which can be returned from parsing.

fn from_str(s: &str) -> Result<bool, ParseBoolError>

Parse a bool from a string.

Yields a Result<bool, ParseBoolError>, because s may or may not actually be parseable.

// * Examples

use std::str::FromStr;

assert_eq!(FromStr::from_str("true"), Ok(true));
assert_eq!(FromStr::from_str("false"), Ok(false));
assert!(<bool as FromStr>::from_str("not even a boolean").is_err());Run
Note, in many cases, the .parse() method on str is more proper.

assert_eq!("true".parse(), Ok(true));
assert_eq!("false".parse(), Ok(false));
assert!("not even a boolean".parse::<bool>().is_err());Run
impl BitXor<bool> for bool
type Output = bool

// The resulting type after applying the ^ operator.

fn bitxor(self, other: bool) -> bool

// Performs the ^ operation.

impl<'a> BitXor<&'a bool> for bool
type Output = <bool as BitXor<bool>>::Output

// The resulting type after applying the ^ operator.

fn bitxor(self, other: &'a bool) -> <bool as BitXor<bool>>::Output

// Performs the ^ operation.

impl<'a> BitXor<bool> for &'a bool
type Output = <bool as BitXor<bool>>::Output

// The resulting type after applying the ^ operator.

fn bitxor(self, other: bool) -> <bool as BitXor<bool>>::Output

// Performs the ^ operation.

impl<'a, 'b> BitXor<&'a bool> for &'b bool
type Output = <bool as BitXor<bool>>::Output

// The resulting type after applying the ^ operator.

fn bitxor(self, other: &'a bool) -> <bool as BitXor<bool>>::Output

// Performs the ^ operation.

impl<'a> Not for &'a bool
type Output = <bool as Not>::Output

// The resulting type after applying the ! operator.

fn not(self) -> <bool as Not>::Output

// Performs the unary ! operation.

impl Not for bool
type Output = bool

// The resulting type after applying the ! operator.

fn not(self) -> bool

// Performs the unary ! operation.
```
