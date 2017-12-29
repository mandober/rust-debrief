# Default trait
https://doc.rust-lang.org/std/default/trait.Default.html

Trait `std::default::Default` 1.0.0

A trait for giving a type a useful default value.

```rust
pub trait Default {
    fn default() -> Self;
}
```

Sometimes, you want to fall back to some kind of default value, and don't 
particularly care what it is. This comes up often with structs that define a 
set of options:

```rust
struct SomeOptions {
    foo: i32,
    bar: f32,
}
```

How can we define some default values? You can use Default:

```rust
#[derive(Default)]
struct SomeOptions {
    foo: i32,
    bar: f32,
}

fn main() {
    let options: SomeOptions = Default::default();
}
```
Now, you get all of the default values. Rust implements `Default` for various 
primitives types.

If you want to override a particular option, but still retain the other defaults:

```rust
fn main() {
    let options = SomeOptions { foo: 42, ..Default::default() };
}
```


## Derivable

This trait can be used with `#[derive]` if all of the type's fields implement 
`Default`. When derived, it will use the default value for each field's type.


## implement Default

Provide an implementation for the `default()` method that returns the value of 
your type that should be the default:

```rust
enum Kind {
    A,
    B,
    C,
}

impl Default for Kind {
    fn default() -> Kind { Kind::A }
}
```

## Examples
```rust
#[derive(Default)]
struct SomeOptions {
    foo: i32,
    bar: f32,
}
```


## Required Methods

```rust
fn default() -> Self[−]
```
Returns the "default value" for a type.

Default values are often some kind of initial value, identity value, or anything 
else that may make sense as a default.


Examples:

Using built-in default values:

```rust
let i: i8 = Default::default();
let (x, y): (Option<String>, f64) = Default::default();
let (a, b, (c, d)): (i32, u32, (bool, bool)) = Default::default();
```

Making your own:

```rust
enum Kind {
    A,
    B,
    C,
}

impl Default for Kind {
    fn default() -> Kind { Kind::A }
}
```