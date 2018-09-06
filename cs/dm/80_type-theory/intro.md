# Introduction to Type Theory

- In type theory, every term has a type.
- Operations are restricted to terms of a certain type.
- Two well-known type theories (that can serve as foundations for math):
  - `Typed Lambda Calculus` by Alonzo Church
  - `Intuitionistic Type Theory` by Per Martin-Löf



In type theory
- concepts like _and_ and _or_ can be encoded as types.
- terms generally belong to only one type. Where a subset would be used, type theory creates a new type, called a *dependent sum type*, with new terms. Union is similarly achieved by a new sum type and new terms.
- types that combine unrelated types do so by creating new terms.


## Concepts

In a system of type theory, each term has a type and operations are restricted to terms of a certain type.

An assertion `v: T` describes that the term `v` has type `T`.

For example, `uint` may be a type representing positive integers and `0, 1, ...` may be members of that type. The assertion that 42 has a type `uint` is written as `42: uint`.

A function in type theory is denoted with an arrow, `->`.

For example, the function `add` has the assertion `add: int -> int`, meaning that it takes an integer and returns an integer.


## Dependent type

A dependent type, also called *pi type*, is a type that depends on a term or on another type.

Two common examples of dependent types are *dependent functions* and *dependent pairs*.

A "pair of integers" is a type; a "pair of integers where the second is greater than the first" is a dependent type because of the dependence on the value.

A dependent function's return type may depend on the value of its parameter (not just on its type).

For example, a function that takes a positive integer `n` and returns an array of length `n`.

Dependent types generalize the normal function space to model functions whose output type varies on their input.

While normal function have assertion: `fn() int -> int`

For example, writing `A(R, N)` for the type of n-tuples of real numbers (`R`) and `N` for the type of natural numbers,

∏ Vec(R, n)
n:N

---

https://www.wikiwand.com/en/Type_theory
