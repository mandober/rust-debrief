# Debug
https://doc.rust-lang.org/std/fmt/trait.Debug.html

Trait `std::fmt::Debug` 1.0.0

Format trait for the ? character.


```rust
#[lang = "debug_trait"]
pub trait Debug {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>;
}
```

`Debug` should format the output in a programmer-facing, debugging context.

Generally speaking, you should just derive a `Debug` implementation.

When used with the alternate format specifier `#?`, the output is pretty-printed.


## Deriving

This trait can be used with `#[derive(Debug)]` if all fields implement `Debug`.

When derived for `struct`, it will use
the name of the struct,
then {,
then a comma-separated list of each field's name and `Debug` value,
then }.

When derived for `enum`, it will use 
the name of the variant
and, if applicable, (,
then the `Debug` values of the fields,
then ).


## Examples

### Deriving an implementation:

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
let origin = Point { x: 0, y: 0 };
println!("The origin is: {:?}", origin);
```

### Manually implementing:

```rust
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

let origin = Point { x: 0, y: 0 };
println!("The origin is: {:?}", origin);
```
This outputs: "The origin is: Point { x: 0, y: 0 }"


There are a number of `debug_*` methods on `Formatter` to help you with manual 
implementations, such as `debug_struct`.

Debug implementations using either derive or the debug builder API on `Formatter` 
support pretty printing using the alternate flag: {:#?}.

Pretty printing with `#?`:

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

let origin = Point { x: 0, y: 0 };

println!("The origin is: {:#?}", origin);Run
This outputs:

The origin is: Point {
    x: 0,
    y: 0
}
Required Methods

fn fmt(&self, f: &mut Formatter) -> Result<(), Error>[−]

Formats the value using the given formatter.