# error

- Module `std::error` since 1.0.0
- Trait `std::error::Error`, since 1.0.0
- Trait `Error`: base functionality for all errors in Rust.
- supertraits: `Debug` + `Display`
- required method: `description`
- provided method: `cause`



## Error trait

The `Error`trait provides base functionality for all errors in Rust. It represents the basic expectations for error values, `E` in `Result<T, E>`.

```rust
pub trait Error: Debug + Display {
  fn description(&self) -> &str;
  fn cause(&self) -> Option<&Error> { None }
}
```

At a minimum, errors must provide a `description`, but they may optionally provide additional detail (via `Display`) and `cause` chain information.

## description
- `description` method is required.
- It shoud provide a short description of the error. Avoid newlines or sentence-ending punctuation, to facilitate embedding in larger user-facing strings.

```rust
// signature
fn description(&self) -> &str;

// example
use std::error::Error;

match "xc".parse::<u32>() {
  Err(e) => println!("Error: {}", e.description()),
       _ => println!("Ok, no error occured."),
}
```

## cause
The `cause` method is generally used when errors cross "abstraction boundaries", i.e. when a one module must report an error that is "caused" by an error from a lower-level module. This setup makes it possible for the high-level module to provide its own errors that do not commit to any particular implementation, but also reveal some of its implementation for debugging via `cause` chains.

