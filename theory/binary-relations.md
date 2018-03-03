# Binary relations

- equivalence relation: symmetric and transitive and reflexive
- partial equivalence: symmetric and transitive

<!-- TOC -->

- [Binary relation](#binary-relation)
- [Relations over a set](#relations-over-a-set)
- [List of some of relations by properties](#list-of-some-of-relations-by-properties)
- [Reflexive relations](#reflexive-relations)
- [Symmetric relations](#symmetric-relations)
- [Transitive relations](#transitive-relations)
- [Equivalence relation](#equivalence-relation)
- [Partial equivalence relation](#partial-equivalence-relation)
- [Links](#links)

<!-- /TOC -->

## Binary relation

A binary relation on a set `A` is a collection of ordered pairs of elements of `A`. In other words, it is a subset of the Cartesian product `A×A`. More generally, a binary relation between two sets `A` and `B` is a subset of `A×B`.

A binary relation R between arbitrary sets X (the set of departure) and Y (the set of destination or codomain) is specified by its graph G, which is a subset of the Cartesian product X × Y.

The binary relation R itself is usually identified with its graph G, but some authors define it as an ordered triple (X, Y, G), which is otherwise referred to as a correspondence.

The statement `(x, y) ∈ G` is read _x is R-related to y_, and is denoted by `xRy` or `R(x, y)`. The latter notation corresponds to viewing `R` as the characteristic function of the subset `G` of `X × Y`, i.e. `R(x, y)` equals to 1 (true), if `(x, y) ∈ G`, and 0 (false) otherwise.

The order of the elements in each pair of `G` is important: if `a ≠ b`, then `aRb` and `bRa` can be true or false, independently of each other. For example, the prime 3 divides the integer 9, but 9 doesn't divide 3.

The domain of `R` is the set of all `x` such that `xRy` for at least one `y`. The range of `R` is the set of all `y` such that `xRy` for at least one `x`. The field of `R` is the union of its domain and its range.


## Relations over a set

If `X = Y` then we simply say that the binary relation is over `X`, or that it is an endorelation over `X`. In computer science, such a relation is also called a homogeneous (binary) relation. The set of all binary relations `Rel(X)` on a set `X` is the set `2^(X × X)`. Some important properties that a binary relation `R` over a set `X` may have:

- **reflexive**   
  `∀x ∈ X : xRx` for all `x` in `X` it holds that `xRx`   
  e.g. `≥` (gt or equal to) is reflexive, but `>` (gt) is not.
- **irreflexive** (or strict)   
  `∀x ∈ X : ¬xRx` for all `x` in `X` it holds that `not xRx`   
  e.g. `>` is an irreflexive relation, but `>=` is not.
- **coreflexive**   
  `∀x,y ∈ X (xRy ⇒ x = y)`   
  for all `x` and `y` in `X` it holds that `if xRy then x = y`.   
  An example of a coreflexive relation is the relation on integers in which each odd number is related to itself and there are no other relations. The equality relation is the only example of a both reflexive and coreflexive relation, and any coreflexive relation is a subset of the identity relation.
- **symmetric**   
  `∀x,y ∈ X (xRy ⇔ yRx)`   
  for all `x` and `y` in `X` it holds that `if xRy then yRx`,   
  "Is a blood relative of" is a symmetric relation, because x is a blood relative of y if and only if y is a blood relative of x.
- **antisymmetric**   
  `∀x,y ∈ X (xRy ∧ yRx) ⇒ x = y` or, equivalently,   
  `∀x,y ∈ X (xRy ∧ x ≠ y ⇒ ¬yRx)`   
  for all `x` and `y` in `X`, if `xRy` and `yRx` then `x = y`   
  For example, `>=` is anti-symmetric; so is `>`, but vacuously   
  (the condition in the definition is always false).
- **asymmetric**   
  for all `x` and `y` in `X`, if `xRy` then not `yRx`.   
  A relation is asymmetric if and only if it is both anti-symmetric and irreflexive. For example, `>` is asymmetric, but `>=` is not.
- **transitive**   
  for all `x`, `y` and `z` in `X` it holds that if `xRy` and `yRz` then `xRz`. For example, "is ancestor of" is transitive, while "is parent of" is not. A transitive relation is irreflexive if and only if it is asymmetric.
- **total**   
  for all `x` and `y` in `X` it holds that `xRy` or `yRx` (or both). This definition for total is different from left total in the previous section. For example, `>=` is a total relation.
- **trichotomous**   
  for all `x` and `y` in `X` exactly one of `xRy`, `yRx` or `x = y` holds. For example, `>` is a trichotomous relation, while the relation "divides" on natural numbers is not.
- **Right Euclidean**   
  for all `x`, `y` and `z` in `X` it holds that if `xRy` and `xRz`, then `yRz`.
- **Left Euclidean**   
  for all `x`, `y` and `z` in `X` it holds that if `yRx` and `zRx`, then `yRz`.
- **Euclidean**   
  A Euclidean relation is both left and right Euclidean. Equality is a Euclidean relation because if `x=y` and `x=z`, then `y=z`.
- **serial**   
  for all `x` in `X`, there exists `y` in `X` such that `xRy`. "Is greater than" is a serial relation on the integers. But it is not a serial relation on the positive integers, because there is no `y` in the positive integers such that `1 > y`. However, "is less than" is a serial relation on the positive integers, the rational numbers and the real numbers. Every reflexive relation is serial: for a given `x`, choose `y = x`. A serial relation can be equivalently characterized as every element having a non-empty successor neighborhood. Similarly an inverse serial relation is a relation in which every element has non-empty predecessor neighborhood.
- **set-like** (or local):   
  for every `x` in `X`, the class of all `y` such that `yRx` is a set. (This makes sense only if relations on proper classes are allowed.) The usual ordering `<` on the class of ordinal numbers is set-like, while its inverse `>` is not.


## List of some of relations by properties

- **equivalence**: reflexive, symmetric, transitive.
- **partial equivalence**: symmetric, transitive.
- **reflexive**  : symmetric, transitive, serial.
- **partial order**: reflexive, antisymmetric, transitive.
- **total order** (linear order, or chain): partial order that is total.
- **well-order**: linear order where every nonempty subset has a least element.


## Reflexive relations
A binary relation `R` over a set `X` is reflexive if every element of `X` is related to itself.

Formal definition: `∀x ∈ X : x R x`

An example of a reflexive relation is the relation "is equal to" on the set of real numbers, since every real number is equal to itself. A reflexive relation is said to have the _reflexive property_ or _reflexivity_. Along with symmetry and transitivity, reflexivity is one of three properties defining equivalence relations.

Examples of reflexive relations include:

"is equal to" (equality)
"is a subset of" (set inclusion)
"divides" (divisibility)
"is greater than or equal to"
"is less than or equal to"

Examples of irreflexive relations include:
"is not equal to"
"is coprime to" (for the integers>1, since 1 is coprime to itself)
"is a proper subset of"
"is greater than"
"is less than"


## Symmetric relations
A binary relation `R` over a set `X` is symmetric if it holds for all `a` and `b` in `X` that `a` is related to `b` if and only if `b` is related to `a`.

Formal definition: `∀a,b ∈ X (aRb ⇔ bRa)`


## Transitive relations
A binary relation `R` over a set `X` is transitive if whenever an element `a` is related to an element `b` and `b` is related to an element `c` then `a` is also related to `c`.

Transitivity (or transitiveness) is a key property of both _partial order_ relations and _equivalence_ relations.

Formal definition: `∀a,b,c ∈ X: (aRb ∧ bRc) ⇒ aRc`




## Equivalence relation
An equivalence relation is a binary relation that is at the same time a *reflexive* relation, a *symmetric* relation and a _transitive_ relation.

As a consequence of these properties an equivalence relation provides a partition of a set into equivalence classes.

Notation: two elements `a` and `b` of a set are equivalent with respect to an equivalence relation `R`: `a ~ b` and `a ≡ b`, which are used when `R` is the obvious relation being referenced, and variations of `a ~R b`, `a ≡R b` or `a R b` otherwise.

Definition: A given binary relation `~` on a set `X` is said to be an equivalence relation if and only if it is reflexive, symmetric and transitive. That is, for all `a`, `b` and `c` in `X`:
- Reflexivity: `a ~ a`
- Symmetry: `a ~ b` if and only if `b ~ a`
- Transitivity: if `a ~ b` and `b ~ c` then `a ~ c`
`X` together with the relation `~` is called a **setoid**. The equivalence class of `a` under `~`, denoted `[a]`, is defined as: 
`[a] = {b ∈ X | a ~ b }`


## Partial equivalence relation
A partial equivalence relation (PER) `R` on a set `X` is a relation that is _symmetric_ and _transitive_.

It holds for all `a, b, c ∈ X` that:
1. if `aRb`, then `bRa` (symmetry)
2. if `aRb` and `bRc`, then `aRc` (transitivity)

If `R` is also reflexive, then `R` is an equivalence relation.


## Links
- [Reflexive relation](https://www.wikipedia.com/en/Reflexive_relation)
- [Symmetric relation](https://www.wikipedia.com/en/Symmetric_relation)
- [Transitive relation](https://www.wikipedia.com/en/Transitive_relation)
- [Equivalence relation](https://www.wikipedia.com/en/Equivalence_relation)
- [Partial equivalence relation](https://www.wikipedia.com/en/Partial_equivalence_relation)
