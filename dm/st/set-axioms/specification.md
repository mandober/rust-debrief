# Axiom Schema of Specification

https://www.wikiwand.com/en/Axiom_schema_of_specification

aka:
- Axiom Schema of Specification
- Axiom Schema of Separation
- Subset Axiom Scheme
- Axiom Schema of Restricted Comprehension
- Axiom Schema Of Comprehension (some mathematicians call it this although others use that term for unrestricted comprehension)

In many popular versions of axiomatic set theory the Axiom Schema of Specification is an axiom schema.

> Axiom Schema of Specification essentially says that any definable subclass of a set is a set.

Since restricting comprehension solved Russell's paradox, several mathematicians including Zermelo, Fraenkel and Gödel considered it the most important axiom of set theory.


One instance of the schema is included for each formula $$\phi$$ in the language of set theory with free variables among $$x,w_1,\dots,w_n,A$$. So $$B$$ is not free in $$\phi$$. In the formal language of set theory, the axiom schema is:

$$
\forall w_1,\ldots,w_n\ \\
\forall A\ 
\exists B\ \\
\forall x\ 
(x \in B \Leftrightarrow [x \in A \land \varphi(x, w_1, \ldots, w_n , A)])
$$

given any set $$A$$, 
there exists a set $$B$$ 
such that, 
  given any set $$x$$ 
  $$x$$ is a member of $$B$$ 
  iff $$x$$ is a member of $$A$$ 
  and $$\phi$$ holds for $$x$$.

Because this is an axiom schema so there is one axiom for every such predicate $$\phi$$.


> Since the set B must be a subset of A, the axiom schema really says that: given a set A and a predicate P, we can find a subset B of the set A whose members are the elements of A that satisfy the predicate P.

By the axiom of extensionality this set is unique.

We can denote this set using set-builder notation:
$$\{C \in A : P(C)\}$$


> The essence of the axiom is that every subclass of a set that is defined by a predicate is itself a set.


The axiom schema of specification is characteristic of systems of axiomatic set theory related to the usual set theory ZFC, but does not usually appear in radically different systems of alternative set theory. For example, New Foundations and positive set theory use different restrictions of the axiom of comprehension of naive set theory. The Alternative Set Theory of Vopenka makes a specific point of allowing proper subclasses of sets, called semisets. Even in systems related to ZFC, this scheme is sometimes restricted to formulas with bounded quantifiers, as in Kripke–Platek set theory with urelements.





---

Subsets are commonly constructed using set builder notation.

For example, the even integers can be constructed as the subset of the integers $$\mathbb{Z}$$ satisfying the congruence modulo predicate $$x \equiv 0 \pmod 2$$:

$$x \in \mathbb{Z} : x \equiv 0 \pmod 2$$.

In general, the subset of a set $$z$$ obeying a formula $$\phi (x)$$ with one free variable $$x$$ may be written as:

$$x\in z:\phi (x)$$


The axiom schema of specification states that this subset always exists (it is an axiom schema because there is one axiom for each $$\phi$$).

Formally, let $$\phi$$ be any formula in the language of ZFC with all free variables among $$x, z, w_{1}, \ldots, w_n$$ ($$y$$ is not free in $$\phi$$)

Then:

$$
\forall z
\forall w_{1}
\forall w_{2}
\ldots 
\forall w_{n}
\exists y
\forall x[
  x \in y \Leftrightarrow (x \in z \land \phi)
]
$$


Note that the axiom schema of specification can only construct subsets, and does not allow the construction of sets of the more general form:

\{x:\phi (x)\}.
This restriction is necessary to avoid Russell's paradox and its variants that accompany naive set theory with unrestricted comprehension.

In some other axiomatizations of ZF, this axiom is redundant in that it follows from the axiom schema of replacement and the axiom of the empty set.

On the other hand, the axiom of specification can be used to prove the existence of the empty set, denoted \varnothing , once at least one set is known to exist (see above). One way to do this is to use a property \phi  which no set has. For example, if w is any existing set, the empty set can be constructed as

\varnothing =\{u\in w\mid (u\in u)\land \lnot (u\in u)\}.
Thus the axiom of the empty set is implied by the nine axioms presented here. The axiom of extensionality implies the empty set is unique (does not depend on w). It is common to make a definitional extension that adds the symbol \varnothing  to the language of ZFC.


