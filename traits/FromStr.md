# `FromStr` trait
https://doc.rust-lang.org/std/str/trait.FromStr.html

Trait `std::str::FromStr` 1.0.0

```rust
pub trait FromStr {
    type Err;
    fn from_str(s: &str) -> Result<Self, Self::Err>;
}
```
A trait to abstract the idea of creating
a new instance of a type from a string.

Method `from_str()` in `FromStr` trait is often
used implicitly, through str's method `parse()`.


## Examples

Basic implementation of `FromStr` on an example Point type:

```rust
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
                                 .split(",")
                                 .collect();
        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;
        Ok(Point { x: x_fromstr, y: y_fromstr })
    }
}

let p = Point::from_str("(1,2)");
assert_eq!(p.unwrap(), Point{ x: 1, y: 2} )
```


## Associated Types

```rust
type Err
```
The associated error which can be returned from parsing.


## Required Methods

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Parses a string s to return a value of this type.

If parsing succeeds, return the value inside Ok, otherwise when the
string is ill-formatted return an error specific to the inside Err.
The error type is specific to implementation of the trait.

Examples:

Basic usage with `i32`, a type that implements `FromStr`:

```rust
use std::str::FromStr;
let s = "5";
let x = i32::from_str(s).unwrap();
assert_eq!(5, x);
```


## Implementors

```rust
impl FromStr for bool
  type Err = ParseBoolError;
impl FromStr for f32
  type Err = ParseFloatError;
impl FromStr for i16
  type Err = ParseIntError;
impl FromStr for u8
  type Err = ParseIntError;
impl FromStr for u32
  type Err = ParseIntError;
impl FromStr for u64
  type Err = ParseIntError;
impl FromStr for char
  type Err = ParseCharError;
impl FromStr for i32
  type Err = ParseIntError;
impl FromStr for u16
  type Err = ParseIntError;
impl FromStr for i128
  type Err = ParseIntError;
impl FromStr for isize
  type Err = ParseIntError;
impl FromStr for u128
  type Err = ParseIntError;
impl FromStr for i8
  type Err = ParseIntError;
impl FromStr for usize
  type Err = ParseIntError;
impl FromStr for i64
  type Err = ParseIntError;
impl FromStr for f64
  type Err = ParseFloatError;
impl FromStr for String
  type Err = ParseError;
impl FromStr for IpAddr
  type Err = AddrParseError;
impl FromStr for Ipv4Addr
  type Err = AddrParseError;
impl FromStr for Ipv6Addr
  type Err = AddrParseError;
impl FromStr for SocketAddrV4
  type Err = AddrParseError;
impl FromStr for SocketAddrV6
  type Err = AddrParseError;
impl FromStr for SocketAddr
  type Err = AddrParseError;
impl FromStr for TokenStream
```
