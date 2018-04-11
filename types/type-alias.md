# Type aliases

A type alias defines a new name for an existing type. Type aliases are declared with the keyword type. Every value has a single, specific type, but may implement several different traits, or be compatible with several different type constraints.

For example, the following defines the type Point as a synonym for the type (u8, u8), the type of pairs of unsigned 8 bit integers:

```rust
type Point = (u8, u8);
let p: Point = (41, 68);
A type alias to an enum type cannot be used to qualify the constructors:


enum E { A }
type F = E;
let _: F = E::A;  // OK
// let _: F = F::A;  // Doesn't work


pub type Float = f64;
pub type Matrix<T = Float> = Matrix23<T>;
pub type Matrix<T = u8> = Matrix23<T>;
```
