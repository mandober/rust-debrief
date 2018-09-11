# `io` module

Module `std::io` 1.0.0

The `std::io` module contains things needed for IO:
traits, structs, helpers, and type definitions.

The most core part of this module is the `Read` and `Write`
traits, which provide the most general interface for reading and writing input 
and output.

traits: `Read`, `Write`, `Seek`, `BufRead`
structs: `BufReader` and `BufWriter`




## `Read` and `Write` traits

`Read` and `Write` traits are implemented by a number of other types, 
and you can implement them for your types too. As such, you'll see a few 
different types of IO throughout the documentation in this module: `Files`, 
`TcpStreams`, and sometimes even `Vec<T>`.

`Read` and `Write` are so important, implementors of the two traits have a 
nickname: *readers* and *writers*. "reader" means "a type that implements 
the `Read` trait".

For example, `Read` adds a `read` method, which we can use on `Files`:

```rust
use std::io;
use std::io::prelude::*; // Read, Write, Seek, BufRead
use std::fs::File;

let mut f = File::open("foo.txt")?;
let mut buffer = [0; 10];

// read up to 10 bytes
f.read(&mut buffer)?;

println!("The bytes: {:?}", buffer);
```



## `Seek` and `BufRead` traits

`Seek` and `BufRead` build on top of a reader to control how the reading happens: 
`Seek` lets you control where the next byte is coming from.
`BufRead` uses an internal buffer to provide a number of other ways to read.

```rust
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;

let mut f = File::open("foo.txt")?;
let mut buffer = [0; 10];

// skip to the last 10 bytes of the file
f.seek(SeekFrom::End(-10))?;

// read up to 10 bytes
f.read(&mut buffer)?;

println!("The bytes: {:?}", buffer);
```


## `BufReader` and `BufWriter` structs

Byte-based interfaces are unwieldy and can be inefficient, as we'd need to be 
making near-constant calls to the OS. To help with this, std::io comes with two 
structs, `BufReader` and `BufWriter`, which wrap readers and writers. 

The wrapper uses a buffer, reducing the number of calls and providing nicer 
methods for accessing exactly what you want.

For example, `BufReader` works with the `BufRead` 
trait to add extra methods to any reader:

```rust
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

let f = File::open("foo.txt")?;
let mut reader = BufReader::new(f);
let mut buffer = String::new();

// read a line into buffer
reader.read_line(&mut buffer)?;

println!("{}", buffer);
```

`BufWriter` doesn't add any new ways of writing;
it just buffers every call to write:

```rust
use std::io;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

let f = File::create("foo.txt")?;
{
    let mut writer = BufWriter::new(f);
    // write a byte to the buffer
    writer.write(&[42])?;
} // the buffer is flushed once writer goes out of scope
```



## Standard input and output

A very common source of input is standard input:

```rust
use std::io;

let mut input = String::new();

io::stdin().read_line(&mut input)?;

println!("You typed: {}", input.trim());
```


Note that you cannot use the `?` operator in functions that do not return a 
`Result<T, E>` (e.g. main). Instead, you can call `.unwrap()` or `match` on the 
return value to catch any possible errors:

```rust
use std::io;

let mut input = String::new();

io::stdin().read_line(&mut input).unwrap();
```

And a very common source of output is standard output:

```rust
use std::io;
use std::io::prelude::*;

io::stdout().write(&[42])?;
```

Of course, using `io::stdout` directly is less common than `println!`.


## Iterator types

A large number of the structures provided by std::io are for various ways of 
iterating over IO. 

For example, Lines is used to split over lines:

```rust
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

let f = File::open("foo.txt")?;
let reader = BufReader::new(f);

for line in reader.lines() {
    println!("{}", line?);
}
```


## Functions

There are a number of functions that offer access to various features.

For example, we can use 3 of these fns to copy everything from stin to stdout:

```rust
use std::io;

io::copy(&mut io::stdin(), &mut io::stdout())?;
```


## io::Result

`io::Result` type is used as the return type of many std::io functions that can cause an 
error, and can be returned from your own functions as well. 

Many of the examples in this module use the ? operator:

```rust
use std::io;

fn read_input() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    println!("You typed: {}", input.trim());

    Ok(())
}
```

The return type of `read_input()`, `io::Result<()>`, is a very common type for 
functions which don't have a 'real' return value, but do want to return errors 
if they happen. In this case, the only purpose of this function is to read the 
line and print it, so we use `()`.



## Platform-specific behavior
Many I/O functions throughout the standard library are documented to indicate 
what various library or syscalls they are delegated to. This is done to help 
applications both understand what's happening under the hood as well as 
investigate any possibly unclear semantics. Note, however, that this is 
informative, not a binding contract. The implementation of many of these 
functions are subject to change over time and may call fewer or more 
syscalls/library functions.



## Modules
`prelude`    The I/O Prelude

## Type Definitions
`Result`     A specialized Result type for I/O operations.

## Enums
`ErrorKind`  A list specifying general categories of I/O error.
`SeekFrom`   Enumeration of possible methods to seek within an I/O object.
`CharsError` [LAB] enum of possible errors that can be generated from the Chars adapter.

## Traits
`BufRead` A BufRead is a type of Reader which has an internal buffer, allowing it to perform extra ways of reading.
`Read`    The Read trait allows for reading bytes from a source.
`Seek`    The Seek trait provides a cursor which can be moved within a stream of bytes.
`Write`   A trait for objects which are byte-oriented sinks.

## Functions
`copy`    Copies the entire contents of a reader into a writer.
`empty`   Constructs a new handle to an empty reader.
`repeat`  Creates an instance of a reader that infinitely repeats one byte.
`sink`    Creates an instance of a writer which will successfully consume all data.
`stderr`  Constructs a new handle to the sterr of the current process.
`stdin`   Constructs a new handle to the stdin of the current process.
`stdout`  Constructs a new handle to the stdout of the current process.

## Structs
`BufReader` adds buffering to any reader.
`BufWriter` Wraps a writer and buffers its output.

`Bytes` iterator over u8 values of a reader.
`Lines` iterator over the lines of an instance of BufRead.
`Split` iterator over the contents of an instance of BufRead split on a particular byte.
`Chars` iterator over the chars of a reader. [LAB]
`Take`  Reader adaptor which limits the bytes read from an underlying reader.
`Chain` Reader adaptor to chain together two readers.

`Empty`  A reader which is always at EOF.
`Repeat` A reader which yields one byte over and over and over and over and over and...
`Sink`   A writer which will move data into the void.

`Cursor` A Cursor wraps another type and provides it with a Seek implementation.
`LineWriter` Wraps a writer and buffers output to it, flushing whenever a newline (0x0a, '\n') is detected.
`Initializer` A type used to conditionally initialize buffers passed to Read methods. [LAB]

`Error` The error type for I/O operations of the Read, Write, Seek, and associated traits.
`IntoInnerError` An error returned by into_inner which combines an error that 
                 happened while writing out the buffer, and the buffered writer 
                 object which may be used to recover from the condition.

`Stdin`      handle to the stdin stream of a process.
`Stdout`     handle to the global stdout stream of the current process.
`Stderr`     handle to the stderr stream of a process.
`StdinLock`  locked reference to the Stdin handle.
`StdoutLock` locked reference to the Stdout handle.
`StderrLock` locked reference to the Stderr handle.
