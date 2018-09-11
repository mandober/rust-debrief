# Pattern matching

- irrefutable patterns cannot fail to match for any possible value
- refutable patterns can fail to match for some possible value


<!-- TOC -->

- [Patterns](#patterns)
- [Refutability](#refutability)
- [match](#match)
- [`if let` expressions](#if-let-expressions)
- [`while let`](#while-let)
- [`for` loops](#for-loops)
- [`let` Statements](#let-statements)
- [Function parameters](#function-parameters)

<!-- /TOC -->


## Patterns

An expression is matched against a pattern.
A pattern can be:
- pattern `_` matches anything, binds nothing.
- constant expression (constant, constant identifier)
- variable name (to be bound to it if matched)
- range
- structural type (struct, tuple, array)
- etc. (non-exhaustive list)


## Refutability

**Irrefutable** patterns cannot fail to match for any possible value. The _binding sites_ (`let` statement, function parameters, `for` loop, `match`) only accept irrefutable patterns.

**Refutable** patterns can fail to match for some possible value. The _match sites_ (`match`, `if let`, `while let`) accept refutable patterns since they know how to handle all outcomes (pattern matches, pattern fals to match). `match` must handle all possible outcomes. `if/while let` are shorthand forms for match and they only handle one outcome, ignoring the rest.



## match

match consists of `match` keyword, a _test expression_ and an appropriate number of match arms so that all possible outcomes are covered.

A _match arm_ consist of a _pattern_ and an _branch_ expression, which is executed if the value matches that arm's pattern.

A match arm is an expression (unless teminated with a semicolon) and if matched, it returns a value, which in turn becomes the return value of the whole match expression (unless teminated with a semicolon).



```rust
// match expression
match TEST {
    PAT => BRANCH
}
// match statement
match TEST {
    PAT => BRANCH
};
```





match arms must be "just right": they must be exhaustive, but not overdone:
- all possible outcomes must be accounted for, otherwise the complier will emit the "non-exhaustive patterns" _error_.
- a matching overkill will compile, but the compiler will issue the "unreachable pattern" _warning_.


```rust
// delimit arms with comma:
match TEST {
    PAT1 => EXECUTOR1,
    PAT2 => EXECUTOR2,
}

// delimit arms with braces:
match EXPRESS {
    PAT1 => { EXECUTOR1 }
    PAT2 => { EXECUTOR2 }
}

// delimit arms with braces, in which case comma is optional:
match EXPRESS {
    PAT1 => { EXECUTOR1 },
    PAT2 => { EXECUTOR2 },
}

// mixture:
match EXPRESS {
    PAT1 => EXECUTOR1,
    PAT2 => { EXECUTOR2 },
}
```


Formally, match expressions are defined as the keyword `match`, a value to match on, and one or more match arms that consist of a pattern and an expression to run if the value matches that arm’s pattern:

```rust

match v {
        5 => {println!("five")}
        _ => {println!("no match")}
    }




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

