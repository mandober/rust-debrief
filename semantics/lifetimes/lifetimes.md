# Lifetimes

- The `'static` lifetime is the entire duration of the program. All string literals have the `'static` lifetime.
- Every ref has a lifetime, which is the scope for which that ref is valid.
- GLAs tell Rust how the GLPs relate to each other.
- GLAs are descriptive (not prescriptive); borrow checker compares scopes to determine that all refs are valid.
- Structs can also hold references (besides owned values), but we need to add a GLAs on every reference in the struct definition.
- Just like with generic data types, we have to declare the name of the GLP inside angle brackets after the name of the struct so that we can use the lifetime parameter in the body of the struct definition.

Lifetime Elision Rules
1. In function, each param that is a reference gets its own GLP.
2. If there is only 1 input GLP, that lifetime is set to all output GLPs.
3. In methods with multiple input GLP, lifetime of `self` is set to all output GLP.


## The static lifetime

The `'static` lifetime is the entire duration of the program. All string literals have the `'static` lifetime. The text of the string literals is stored directly in the final binary of the program and the binary of the program is always available - therefore, the lifetime of all string literals is `'static`.

```rust
let sl: &'static str = "string literal";
```


## Scope
Every reference has a lifetime, which is the scope for which that reference is valid. Lifetime annotations are descriptive (not prescriptive) - they only describe how the lifetimes of references relate to each other; borrow checker then compares scope of references to determine that all references are valid, thus avoiding problems like dangling pointer.

```rust
// annotations of the lifetimes of o and i:
{
  let o;             //..................
                     //                 ↑
  {                  //           'outter
    let i = 5;       //........         .
                     //       ↑         .
    o = &i;          //    'inner       .
                     //       ↓         .
                     //........         .
  }                  //                 .
  println!("{}", o); //                 ↓
                     //..................
}
```
Above, the variable `o` is declared in the outer scope; it takes on a value i.e. a reference to the `i` variable, that is valid only in the inner scope. When the inner scope ends, variable `i` is dropped - the variable `o` cannot be allowed to refer to `i` after the inner scope ends, because it would create a dangling pointer.


## Lifetime of references

Only references have lifetimes. Structs can also hold references (besides owned values), but we need to add a GLAs on every reference in the struct definition.
Just like with generic data types, we have to declare the name of the GLP inside angle brackets after the name of the struct so that we can use the lifetime parameter in the body of the struct definition.
