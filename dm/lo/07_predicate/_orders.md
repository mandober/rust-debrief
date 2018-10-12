<!-- TOC -->

- [Aristotelian logic](#aristotelian-logic)
- [Propositional logic](#propositional-logic)
- [Predicate logic](#predicate-logic)
- [Zeroth-order logic](#zeroth-order-logic)
- [First-order logic](#first-order-logic)
- [Second-order logic](#second-order-logic)
- [Higher order logic](#higher-order-logic)
- [Semantics](#semantics)
- [Examples and properties](#examples-and-properties)

<!-- /TOC -->

## Aristotelian logic
Term logic, also known as traditional logic, syllogistic logic or Aristoteli an logic, is a loose name for an approach to logic that began with Aristotle and that was dominant until the advent of modern logic in the late XIX century.


## Propositional logic
deals with propositions and argument flow. Compound propositions are formed by connecting propositions by logical connectives. The propositions without logical connectives are called atomic propositions. Unlike first-order logic, propositional logic does not deal with non-logical objects, predicates about them, or quantifiers. However, all the machinery of propositional logic is included in first-order logic and higher-order logics. In this sense, propositional logic is the foundation of first-order logic and higher-order logic.


## Predicate logic
is in principle sufficient for math as it has a sound and complete deduction system and satisfies important semantic results. Lindstrom's theorems show that there can be no logical system with more expressive power than predicate logic and with the same good semantic properties.


Predicate logic is the general term for all logics that use predicates, as in `P(x)` where `P` is a predicate symbol.

We say that `P` is predicated of `x`. For example, `engineer(a)` means "_engineer is predicated of a_", or loosely, "_a is an engineer_".

Predicate logic also supports variables and quantifiers over variables. For example, `∀x∃y.P(x,y)` means "For all x there exists a y such that the proposition P(x,y) is true".


# Orders

- FOL
- SOL
- HOL

In set-theoretic terms, a first-order logic quantifies over **individuals**, while a second-order logic quantifies over **sets of individuals**.

Viewed this way, a predicate is identical to the set of individuals that it applies to.

You can make higher-order logics by using **metatypes**, **quantifying over sets of predicates**.

This kind of logic can be used to define type systems for predicate logics, though this kind of use is rather abstruse.

## Zeroth-order logic
is, under one definition, first-order logic without variables or quantifiers. Another definition extends propositional logic by adding constants, operations, and relations on non-Boolean values. Every zeroth-order language in this broader sense is complete and compact.


## First-order logic
First-order in logic uses means "_without self-reference_" (in first-order logic); as opposed to "allowing some self-reference" (in higher-order logic).

A formal deductive system 
extended from propositional logic 
with the possibility to quantify over 
(individual members of the universe)
individuals of the domain of discourse.

The adjective "first-order" distinguishes first-order logic from higher-order logic in which there are predicates having predicates or functions as arguments, or in which one or both of predicate quantifiers or function quantifiers are permitted.


First-order logic quantifies only variables that range over individuals; second-order logic, in addition, also quantifies over sets; third-order logic also quantifies over sets of sets, and so on.

Higher-order logic is the union of first-, second-, third-, …, nth-order logic; i.e., higher-order logic admits quantification over sets that are nested arbitrarily deeply.

The characteristic feature of first-order logic is that individuals can be quantified, but not predicates. Second-order logic extends first-order logic by adding the latter type of quantification. Other higher-order logics allow quantification over even higher types than second-order logic permits. These higher types include relations between relations, functions from relations to relations between relations, and other higher-type objects. Thus _the "first" in first-order logic describes the type of objects that can be quantified_.



## Second-order logic
https://en.wikipedia.org/wiki/Second-order_logic

A formal system which 
extended from first-order logic 
with the possibility to quantify over 
relations between members of the universe.


First-order logic quantifies only variables that range over individuals; second-order logic, in addition, also quantifies over sets; third-order logic also quantifies over sets of sets, and so on.

Higher-order logic is the union of first-, second-, third-, …, nth-order logic; i.e., higher-order logic admits quantification over sets that are nested arbitrarily deeply.

In logic and mathematics second-order logic is an extension of first-order logic, which itself is an extension of propositional logic. Second-order logic is in turn extended by higher-order logic and type theory.

First-order logic quantifies only variables that range over individuals (elements of the domain of discourse); second-order logic, in addition, also quantifies over relations. For example, the second-order sentence \forall P\,\forall x(x\in P\lor x\notin P) says that for every unary relation (or set) P of individuals, and every individual x, either x is in P or it is not (this is the principle of bivalence). Second-order logic also includes quantification over sets, functions, and other variables as explained in the section Syntax and fragments. Both first-order and second-order logic use the idea of a domain of discourse (often called simply the "domain" or the "universe"). The domain is a set over which individual elements may be quantified.

The syntax of second-order logic tells which expressions are well formed formulas. In addition to the syntax of first-order logic, second-order logic includes many new sorts (sometimes called types) of variables. 



## Higher order logic

https://en.wikipedia.org/wiki/Higher-order_logic

In both cases, you have a universe which is a set of elements. In first order logic, you're allowed to quantify over _individual elements of the domain_, so you can make statements like "for a fixed set S, every x is either in S or not in S".

In second order logic, you're allowed to quantify over _relations on the universe_, so you can make statements like "for every set S and element x, either x is in S or x is not in S".

In first-order theories, predicates are often associated with sets. In interpreted higher-order theories, predicates may be interpreted as sets of sets.


higher-order logic is a form of predicate logic that is distinguished from first-order logic by additional quantifiers and, sometimes, stronger semantics.

Higher-order logics with their standard semantics are more expressive, but their model-theoretic properties are less well-behaved than those of first-order logic.

The term "higher-order logic (HOL)", is commonly used to mean higher-order simple predicate logic. Here "simple" indicates that the underlying type theory is simple, not polymorphic or dependent.


## Semantics
There are two possible semantics for higher order logic.

In the standard or full semantics, quantifiers over higher-type objects range over all possible objects of that type. For example, a quantifier over sets of individuals ranges over the entire powerset of the set of individuals. Thus, in standard semantics, once the set of individuals is specified, this is enough to specify all the quantifiers. HOL with standard semantics is more expressive than first-order logic. For example, HOL admits categorical axiomatizations of the natural numbers, and of the real numbers, which are impossible with first-order logic. However, by a result of Gödel, HOL with standard semantics does not admit an effective, sound, and complete proof calculus.

The model-theoretic properties of HOL with standard semantics are also more complex than those of first-order logic. For example, the Löwenheim number of second-order logic is already larger than the first measurable cardinal, if such a cardinal exists.[3] The Löwenheim number of first-order logic, in contrast, is ℵ0, the smallest infinite cardinal.

In Henkin semantics, a separate domain is included in each interpretation for each higher-order type. Thus, for example, quantifiers over sets of individuals may range over only a subset of the powerset of the set of individuals. HOL with these semantics is equivalent to many-sorted first-order logic, rather than being stronger than first-order logic. In particular, HOL with Henkin semantics has all the model-theoretic properties of first-order logic, and has a complete, sound, effective proof system inherited from first-order logic.

## Examples and properties
Higher order logics include the offshoots of Church's Simple Theory of Types and the various forms of Intuitionistic type theory. Gérard Huet has shown that unifiability is undecidable in a type theoretic flavor of third-order logic, that is, there can be no algorithm to decide whether an arbitrary equation between third-order (let alone arbitrary higher-order) terms has a solution.

Up to a certain notion of isomorphism, the powerset operation is definable in second-order logic. Using this observation, Hintikka established in 1955 that second-order logic can simulate higher-order logics in the sense that for every formula of a higher order-logic one can find an equisatisfiable formula for it in second-order logic.

The term "higher-order logic" is assumed in some context to refer to classical higher-order logic. However, modal higher-order logic has been studied as well. According to several logicians, Gödel's ontological proof is best studied (from a technical perspective) in such a context.
