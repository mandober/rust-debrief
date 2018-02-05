# Pattern matching

Refutability
- patterns which cannot fail to match for any possible value are irrefutable, and patterns which can fail to match for some possible value are refutable.
- `let` statements, function parameters, and `for` loops only accept _irrefutable_ patterns, since there’s nothing correct the program could do if the pattern fails to match.
- `if let`, `while let` only accept _refutable_ patterns, since it is their job to handle possible failure.

Places where patterns are used:
- `match` expression
- refutable: `if let`, `while let` expressions
- irrefutable: `let`statements, `for` loops, function parameters






## `match`expressions

match expressions are required to be exhaustive.

an `_` matches anything, but never binds variables.

Formally, match expressions are defined as the keyword `match`, a value to match on, and one or more match arms that consist of a pattern and an expression to run if the value matches that arm’s pattern:

```rust
let result: Result<File, Error> = std::fs::File::open("hello.txt");
let file_handle: File = match result {
    Ok(fh) => fh,
    // match guard
    Err(ref err) if err.kind() == std::io::ErrorKind::NotFound => {
        // ...
    },
    Err(error) => panic!("problem opening the file: {:?}", error),
    _ => //...
};
```



## `if let` expressions

```rust
// assign to f the matching value
let f = match result {
    Ok(v) => v,
    Err(e) => panic!("{:?}", e),
};

// if let variant:
if let Ok(v) = result {
    let f = v;
}
```

It’s even possible to mix and match `if let`, `else if`, and `else if let`.

```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```
Note that `if let` can also introduce shadowed variables like match arms can: `if let Ok(age) = age` introduces a new shadowed `age` variable that contains the value inside the Ok variant. This also means the `if age > 30` condition needs to go within the block; we aren’t able to combine these two conditions into `if let Ok(age) = age && age > 30` since the shadowed age that we want to compare to 30 *isn’t valid until the new scope starts* with the braces.


## `while let`

Allows you to do a while loop as long as a pattern continues to match.

```rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```
The while loop will continue running the code in its block as long as pop is returning Some. Once it returns None, the while loop stops. 



## `for` loops

Using a pattern in a for loop to destructure it.

```rust
let v = vec![1, 2, 3];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```

## `let` Statements

`let PATTERN = EXPRESSION;`

```rust
let (x, y, z) = (1, 2, 3);
```
Using a pattern to destructure a tuple.



## Function parameters

function parameters can also be patterns.

```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
```

