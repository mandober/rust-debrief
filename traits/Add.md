# Add trait


- `std::ops::Add`, since: 1.0.0
- online [doc](https://doc.rust-lang.org/std/ops/trait.Add.html)
- the addition operator: `+`


```rust
#[lang = "add"]
pub trait Add<RHS = Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}
```

Note that `RHS = Self` by default, but this is not mandatory. 
For example, `std::time::SystemTime` implements `Add<Duration>`, which permits 
operations of the form `SystemTime = SystemTime + Duration`.



## Addable points:
This example creates a `Point` struct and implements `Add` trait.

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });
```

## Implementing Add with generics
Example of the same `Point` struct implementing the `Add` trait using generics.

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

// Notice that the implementation uses the associated type `Output`.
impl<T: Add<Output=T>> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });
```