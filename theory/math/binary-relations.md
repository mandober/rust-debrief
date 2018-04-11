# Binary relations

- equivalence relation: symmetric, transitive, reflexive
- partial equivalence: symmetric, transitive

<!-- TOC -->

- [Binary relations](#binary-relations)
- [List of properties](#list-of-properties)
- [List of relations](#list-of-relations)
- [Reflexive](#reflexive)
- [Irreflexive](#irreflexive)
- [Coreflexive](#coreflexive)
- [symmetric](#symmetric)
- [antisymmetric](#antisymmetric)
- [asymmetric](#asymmetric)
- [transitive](#transitive)
- [total](#total)
- [trichotomous](#trichotomous)
- [Euclidean](#euclidean)
- [serial](#serial)
- [set-like (or local)](#set-like-or-local)
- [Symmetric relations](#symmetric-relations)
- [Transitive relations](#transitive-relations)
- [Equivalence relation](#equivalence-relation)
- [Partial equivalence relation](#partial-equivalence-relation)
- [Partially ordered set](#partially-ordered-set)
- [Links](#links)

<!-- /TOC -->

## Binary relations

A binary relation (on a set) is a **collection of ordered pairs** of the set's elements. Binary relations are used to model concepts like "is greater than", "is equal to", and similar. The concept of function is defined as a special kind of binary relation.

An example is the _"divides" relation_ between the set of prime numbers and the set of integers, in which every prime number is associated with every integer that is a multiple of that prime (but with no integer that is not a multiple).

In this relation, for instance, the prime 2 is associated with numbers that include −4, 0, 6, 10, but not 1 or 9; and the prime 3 is associated with numbers that include 0, 6, and 9, but not 4 or 13.

The order of elements in each pair is important; resuming the "divides" relation example, the prime 3 divides the integer 9, but 9 doesn't divide 3.


## List of properties

Some important properties that a binary relation may have:
- reflexive, irreflexive, coreflexive
- symmetric, antisymmetric, asymmetric
- transitive
- total
- trichotomous
- right Euclidean, left Euclidean, Euclidean
- serial
- set-like


## List of relations

List of some relations by properties
- equivalence: reflexive, symmetric, transitive.
- partial equivalence: symmetric, transitive.
- reflexive: symmetric, transitive, serial.
- partial order: reflexive, antisymmetric, transitive.
- total order (linear order, or chain): partial order that is total.
- well-order: linear order where every nonempty subset has a least element.


## Reflexive
A binary relation is reflexive if every element of the set is related to itself.

An example is the relation "is equal to" on the set of real numbers, since every real number is equal to itself. Along with symmetry and transitivity, reflexivity is one of the 3 properties defining _equivalence relations_.

Examples of reflexive relations include:
- "is equal to" (equality)
- "is greater than or equal to"
- "is less than or equal to"
- "is a subset of" (set inclusion)
- "divides" (divisibility)


## Irreflexive
A binary relation is irreflexive (anti-reflexive) if no element is related to itself.

An example is the "greater than" relation on the set of real numbers, since no real number is greater than itself.

Not every relation which is not reflexive is irreflexive; it is possible to define relations where some elements are related to themselves but others are not. For example, the binary relation "the product of x and y is even" is reflexive on the set of even natural numbers, irreflexive on the set of odd natural numbers, and neither reflexive nor irreflexive on the set of natural numbers.

Examples of irreflexive relations include:
- "is not equal to"
- "is a proper subset of"
- "is greater than"
- "is less than"
- "is coprime to" (for the integers > 1, since 1 is coprime to itself)


## Coreflexive
A binary relation is coreflexive

An example of a coreflexive relation is the relation on integers in which each odd number is related to itself and there are no other relations. The equality relation is the only example of a both reflexive and coreflexive relation, and any coreflexive relation is a subset of the identity relation.

## symmetric
"Is a blood relative of" is a symmetric relation, because x is a blood relative of y if and only if y is a blood relative of x.


## antisymmetric
For example, `>=` is anti-symmetric; so is `>`, but vacuously (the condition in the definition is always false).

## asymmetric
A relation is asymmetric if and only if it is both anti-symmetric and irreflexive. For example, `>` is asymmetric, but `>=` is not.

## transitive
For example, "is ancestor of" is transitive, while "is parent of" is not. A transitive relation is irreflexive if and only if it is asymmetric.

## total
This definition for total is different from left total. 
For example, `>=` is a total relation.

## trichotomous
For example, `>` is a trichotomous relation, while the relation "divides" on natural numbers is not.

## Euclidean
A Euclidean relation is both left and right Euclidean. Equality is a Euclidean relation because if `x=y` and `x=z`, then `y=z`.

## serial
"Is greater than" is a serial relation on the integers.
But it is not a serial relation on the positive integers, because there is no `y` in the positive integers such that `1 > y`. 
However, "is less than" is a serial relation on the positive integers, the rational numbers and the real numbers.

Every reflexive relation is serial: for a given `x`, choose `y = x`.
A serial relation can be equivalently characterized as every element having a non-empty successor neighborhood. Similarly an inverse serial relation is a relation in which every element has non-empty predecessor neighborhood.


## set-like (or local)
The usual ordering `<` on the class of ordinal numbers is set-like, while its inverse `>` is not.





## Symmetric relations
A binary relation `R` over a set `X` is symmetric if it holds for all `a` and `b` in `X` that `a` is related to `b` if and only if `b` is related to `a`.

Formal definition: `∀a,b ∈ X (aRb ⇔ bRa)`


## Transitive relations
A binary relation `R` over a set `X` is transitive if whenever an element `a` is related to an element `b` and `b` is related to an element `c` then `a` is also related to `c`.

Transitivity (or transitiveness) is a key property of both _partial order_ relations and _equivalence_ relations.

Formal definition: `∀ a,b,c ∈ X: a ⥽ b ∧ b ⥽ c ⇒ a ⥽ c`


## Equivalence relation
An equivalence relation is a binary relation that is at the same time a *reflexive* relation, a *symmetric* relation and a _transitive_ relation.

As a consequence of these properties an equivalence relation provides a partition of a set into equivalence classes.

Notation: two elements `a` and `b` of a set are equivalent with respect to an equivalence relation `R`: `a ~ b` and `a ≡ b`, which are used when `R` is the obvious relation being referenced, and variations of `a ~R b`, `a ≡R b` or `a R b` otherwise.

Definition: A given binary relation `~` on a set `X` is said to be an equivalence relation if and only if it is reflexive, symmetric and transitive. That is, for all `a`, `b` and `c` in `X`:
- Reflexivity: `a ~ a`
- Symmetry: `a ~ b` if and only if `b ~ a`
- Transitivity: if `a ~ b` and `b ~ c` then `a ~ c`
`X` together with the relation `~` is called a setoid. The equivalence class of `a` under `~`, denoted `[a]`, is defined as: 
`[a] = {b ∈ X | a ~ b }`


## Partial equivalence relation
A partial equivalence relation (PER) `R` on a set `X` is a relation that is _symmetric_ and _transitive_.

It holds for all `a, b, c ∈ X` that:
1. if `aRb`, then `bRa` (symmetry)
2. if `aRb` and `bRc`, then `aRc` (transitivity)

If `R` is also reflexive, then `R` is an equivalence relation.



## Partially ordered set
A partially ordered set (poset) consists of a set together with a binary relation indicating that, for certain pairs of elements in the set, one of the elements precedes the other in the ordering. The word "partial" is used as an indication that not every pair of elements need be comparable: there may be pairs of elements for which neither element precedes the other.

Partial orders thus generalize total orders, in which every pair is comparable.










---


## Links
- [Reflexive relation](http://www.wikipedia.com/en/Reflexive_relation)
- [Symmetric relation](http://www.wikipedia.com/en/Symmetric_relation)
- [Transitive relation](http://www.wikipedia.com/en/Transitive_relation)
- [Equivalence relation](http://www.wikipedia.com/en/Equivalence_relation)
- [Partial equivalence relation](http://www.wikipedia.com/en/Partial_equivalence_relation)
