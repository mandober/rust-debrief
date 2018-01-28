# mem module

- module: `std::mem`
- since: 1.0.0
- docs: [std](https://doc.rust-lang.org/std/mem/index.html)
- functions for manipulating memory and querying types properties.
- **Structs**
  - `Discriminant`: opaque type representing discriminant
- **Unions**
  - `ManuallyDrop`: inhibits destructor.
- **Functions**
  - `size_of`: size of a type in bytes.
  - `size_of_val`: size of pointed-to value's type.
  - `align_of`: minimum alignment of a type.
  - `align_of_val`: minimum alignment of the pointed-to value's type.
  - `discriminant`: uniquely identifies discriminant.
  - `swap`: swaps values, deinitializing neither.
  - `replace`: replaces and returns the old value, no deinitializing.
  - `drop`: drops the value.
  - `needs_drop`: whether dropping the values is needed.
  - `forget`: leaks the value
  - `zeroed`: ⚠ creates all zero bytes value.
  - `transmute`: ⚠ reinterprets the value as another type.
  - `transmute_copy`: ⚠ menipulating with type reinterpretation.
  - `uninitialized`: ⚠ bypasses memory initialization checks.
  - `unreachable`: ⚠ marks unreachable code



## swap

```rust
pub fn swap<T>(x: &mut T, y: &mut T)
```
Swaps the values at two mutable locations, deinitializing neither.
```rust
let mut x = 5;
let mut y = 42;
std::mem::swap(&mut x, &mut y);
assert_eq!(42, x);
assert_eq!(5, y);
```
