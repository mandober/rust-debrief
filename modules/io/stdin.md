# Struct `std::io::Stdin` 1.0.0

https://doc.rust-lang.org/stable/std/io/struct.Stdin.html

```rust
pub struct Stdin { /* fields omitted */ }
```
A handle to the standard input stream of a process.

Each handle is a shared reference to a global buffer of input data to this 
process. A handle can be lock'd to gain full access to `BufRead` methods 
(e.g. `lines()`). Reads to this handle are otherwise locked with respect to 
other reads.

This handle implements the `Read` trait, but beware that concurrent reads 
of `Stdin` must be executed with care.

Created by the `io::stdin` method.



## Methods

### `impl Stdin`

Two methods that implement Stdin: `lock()` and `read_line()`


#### lock()

```rust
fn lock(&self) -> StdinLock
```

Locks this handle to the standard input stream, returning a readable guard. 
The lock is released when the returned lock goes out of scope. The returned 
guard also implements the `Read` and `BufRead` traits for accessing the 
underlying data.

```rust
use std::io::{self, Read};

let mut buffer = String::new();
let stdin = io::stdin();
let mut handle = stdin.lock();

handle.read_to_string(&mut buffer)?;
```


#### read_line ()

```rust
fn read_line(&self, buf: &mut String) -> Result<usize>
```

Locks this handle and reads a line of input into the specified buffer. 
For detailed semantics, see [`BufRead::read_line`]
(https://doc.rust-lang.org/stable/std/io/trait.BufRead.html#method.read_line).

```rust
use std::io;

let mut input = String::new();
match io::stdin().read_line(&mut input) {
    Ok(n) => {
        println!("{} bytes read", n);
        println!("{}", input);
    }
    Err(error) => println!("error: {}", error),
}
```
