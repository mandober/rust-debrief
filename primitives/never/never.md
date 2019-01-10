# Never type

- never type, `!`
- primitive type. **LAB**
- online [docs](https://doc.rust-lang.org/nightly/std/primitive.never.html)
- `!` represents the type of computations that never resolves to a value
- `break`, `continue` and `return` expressions have never type, `!`
-  expressions with type `!` will coerce into any other type

```rust
#![feature(never_type)]
let x: ! = {
    return 123
};
```

Although the let is pointless here, it illustrates the meaning of `!`. Since `x` is never assigned a value (because `return` returns from the entire function), `x` can be given type `!`. We could also replace `return 123` with a `panic!` or a never-ending loop and this code would still be valid.



## In type theory

Never is the bottom type i.e. the type that has no values. In type theory it is also called the zero or empty type, and is sometimes denoted with falsum, `⊥`.

In subtyping systems, the bottom type is the subtype of all types. However, the converse is not true - a subtype of all types is not necessarily the bottom type. It is used to represent the return type of a function that does not return a value: for instance, one which loops forever, signals an exception, or exits. A function whose return type is bottom type cannot return any value.

Because the bottom type is used to indicate the lack of a normal return, it typically has no values. It contrasts with the _top type_, which spans all possible values in a system, and a _unit type_, which has exactly one value. The bottom type is sometimes confused with the so-called _void type_, which is actually a unit type, albeit one with no defined operations.

In Rust, the bottom type is denoted by `!`. It is present in the type signature of functions guaranteed to never return, for example by calling `panic!()` or looping forever.
