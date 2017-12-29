# Module `std::error` 1.0.0

Traits for working with Errors.

The `Error` trait
Error is a trait representing the basic expectations for error values,
i.e. values of type `E` in `Result<T, E>`. 

At a minimum, errors must provide a description, but they may optionally provide 
additional detail (via `Display`) and `cause` chain information:

```rust
use std::fmt::Display;

trait Error: Display {
    fn description(&self) -> &str;

    fn cause(&self) -> Option<&Error> { None }
}
```

The `cause` method is generally used when errors cross "abstraction boundaries",
i.e. when a one module must report an error that is "caused" by an error from a 
lower-level module. This setup makes it possible for the high-level module to 
provide its own errors that do not commit to any particular implementation, but 
also reveal some of its implementation for debugging via `cause` chains.


## Traits
`Error` Base functionality for all errors in Rust.
