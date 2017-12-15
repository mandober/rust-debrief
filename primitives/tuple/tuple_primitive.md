# Tuples
https://doc.rust-lang.org/std/primitive.tuple.html

Primitive Type *tuple* 1.0.0

A finite heterogeneous sequence, `(T, U, ..)`.

Let's cover each of those in turn:

Tuples are finite, a tuple has a length.

Here's a tuple of length 3:

```rust
("hello", 5, 'c');
```

'Length' is also sometimes called 'arity' here; each tuple of a different length 
is a different, distinct type.

Tuples are heterogeneous. This means that each element of the tuple can have a 
different type. In that tuple above, it has the type:

```rust
(&'static str, i32, char)
```

Tuples are a sequence. This means that they can be accessed by position;
this is called 'tuple indexing', and it looks like this:

```rust
let tuple = ("hello", 5, 'c');

assert_eq!(tuple.0, "hello");
assert_eq!(tuple.1, 5);
assert_eq!(tuple.2, 'c');
```

For more about tuples, see the book.


## Trait implementations

If every type inside a tuple implements one of the following traits, then a 
tuple itself also implements it.
- `Clone`
- `Copy`
- `PartialEq`, `Eq`, `PartialOrd`, `Ord`
- `Debug`
- `Default`
- `Hash`

Due to a temporary restriction in Rust's type system, these traits are only 
implemented on tuples of arity 12 or less. In the future, this may change.

Examples:

```rust
let tuple = ("hello", 5, 'c');
assert_eq!(tuple.0, "hello");
```

Tuples are often used as a return type when you want to return more than one value:

```rust
fn calculate_point() -> (i32, i32) {
    // Don't do a calculation, that's not the point of the example
    (4, 5)
}

let point = calculate_point();
assert_eq!(point.0, 4);
assert_eq!(point.1, 5);

// Combining this with patterns can be nicer.
let (x, y) = calculate_point();

assert_eq!(x, 4);
assert_eq!(y, 5);
```
