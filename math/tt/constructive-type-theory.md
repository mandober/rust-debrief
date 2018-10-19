# Constructive type theory

Types and propositions seem to have no relation to each other: types come from programming languages and propositions from logic, but if we make certain assumptions about both logic and programming, then we can define a system which is simultaneously a logic and a programming language, and in which propositions and types are identical. This is the system of _constructive type theory_, based primarily on the work of the Swedish logician and philosopher, Per Martin-Lof.


## Correct Programming
Testing cannot ensure the absence of errors; only a formal proof of correctness can guarantee that a program meets its specification. We should try to develop programs in such a way that they must behave according to specification.

The aim is to design a language in which correctness is guaranteed. We look particularly for a functional programming language with this property, as semantically the properties of functional languages are the most straightforward, with a program simply being a value of a particular explicit type (referential transparency), rather than a state transformer (as is the case in imperative languages).

The type system will have to be more powerful: we want to express a specification by means of a statement of the form `p:P`, which is how we write "_the value p has the type P_".

The types in programming languages cannot express the constraint, e.g., that for every numeric input value the result is the positive square root of the value.

If the language allows general recursion, then every type contains at least one value, defined by the equation x = x. This mirrors the observation that a non-terminating program meets every specification if we are only concerned with partial correctness. If we require total correctness we will need to design a language which only permits the definition of total functions and fully-defined objects.

To summarise, we are interested in a language in which correctness is guaranteed just like type-correctness can be guaranteed. Specificaly, we need a type system that can express logical specifications.


## Constructive Logic
If it is contradictory for no object x to have the property `P(x)`, then there is an object x with the property `P(x)`, `¬∀x.¬P(x) ⇒ ∃x.P(x)`

This is a principle of indirect proof, but the problem with it is that it asserts the existence of an object without giving any indication of what the object is. It is a non-constructive method of proof. It is necessary to reject classical logic and to look for modes of reasoning which permit only constructive derivations.

Instead of giving a classical, truth-functional, explanation of what is valid, we will explain what it means for a particular object p to be a proof of the proposition P. Our logic is proof-functional rather than truth-functional.

The crucial explanation is for the existential quantifier.

An assertion that `∃z.P(z)` can only be deduced if we can produce an `a` with the property `P(a)`. A proof of `∃z.P(z)` will therefore be a pair, `(a,p)`, consisting of an object `a` and a proof that `a` does in fact have the property `P`.

A universal statement `∀z.Q(z)` can be deduced only if there is a function taking any object `a` to a proof that `Q(a)`.

If we put these two explanations together, a constructive proof of the statement `∀x.∃y.R(x,y)` can be seen to require that there is a function, e.g. `f`, taking any `a` to a value so that `R(a, f a)`.


Here we see that a constructive proof has computational content, in the shape of a function which gives an explicit witness value `f a` for each `a`.

A proof of the conjunction `A ∧ B` can be seen as a pair of proofs, `(p, q)`, with `p` a proof of `A` and `q` of `B`.

A proof of the implication `A ⇒ B` can be seen as a proof transformation: given a proof of `A`, we can produce a proof of `B` from it.

A proof of the disjunction `A ∨ B` is either a proof of `A` or a proof of `B`, together with an indication which one it is (A or B).

The negation `¬A` is defined to be the implication `A ⇒ ⊥`, where ⊥ is the absurd or false proposition, which has no proof but from which we can infer anything. A proof of `¬A` is thus a function taking a proof of A to a proof of absurdity.

Given these explanations, it is easy to see that the law of the _excluded middle_ will not be valid, as for a general A we cannot say that either A or ¬A is provable. Similarly, the law of _indirect proof_ will not be valid.
