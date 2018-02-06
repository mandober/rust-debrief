# Errors

- Error handling: exceptions don't exist, instead `Result<T, E>` for handling recoverable errors and `panic!` macro for unrecoverable.
- Clean exit: `panic!` prints a failure message, unwinds the stack, quits.
- Unwinding the stack
- Abrupt exit: immediate quit, leave 



## Unrecoverable errors

When called `panic!` macro can start the exit sequence properly or just quit abruptly. The exit sequence stops the execution :
- prints a failure message
- unwinds the stack
- quit

To quit abruptly
When unwinding is not wanted (one wants to quit immediately without cleanup)

To abort on panic instead of unwinding, add
`panic = 'abort'` to the `[profile]` section of `Cargo.toml`

For example, if you want to abort on panic in release mode:
[profile.release]
panic = 'abort'


### BACKTRACING

`RUST_BACKTRACE=1 cargo run`
shows a backtrace - a list of all the functions that have been called to get to this point in execution.

the key to reading the backtrace is to start from the top and read until you see
files you wrote. That’s the spot where the problem originated. The lines above
the lines mentioning your files are code that your code called; the lines below
are code that called your code. These lines might include core Rust code,
standard library code, or crates that you’re using.

## Recoverable Errors with Result

T represents the type of the value that will be returned in a success case within
the Ok variant, and E represents the type of the error that will be returned in
a failure case within the Err variant.
Result enum is defined as having two variants, Ok and Err, as follows:
*/
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// File::open could result in error or success
use std::fs::File;
fn main() {
    let f = File::open("hello.txt");
}
// How do we know File::open returns a Result? Check Docs or deliberately put
// the wrong type and see what compiler says (!?).

// In the case where File::open succeeds, the value we will have in the variable
// f will be an instance of Ok that contains a file handle. In the case where
// it fails, the value in f will be an instance of Err

// After the match, we can then use the file handle for reading or writing.

// use match to deal with Result
use std::fs::File;
fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("problem opening the file: {:?}", error),
    };
}


// ! Matching on Different Errors

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        // & matches a reference and gives us its value, but
        // ref MATCHES A VALUE and gives us a REFERENCE TO IT.
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating a file: {:?}", e),
            }
        },
        Err(e) => panic!("Error opening the file: {:?}", e),
    };
}

/**
The type of the value that `File::open` returns inside the `Err` variant is
`io::Error`, which is a struct provided by the standard library. This struct has
a method `kind()` that we can call to get an `io::ErrorKind` value.

`io::ErrorKind` is an ENUM provided by the standard library that has variants
representing the different kinds of errors that might result from an `io` operation.

The variant we’re interested in is `ErrorKind::NotFound`, which indicates the
file we’re trying to open doesn’t exist yet.

The condition `if error.kind() == ErrorKind::NotFound` is called a MATCH GUARD:
it’s an extra condition on a match arm that further refines the arm’s pattern.

The `ref` in the pattern is needed so that `error` is not moved into the guard
condition but is merely referenced by it.

`ref` is used to take a reference in a pattern instead of `&` cuz in the context
of a pattern, `&` matches a reference and gives us its value, but `ref` matches
a value and gives us a reference to it.


// ! Shortcuts for Panic on Error: unwrap and expect

// ? unwrap
`unwrap` method defined on `Result<T, E>` is a shortcut method:
- if the `Result` value is the `Ok` variant, unwrap will return the value inside
the `Ok`.
- if the `Result` is the `Err` variant, unwrap will call the `panic!` macro with
default panic message.
*/
use std::fs::File;
fn main() {
    let f = File::open("hello.txt").unwrap();
}

// ? expect
// `expect` method is similar to unwrap, it just allows for custom panic message.
use std::fs::File;
fn main() {
    let f = File::open("hello.txt").expect("Fail to open");
}


// ! Propagating Errors
/**
When writing a function whose implementation calls something that might fail,
instead of handling the error within this function, you can choose to let your
caller know about the error so they can decide what to do.

This is known as propagating the error, and gives more control to the calling
code where there might be more information or logic that dictates how the error
should be handled than what you have available in the context of your code.
*/
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    // let mut f = File::open("hello.txt")?;
    // would do the same as match below:
    let mut f = match f {
        Ok(file) => file,
        // return early, return Err(e) to caller
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        // return payload to caller, wrapped in Ok(s)
        Ok(_) => Ok(s),
        // return Err(e) to caller
        // last expression, no need for `return`
        Err(e) => Err(e),
    }
}
/**
Here a function reads a username from a file. If the file doesn’t exist or can’t
be read, this function will return those errors to the code that called this function

// * -> Result<String, io::Error>
Return type of the function means that the function is returning a value of the
type `Result<T, E>` where the generic parameter `T` has been filled in with the
concrete type `String`, and the generic type `E` has been filled in with the
concrete type `io::Error`.

- Ok value will hold a String (username read from file)
- Err value will hold an instance of io::Error (that contains
  more information about what the problems were).

We chose `io::Error` as the return type because that is the type of the error
value returned from both `File::open` and `read_to_string`.

// * Err(e) => return Err(e)
in the `Err` case, we RETURN EARLY FROM THIS FUNCTION and PASS THE ERROR VALUE
from `File::open` BACK TO THE CALLER as THIS FUNCTION’S ERROR (RETURN) VALUE.
If `File::open` succeeds, we store the file handle in the variable `f` and
continue with this function.

// * Ok(_) => Ok(s)
if `read_to_string` succeeds, then our function has succeeded, and we return the
username from the file that’s now in `s` wrapped in an `Ok` as `Ok(s)`.

// * Err(e) => Err(e)
If `read_to_string fails`, we return the error value in the same way that we
returned the error value in the match that handled the return value of `File::open`.
We don’t need to explicitly say `return`, however, since this is the last
expression in the function.

The code that calls this code will then handle getting either an `Ok` value that
contains a username or an `Err` value that contains an `io::Error`.
We don’t have enough information on what the caller is actually trying to do, so
we propagate all the success or error information upwards for them to handle as
they see fit.


// ! questionmark operator (?)
This pattern of propagating errors is so common that there is dedicated syntax
using `?` operator
*/
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    let mut f = File::open("hello.txt")?;
    /* instead of:
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    }; */

    f.read_to_string(&mut s)?;
    /* instead of:
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    } */

    Ok(s) // explicitally return Ok(s) to caller.
}

/**
The `?` placed after a `Result` value is defined to work the exact same way as
the `match` expressions we defined to handle the `Result` values above.

If the value of the Result is an Ok, the value inside the Ok will get returned
from this expression and the program will continue.

If the value is an Err, the value inside the Err will be RETURNED FROM THE WHOLE
FUNCTION AS IF WE HAD USED THE RETURN keyword so that the error value gets
propagated to the caller.

In the match handling of Result above, using a ? at the end of the File::open
call will return the value inside an Ok to the variable f.

If an error occurs, ? will RETURN EARLY out of the whole function and give any
Err value to our caller.

The same thing applies to the ? at the end of the read_to_string call. But we
need to explicitally return Ok(s) to caller.


// ? Shorten by CHAINING METHOD CALLS immediately after `?`:
*/
use std::io;
use std::io::Read;
use std::fs::File;
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

/**
The ? can only be used in fn that have a return type of Result
it can't be used in main() function (which returns unit type)
In functions that don’t return Result, when you call other functions that
return Result, you’ll need to use a `match` or one of the Result methods to
handle it, instead of using ? to potentially propagate the error up the caller
chain.
*/