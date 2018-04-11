# Type Annotations

A static type system performs type checking at compile-time, so in order to do its job it requires that all expressions have been designated with a type.

The process of setting a type may be dynamic (performed by compiler or interpreter), or static (done manually by the programmer), or, to a different extent, somewhere in between.

Type annotation is an explicit identification of the data type by placing the type identifiers directly on expressions in the source code. Type identifier directly refers to a named data type, with the same name as the identifier, but it can also refer to a type alias.

In contrast to manual type annotation, type inference is identification of data types entirely performed by the compiler, where the type of value is deduced from the context at compile-time.

Rust can infer types in many situations allowing the programmer to omit type annotations while still enjoying the benefits of type checking.


- placement
- turbofish
- type hint `_`
- mandatory places
- type alias

