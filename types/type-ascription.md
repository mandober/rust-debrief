# Type Ascription

RFC: https://github.com/nrc/rfcs/blob/ascription/text/0000-type-ascription.md

Type inference is imperfect; it is often useful to help type inference by annotating a sub-expression with a type.

Currently, this is only possible by extracting the sub-expression into a variable using a `let` statement and/or giving a type for a whole expression or pattern. This is not ergonomic, and sometimes impossible due to lifetime issues;specifically, when a variable has lifetime of its enclosing scope, but a sub-expression's lifetime is typically limited to the nearest semi-colon. Typical use case is when a function's return type is generic (e.g. `collect`) and when we want to force a coercion.

