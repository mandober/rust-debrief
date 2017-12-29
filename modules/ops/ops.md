Module `std::ops`

[std::ops](https://doc.rust-lang.org/std/ops/index.html)

# Overloadable operators

- Implementing these traits allows overloading of certain operators.
- Only operators backed by traits can be overloaded.
  For example, the `+` can be overloaded through the `Add` trait, 
  but `=` cannot since it has no backing trait.

Many of the operators take their operands by value, so using these operators in 
generic code, requires some attention if values have to be reused as opposed to 
letting the operators consume them.

One option is to occasionally use `clone`. Another option is to rely on the types 
involved providing additional operator implementations for references. 

For example, for a user-defined type `T` which is supposed to support addition, 
it is probably a good idea to have both `T` and `&T` implement the traits `Add<T>` 
and `Add<&T>` so that generic code can be written without unnecessary cloning.


## Examples
This example creates a `Point` struct that implements `Add` and `Sub`
traits and then demonstrates adding and subtracting two `Points`.

```rust
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {x: self.x - other.x, y: self.y - other.y}
    }
}

assert_eq!(Point {x: 3, y: 3}, Point {x: 1, y: 0} + Point {x: 2, y: 3});
assert_eq!(Point {x: -1, y: -3}, Point {x: 1, y: 0} - Point {x: 2, y: 3});
```
