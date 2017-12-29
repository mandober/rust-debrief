# `swap` function
https://doc.rust-lang.org/std/mem/fn.swap.html

Function `std::mem::swap` 1.0.0

Swaps the values at two mutable locations, without deinitializing either one.

```rust
pub fn swap<T>(x: &mut T, y: &mut T)```

Examples:

```rust
use std::mem;

let mut x = 5;
let mut y = 42;

mem::swap(&mut x, &mut y);

assert_eq!(42, x);
assert_eq!(5, y);
```
