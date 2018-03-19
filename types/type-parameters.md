# Type parameters

Items that can be parameterized by type:
- functions
- type aliases
- structs
- enums
- unions
- traits
- implementations

Type parameters are given as a comma-separated list of identifiers enclosed in angle brackets after the name of the item; except for implementations, where they come directly after `impl` and before its definition.

The type parameters of an item are considered "part of the name", not part of the type of the item.

A referencing path must (in principle) provide type arguments as a list of comma-separated types enclosed within angle brackets, in order to refer to the type-parameterized item. In practice, the type-inference system can usually infer such argument types from context.

There are no general _type-parametric types_, only _type-parametric items_. That is, Rust has no notion of type abstraction: there are no _higher-ranked types_ (no `forall`) abstracted over other types, though higher-ranked types do exist in connection with lifetimes.

Type parameters stand in for **input** types and associated types stand in for **output** types.