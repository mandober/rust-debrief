# Struct `std::io::Error` 1.0.0

The error type for IO operations of the Read, Write, Seek, and associated traits.

```rust
pub struct Error { /* fields omitted */ }


// from src: https://doc.rust-lang.org/src/std/io/error.rs.html#67-69

#[derive(Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct Error {
    repr: Repr,
}

enum Repr {
    Os(i32),
    Simple(ErrorKind),
    Custom(Box<Custom>),
}

#[derive(Debug)]
struct Custom {
    kind: ErrorKind,
    error: Box<error::Error+Send+Sync>,
}
```

Errors mostly originate from the underlying OS, but custom instances of Error 
can be created with crafted error messages and a particular value of ErrorKind.


## Methods
`new`
`last_os_error`
`from_raw_os_error`
`raw_os_error`
`get_ref`
`get_mut`
`into_inner`
`kind`

## Trait Implementations
`From`, `Debug`, `Error`, `Display`




## Methods

impl `Error`

```rust
impl Error {
    pub fn new<E>(kind: ErrorKind, error: E) -> Error
        where E: Into<Box<Error + Send + Sync>>;
}
```
Creates a new IO error from a known kind of error as well as an arbitrary error payload.

This function is used to generically create I/O errors which do not originate 
from the OS itself. The error argument is an arbitrary payload which will be 
contained in this Error.

```rust
use std::io::{Error, ErrorKind};

// errors can be created from strings
let custom_error = Error::new(ErrorKind::Other, "oh no!");

// errors can also be created from other errors
let custom_error2 = Error::new(ErrorKind::Interrupted, custom_error);
```

```rust
impl Error {
    pub fn last_os_error() -> Error
}
```

Returns an error representing the last OS error which occurred.

This function reads the value of errno for the target platform (e.g. GetLastError on Windows) and will return a corresponding instance of Error for the error code.

```rust
use std::io::Error;

println!("last OS error: {:?}", Error::last_os_error());
```


```rust
pub fn from_raw_os_error(code: i32) -> Error
```

Creates a new instance of an Error from a particular OS error code.

On Linux:

```rust
use std::io;

let error = io::Error::from_raw_os_error(98);
assert_eq!(error.kind(), io::ErrorKind::AddrInUse);
```

On Windows:

```rust
use std::io;

let error = io::Error::from_raw_os_error(10048);
assert_eq!(error.kind(), io::ErrorKind::AddrInUse);
```


```rust
pub fn raw_os_error(&self) -> Option<i32>
```

Returns the OS error that this error represents (if any).

If this Error was constructed via last_os_error or from_raw_os_error, then this function will return Some, otherwise it will return None.

Examples

```rust
use std::io::{Error, ErrorKind};

fn print_os_error(err: &Error) {
    if let Some(raw_os_err) = err.raw_os_error() {
        println!("raw OS error: {:?}", raw_os_err);
    } else {
        println!("Not an OS error");
    }
}

fn main() {
    // Will print "raw OS error: ...".
    print_os_error(&Error::last_os_error());
    // Will print "Not an OS error".
    print_os_error(&Error::new(ErrorKind::Other, "oh no!"));
}
```


```rust
pub fn get_ref(&self) -> Option<&(Error + Send + Sync + 'static)>
```

1.3.0

Returns a reference to the inner error wrapped by this error (if any).

If this Error was constructed via new then this function will return Some, otherwise it will return None.

Examples

```rust
use std::io::{Error, ErrorKind};

fn print_error(err: &Error) {
    if let Some(inner_err) = err.get_ref() {
        println!("Inner error: {:?}", inner_err);
    } else {
        println!("No inner error");
    }
}

fn main() {
    // Will print "No inner error".
    print_error(&Error::last_os_error());
    // Will print "Inner error: ...".
    print_error(&Error::new(ErrorKind::Other, "oh no!"));
}
```

```rust
pub fn get_mut(&mut self) -> Option<&mut (Error + Send + Sync + 'static)>
```
1.3.0

Returns a mutable reference to the inner error wrapped by this error (if any).

If this Error was constructed via new then this function will return Some, otherwise it will return None.

Examples
```rust
use std::io::{Error, ErrorKind};
use std::{error, fmt};
use std::fmt::Display;

#[derive(Debug)]
struct MyError {
    v: String,
}

impl MyError {
    fn new() -> MyError {
        MyError {
            v: "oh no!".to_string()
        }
    }

    fn change_message(&mut self, new_message: &str) {
        self.v = new_message.to_string();
    }
}

impl error::Error for MyError {
    fn description(&self) -> &str { &self.v }
}

impl Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyError: {}", &self.v)
    }
}

fn change_error(mut err: Error) -> Error {
    if let Some(inner_err) = err.get_mut() {
        inner_err.downcast_mut::<MyError>().unwrap().change_message("I've been changed!");
    }
    err
}

fn print_error(err: &Error) {
    if let Some(inner_err) = err.get_ref() {
        println!("Inner error: {}", inner_err);
    } else {
        println!("No inner error");
    }
}

fn main() {
    // Will print "No inner error".
    print_error(&change_error(Error::last_os_error()));
    // Will print "Inner error: ...".
    print_error(&change_error(Error::new(ErrorKind::Other, MyError::new())));
}
```


```rust
pub fn into_inner(self) -> Option<Box<Error + Send + Sync>>
```
1.3.0

Consumes the Error, returning its inner error (if any).

If this Error was constructed via new then this function will return Some, otherwise it will return None.

Examples

```rust
use std::io::{Error, ErrorKind};

fn print_error(err: Error) {
    if let Some(inner_err) = err.into_inner() {
        println!("Inner error: {}", inner_err);
    } else {
        println!("No inner error");
    }
}

fn main() {
    // Will print "No inner error".
    print_error(Error::last_os_error());
    // Will print "Inner error: ...".
    print_error(Error::new(ErrorKind::Other, "oh no!"));
}
```

```rust
pub fn kind(&self) -> ErrorKind
```

Returns the corresponding ErrorKind for this error.

```rust
use std::io::{Error, ErrorKind};

fn print_error(err: Error) {
    println!("{:?}", err.kind());
}

fn main() {
    // Will print "No inner error".
    print_error(Error::last_os_error());
    // Will print "Inner error: ...".
    print_error(Error::new(ErrorKind::AddrInUse, "oh no!"));
}
```



## Trait Implementations

```rust
impl From<NulError> for Error {
    fn from(_: NulError) -> Error;
    // Performs the conversion.
}

impl<W> From<IntoInnerError<W>> for Error {
    fn from(iie: IntoInnerError<W>) -> Error;
    // Performs the conversion.
}

impl Debug for Error {
    fn fmt(&self, __arg_0: &mut Formatter) -> Result;
    // Formats the value using the given formatter.
}

// Intended for use for errors not exposed to the user, where allocating onto
// the heap (for normal construction via Error::new) is too costly.
impl From<ErrorKind> for Error { // 1.14.0
    fn from(kind: ErrorKind) -> Error;
    // Performs the conversion.
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> Result;
    // Formats the value using the given formatter.
}

impl Error for Error {
    fn description(&self) -> &str;
    // A short description of the error.

    fn cause(&self) -> Option<&Error>;
    // The lower-level cause of this error, if any.
}
```
