# Variables

A variable is a storage location paired with an associated symbolic name (an identifier) that contains some quantity of information referred to as a value.

The variable name is the usual way to reference the stored value; this separation of name and content allows the name to be used independently of the exact information it represents.



## Variables in Rust

By default, variable bindings have move semantics. However, if a type implements `Copy`, it instead has 'copy semantics':


scope: local scope delimited by braces

```rust
let n;          // error: "cannot infer type for _"
let n: i32;     // ok: explicit type annotation
let n = 5;      // ok: type infered as i32 (default for integers)
let n: i32 = 5; // ok: reduntant explicit type annotation
let n: i16 = 5; // ok: explicit type annotation
```