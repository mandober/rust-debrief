# Mutability

- variables are *immutable* by default which means once a variable is bound to a value, it cannot be rebound.
- variables can be made mutable with keyword `mut`
- when variable is first declared and only later bound, it doesn't have to be mutable for that single initial binding.
- It is not allowed to use an uninitialized variable
- a variable can be *overshadowed* with another variable of the same name. The new variable doesn't have to be of the same type as the old one.
- *constants* are values that are bound to an name (identifier)
  - declared with `const` keyword (in any scope), their type must be annotated
  - they can only be set to a constant expression
  - always immutable
  - naming convention: upper case (separated with underscores)

Values of the `Cell<T>` and `RefCell<T>` types may be mutated through shared references (i.e. the common `&T` type), whereas most Rust types can only be mutated through unique (`&mut T`) references. We say that `Cell<T>` and `RefCell<T>` provide **interior mutability**, in contrast with typical Rust types that exhibit __inherited mutability__.


```rust
// immutable variable
let x = 5;
x = 6; // error[E0384]: re-assignment of immutable variable `x`

// mutable variable
let mut x = 5;
x = 6; // ok

// declared, then bound
let o;  // declaration without `mut`
o = &6; // initial binding
println!("{}", o);

// uninitialized variable
let a: i32;
let b = a; // error[E0381]: use of possibly uninitialized variable: `a`

// overwhadowing
let x = 5;
let x += 2; // x is now 7

```
