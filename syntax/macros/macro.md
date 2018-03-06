# Macros

- Macros are expanded before the compiler interprets the code.
- Macro definitions are not namespaced within modules (possible conflicts).
- They must be defined or imported into scope before they're used.
- macros: declarative and procedural.


Macros are expanded before the compiler interprets the code.

Macros are not namespaced within modules, so macros from different crates with the same name can cause conflicts. To avoid this, macros must be explicitly imported into the scope of the current crate with `#[macro_use]` attribute. 

For example, to import all macros from `serde` crate:

```rust
#[macro_use]
extern crate serde;

// frequently seen as oneliner:
#[macro_use] extern crate serde;
```


There are two kinds of macros in Rust: declarative and procedural.
