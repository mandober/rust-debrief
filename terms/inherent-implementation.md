

An inherent implementation is defined as the sequence of the impl keyword, generic type declarations, a path to a nominal type, a where clause, and a bracketed set of associable items. The nominal type is called the implementing type and the associable items are the associated items to the implementing type.

Inherent implementations associate the associated items to the implementing type.

The associated item has a path of a path to the implementing type followed by the associate item's path component.

- Inherent implementations cannot contain associated type aliases.
- A type can have multiple inherent implementations.
- The implementing type must be defined within the same crate.


```rust
// a type
struct Point {x: i32, y: i32}

// and its inherent impl block
impl Point {
  fn log(&self) {
    println!("Point is at ({}, {})", self.x, self.y);
  }
}
```
