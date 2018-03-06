# Procedural Macros

Procedural macros allow creating syntax extensions as execution of a function. Procedural macros can be used to implement custom derive on your own types.

Procedural macros involve a few different parts of the language and std:
- `proc_macro` crate defines an interface for building proc macros.
- `#[proc_macro_derive(Foo)]` attribute marks the deriving function.
- proc macros must be in their own crate, with `proc-macro` crate type.

The deriving function must have this exact signature:

```rust
use proc_macro::TokenStream;

#[proc_macro_derive(Hello)]
pub fn hello_world(input: TokenStream) -> TokenStream
```

Procedural macros accept some Rust code as an input, operate on that code, and produce some Rust code as an output, rather than matching against patterns and replacing the code with other code as declarative macros do.

Today, the only thing you can define procedural macros for is to allow your traits to be implemented on a type by specifying the trait name in a derive annotation.


