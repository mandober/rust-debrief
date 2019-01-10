# From trait


Trait `std::convert::From` 1.0.0

Simple and safe type conversions into `Self`. It is the reciprocal of `Into`.

```rust
pub trait From<T> {
    fn from(T) -> Self;
}
```

Useful for error handling, closely related to the `?` operator.

When constructing a fn that is capable of failing the return type will
generally be of the form `Result<T, E>`. The `From` trait allows for 
simplification of error handling by providing a means of returning a single error 
type that encapsulates numerous possible erroneous situations.

This trait is not limited to error handling, rather the general case for this
trait would be in any type conversions to have an explicit definition of how
they are performed.

Note: **this trait must not fail**. If the conversion can fail, use `TryFrom` 
or a dedicated method which returns an `Option<T>` or a `Result<T, E>`.


## Generic Implementations

`From<T>` for `U` implies `Into<U>` for `T`
`from` is reflexive, which means that `From<T>` for `T` is implemented


## Required Methods

`from` - performs the conversion.

```rust
fn from(T) -> Self
```

Examples:

String implements `From<&str>`

```rust
let string = "hello".to_string();
let other_string = String::from("hello");
assert_eq!(string, other_string);
```


Example usage for error handling

```rust
use std::io::{self, Read};
use std::num;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::IoError(error)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(error: num::ParseIntError) -> Self {
        CliError::ParseError(error)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    let mut file = std::fs::File::open("test")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}
```
