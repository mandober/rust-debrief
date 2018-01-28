# Type theory

A type theory is any of a class of formal systems, some of which can serve as alternatives to set theory. In type theory, every "term" has a "type" and operations are restricted to terms of a certain type. Type theory is closely related to, and in some cases overlaps with, *type systems*, which are a feature of programming languages . Two well-known type theories are Alonzo Church's typed λ-calculus and Per Martin-Löf's intuitionistic type theory.

In type theory
- concepts like "and" and "or" can be encoded as types
- terms (generally) belong to only one type. Where a subset would be used, type theory creates a new type, called a dependent sum type, with new terms. Union is similarly achieved by a new sum type and new terms.
- types that combine unrelated types do so by creating new terms.


## Concepts
In a system of type theory, each term (expression) has a type and operations are restricted to terms of a certain type.

An assertion `v: T` describes that the term `v` has type `T`. For example, `uint` may be a type representing positive integers and `0,1,...` may be members of that type. The assertion that 42 has a type `uint` is written as `42: uint`.

A function in type theory is denoted with an arrow, `->`. For example, the function `add` has the assertion `add: int -> int`, meaning that it takes an integer and returns an integer.


## Dependent type
A dependent type, also called *pi type*, is a type that depends on a term or on another type. 

Two common examples of dependent types are dependent functions and dependent pairs.

A "pair of integers" is a type; a "pair of integers where the second is greater than the first" is a dependent type because of the dependence on the value.

A dependent function's return type may depend on the value (not just type) of its parameter. For example, a function that takes a positive integer `n` and returns an array of length `n`.










The type returned by a function may depend upon the argument to the function.

Dependent types generalize the normal function space to model functions whose output type varies on their input. While normal function have assertion:  
`fn() int -> int`



For example, writing `Vec(R, n)` for the type of n-tuples of real numbers (R) and N for the type of natural numbers,

∏ Vec(R, n)
n:N













---
https://www.wikiwand.com/en/Type_theory

