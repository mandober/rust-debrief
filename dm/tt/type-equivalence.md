# Type Equivalence

Type Compatibility and Equivalence

https://en.wikipedia.org/wiki/Type_equivalence


### Compatibility: equivalence and subtyping

> A type-checker for a statically typed language must verify that the type of any expression is consistent with the type expected by the context in which that expression appears.

For example, in an assignment statement of the form `x := exp`, the inferred type of the expression `exp` must be consistent with the (declared or inferred) type of the variable `x`. This notion of consistency, __type compatibility__, is specific to each programming language.

If the type of `exp` and the type of `x` are the same, and assignment is allowed for that type, then this is a valid expression. Thus, 
In the simplest type systems, the question of whether two types are compatible reduces to that of whether they are equal (or equivalent).

Different languages, however, have different criteria for when two type expressions are understood to denote the same type. These different _equational theories_ of types vary widely, two extreme cases being __structural type systems__, in which any two types that describe values with the same structure are equivalent, and __nominative type systems__, in which no two syntactically distinct type expressions denote the same type (i.e., types must have the same "name" in order to be equal).

In languages with subtyping, the compatibility relation is more complex. In particular, if A is a subtype of B, then a value of type A can be used in a context where one of type B is expected, even if the reverse is not true. Like equivalence, the *subtype relation* is defined differently for each programming language, with many variations possible. The presence of _parametric polymorphism_ or _ad hoc polymorphism_ in a language may also have implications for type compatibility.


## Nominal type system
https://en.wikipedia.org/wiki/Nominal_type_system

In a nominal (nominative, name-based) type system the language decides whether types are compatible or equivalent based on __explicit declarations or the name of the types__.

Nominal systems are used to determine if types are equivalent, as well as if a type is a subtype of another. It contrasts with structural systems, where comparisons are based on the structure of the types in question and do not require explicit declarations.

Nominal typing means that two variables are type-compatible iff their declarations name the same type. For example, in C, two struct types with different names in the same translation unit are never considered compatible, even if they have identical field declarations.


## Structural type system
https://en.wikipedia.org/wiki/Structural_type_system

A structural type system means that the language decides whether types are compatible or equivalent based on the definition and characteristics of the types.
