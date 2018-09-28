# Predicate logic

All the machinery of propositional logic is included in first-order logic and higher-order logics. In this sense, propositional logic is the foundation of first-order logic and higher-order logic.



Predicate logic is the general term for all logics that use predicates, as in `P(x)` where `P` is a predicate and we say that `P` is predicated of `x`. 

For example, `engineer(a)` means "_engineer is predicated of a_", or loosely, "_a is an engineer_".

Predicate logic is an extension of propositional logic; a proposition is a predicate with no arguments.

Predicate logic also supports variables and quantifiers over variables.
For example, `∀x∃y.P(x,y)` means "_For all_ x _there exists a_ y _such that the proposition_ P(x,y) _is true_".

In first-order predicate logic, variables can appear only inside a predicate i.e. you can quantify over variables, but not predicates themselves.

In second-order logic, you can also quantify over predicates, e.g. 
`∀P ∀x . [P(x) ∨ ¬P(x)]` is true: "_for every predicate P, either P(x) or not P(x) is true, regardless of what x is_".

In set-theoretic terms, a first-order logic quantifies over **individuals**, while a second-order logic quantifies over **sets of individuals**.

Viewed this way, a predicate is identical to the set of individuals that it applies to.

You can make higher-order logics by using **metatypes**, **quantifying over sets of predicates**.

This kind of logic can be used to define type systems for predicate logics, though this kind of use is rather abstruse.

---

First order logic
First-order predicate logic allows variables to range over atomic symbols in the domain. It doesn't allow variables to be bound to predicate symbols, however. A second order logic (such as second order predicate logic) does allow this, and you can write sentences such as: ∀p.p(Socrates).

Higher order logic
A higher order logic allows predicates to accept arguments which are themselves predicates. Second order logic cannot be reduced to first-order logic.

---

In both cases, you have a universe which is a set of elements. In first order logic, you're allowed to quantify over _individual elements of the domain_, so you can make statements like "for a fixed set S, every x is either in S or not in S".

In second order logic, you're allowed to quantify over _relations on the universe_, so you can make statements like "for every set S and element x, either x is in S or x is not in S".




In first-order theories, predicates are often associated with sets. In interpreted higher-order theories, predicates may be interpreted as sets of sets.


