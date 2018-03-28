# Error handling


## Option

`Some` is a value constructor for the `Option` type.
Like a function `fn<T>(value: T) -> Option<T>`

`None` is a value constructor for the `Option` type.
Like a function that takes no arguments: `fn<T>() -> Option<T>`


## Result
There are numerous (proper) `Result` types in std and numerous `Result` type aliases for the proper Result types.

```rust
// proper `Result`s
use std::result::Result;
use std::fmt::Result;
use std::io::Result;
// etc.
```

### Result alias
In the standard library, you may frequently see types like `Result<i32>`, even though `Result<T, E>` should have 2 type parameters. This is achieved by
defining a type alias that fixes one of the type parameters to a particular type.

In the case of `Result`, the fixed type is the error type. First, using a concrete error type: instead of generic type parameter `E` in `Result<T, E>`, the error type is fixed to a concrete type `ParseIntError`, resulting in `Result<T, ParseIntError>`. Second, a type alias is created:

```rust
use std::num::ParseIntError;
use std::result;
// Result alias with fixed error:
// Result<T, E> => Result<T, ParseIntError> => Result<T> (alias)
type Result<T> = result::Result<T, ParseIntError>;
// return type is Result alias
fn parse_int(p: &str) -> Result<i32> {}
```

## Error structs
The error type parameter, `E`, in `Result<T, E>` enum is usually a struct. For example `parse` function returns `Result<T, ParseIntError>` with error type being a `ParseIntError` struct. This struct has a single field `kind` of type `IntErrorKind`, which is an enum that describes all kinds of possible errors resulting from parsing a string into a number.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseIntError {
  kind: IntErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum IntErrorKind {
  Empty,
  InvalidDigit,
  Overflow,
  Underflow,
}

impl ParseIntError {
  pub fn error_description(&self) -> &str {
    match self.kind {
      IntErrorKind::Empty        => "cannot parse integer from empty string",
      IntErrorKind::InvalidDigit => "invalid digit found in string",
      IntErrorKind::Overflow     => "number too large to fit in target type",
      IntErrorKind::Underflow    => "number too small to fit in target type",
    }
  }
}
```

## Composition of distinct error types

```rust
use std::env;
fn main() {
    let mut argv = env::args();
    let arg: String = argv.nth(1).unwrap(); // error 1
    let n: i32 = arg.parse().unwrap(); // error 2
    println!("{}", 2 * n);
}
```

Here, `nth()` produces an `Option` while `parse()` produces a `Result`. These aren’t directly composable. When faced with both an Option and a Result, the solution is usually to convert the Option to a Result.

```rust
use std::env;

fn double_arg(mut argv: env::Args) -> Result<i32, String> {
  argv.nth(1)
    // get Some(v) or return a String describing the error
    .ok_or("No param supplied".to_owned())
    // get Ok(v) or convert parse's Err(error) into a String to return
    .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
}

fn main() {
  match double_arg(env::args()) {
    Ok(n)    => println!("{}", n),
    Err(err) => println!("Error: {}", err),
  }
}
```

`Option::ok_or` combinator is one way to convert an Option into a Result. The conversion requires specifing an error to use if Option is None.

`Result::map_err` maps a function on to the error of Result. If the Result is an Ok variant, it is returned unmodified. It is used here because it is necessary the error types are the same. Since we converted `Option<String>` (from `nth` method) to `Result<String, String>`, we must also convert the `ParseIntError` (from `parse` method) to a String.


## Own error type
We define our own error type that represents errors with structured data by defining an enum. An error is either `io::Error` or `num::ParseIntError`:

```rust
use std::io::{self, Read};
use std::num;
use std::fs::File;

// our custum error type that will hold 2 different error types
#[derive(Debug)]
enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

// convert io::Error to CliError
impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::IoError(error)
    }
}

// convert num::ParseIntError to CliError
impl From<num::ParseIntError> for CliError {
    fn from(error: num::ParseIntError) -> Self {
        CliError::ParseError(error)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<u8, CliError> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let num: u8 = contents.trim().parse()?;
    Ok(num)
}

pub fn run() {
    match open_and_parse_file("num.txt") {
    Ok(v)  => println!("ok: {}", v),
    Err(e) => eprintln!("er: {:?}", e),
  }
}
```

## Error trait

```rust
use std::fmt::{Debug, Display};

trait Error: Debug + Display {
  /// A short description of the error.
  fn description(&self) -> &str;
  /// The lower level cause of this error, if any.
  fn cause(&self) -> Option<&Error> { None }
}
```

The trait is impl for all types that represent errors. It allows to do:
- Obtain a `Debug` representation of the error.
- Obtain a user-facing `Display` representation of the error.
- Obtain a short description of the error (`description` method).
- Inspect the causal chain of an error, if one exists (`cause` method).

The power of `Error` comes from the fact that all error types implement this trait, which means errors can be treated as trait objects, either `Box<Error>` or `&Error`. The `cause` method returns `&Error`, which is a trait object.


The `try!` macro or `?` encapsulates 3 things simultaneously:
- Case analysis
- Control flow (early returns)
- Error type conversion
