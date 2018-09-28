# Higher-order logic

https://en.wikipedia.org/wiki/Higher-order_logic

In mathematics and logic, a higher-order logic is a form of predicate logic that is distinguished from first-order logic by additional quantifiers and, sometimes, stronger semantics.

Higher-order logics with their standard semantics are more expressive, but their model-theoretic properties are less well-behaved than those of first-order logic.

The term "_higher-order logic (HOL)_", is commonly used to mean _higher-order simple predicate logic_. Here "_simple_" indicates that the underlying type theory is simple, not polymorphic or dependent.


## Quantification scope
"ordered logic" redirects here. For the unrelated term in linear logic, see noncommutative logic.

First-order logic quantifies only variables that range over individuals; second-order logic, in addition, also quantifies over sets; third-order logic also quantifies over sets of sets, and so on.

Higher-order logic is the union of first-, second-, third-, …, nth-order logic; i.e., higher-order logic admits quantification over sets that are nested arbitrarily deeply.

## Semantics
There are two possible semantics for higher order logic.

In the standard or full semantics, quantifiers over higher-type objects range over all possible objects of that type. For example, a quantifier over sets of individuals ranges over the entire powerset of the set of individuals. Thus, in standard semantics, once the set of individuals is specified, this is enough to specify all the quantifiers. HOL with standard semantics is more expressive than first-order logic. For example, HOL admits categorical axiomatizations of the natural numbers, and of the real numbers, which are impossible with first-order logic. However, by a result of Gödel, HOL with standard semantics does not admit an effective, sound, and complete proof calculus.[2]

The model-theoretic properties of HOL with standard semantics are also more complex than those of first-order logic. For example, the Löwenheim number of second-order logic is already larger than the first measurable cardinal, if such a cardinal exists.[3] The Löwenheim number of first-order logic, in contrast, is ℵ0, the smallest infinite cardinal.

In Henkin semantics, a separate domain is included in each interpretation for each higher-order type. Thus, for example, quantifiers over sets of individuals may range over only a subset of the powerset of the set of individuals. HOL with these semantics is equivalent to many-sorted first-order logic, rather than being stronger than first-order logic. In particular, HOL with Henkin semantics has all the model-theoretic properties of first-order logic, and has a complete, sound, effective proof system inherited from first-order logic.

## Examples and properties
Higher order logics include the offshoots of Church's Simple Theory of Types[4] and the various forms of Intuitionistic type theory. Gérard Huet has shown that unifiability is undecidable in a type theoretic flavor of third-order logic,[5][6][7] that is, there can be no algorithm to decide whether an arbitrary equation between third-order (let alone arbitrary higher-order) terms has a solution.

Up to a certain notion of isomorphism, the powerset operation is definable in second-order logic. Using this observation, Hintikka established in 1955 that second-order logic can simulate higher-order logics in the sense that for every formula of a higher order-logic one can find an equisatisfiable formula for it in second-order logic.[8]

The term "higher-order logic" is assumed in some context to refer to classical higher-order logic. However, modal higher-order logic has been studied as well. According to several logicians, Gödel's ontological proof is best studied (from a technical perspective) in such a context.[9]

