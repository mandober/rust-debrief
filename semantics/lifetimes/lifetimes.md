# Lifetimes

- Generic Lifetime Parameter (GLP)
- Generic Lifetime Annotation (GLA)
- The `'static` lifetime is the entire duration of the program. All string literals have the `'static` lifetime.
- Every ref has a lifetime, which is the scope for which that ref is valid.
- GLAs tell Rust how the GLPs relate to each other.
- GLAs are descriptive (not prescriptive); borrow checker compares scopes to determine that all refs are valid.
- Structs can also hold references (besides owned values), but we need to add a GLAs on every reference in the struct’s definition.
- Just like with generic data types, we have to declare the name of the GLP inside angle brackets after the name of the struct so that we can use the lifetime parameter in the body of the struct definition.
* Lifetime Elision Rules (LER):
  1. In function, each parameter that is a reference gets its own GLP.
  2. If there is only 1 input GLP, that lifetime is set to all output GLPs.
  3. In methods with multiple input GLPs, lifetime of `self` is set to all output GLPs.
  - Input lifetimes are lifetimes on function's parameters.
  - Output lifetimes are lifetimes on function's return values.
  - The elision rules are a set of particular cases that the compiler will consider, and if the code fits these cases, GLA can be ommitted.
  - The first rule applies to input lifetimes, and the second two rules apply to output lifetimes.
  - If the compiler gets to the end of the 3 rules and there are still references that it can't figure out lifetimes for, the compiler will stop with an error.



## The static lifetime
The `'static` lifetime is the entire duration of the program. All string literals have the `'static` lifetime. The text of the string literals is stored directly in the final binary of the program and the binary of the program is always available - therefore, the lifetime of all string literals is `'static`.

```rust
let sl: &'static str = "string literal";
```


## Scope
Every reference has a lifetime, which is the scope for which that reference is valid. Lifetime annotations are descriptive (not prescriptive) - they only describe how the lifetimes of refenrences relate to each other; borrow checker then compares scope of references to determine that all references are valid, thus avoiding problems like dangling pointer.

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
Above, the varibale `o` is declared in the outter scope; it takes on a value i.e. a reference to the `i` variable, that is valid only in the inner scope. When the inner scope ends, variable `i` is dropped - the variable `o` cannot be allowed to refer to `i` after the inner scope ends, because it would create a dangling pointer.


## Lifetime of references
Only references have lifetimes.

Structs can also hold references (besides owned values), but we need to add a GLAs on every reference in the struct’s definition.
Just like with generic data types, we have to declare the name of the GLP inside angle brackets after the name of the struct so that we can use the lifetime parameter in the body of the struct definition.
