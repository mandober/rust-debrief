# Propositional logic

AKA propositional calculus, statement logic, sentential calculus, sentential logic, zeroth-order logic.


**Propositional logic** deals with propositions and argument flow. Compound propositions are formed by connecting propositions by logical connectives. The propositions without logical connectives are called atomic propositions.

Unlike first-order logic, propositional logic does not deal with non-logical objects, predicates about them, or quantifiers. However, all the machinery of propositional logic is included in first-order logic and higher-order logics. In this sense, propositional logic is the foundation of first-order logic and higher-order logic.



Propositional logic is based on __propositions__, which are declarative sentences that have a __truth value__, which means they can be evaluated to true or false (i.e. truth values are true and false).

The argument is **valid** if the conclusion logically follows from the propositions - whether the propositions are actually true is not important, only the form; we are only concerned with the logical structure of statements, not with their semantics for we want to transform declarative sentences into formulas in order to make arguments rigorous. We also need a __symbolic language__ in which we can express sentences in a way that brings out their logical structure since this lets us concentrate on the argumentation alone.

Similarly to transforming a subset of a natural language, we can transform a subset of a programming language, opening up the possibility for __automatic program verification__ based on the __rules of logic__.

By identifying __atomic__ (indecomposable) declarative statements we can assign symbols, e.g. $$p,q,r$$, to each atom, allowing us to make additional, more complex statements by composing the atoms with __logical connectives__ according to the rules.



## Syntactic rules

Given a set $$A=\{p,q,r,\dots\}$$ of atomic propositions, the language of propositional logic is constructed according to the following rules:   

$$\phi ::= p \in A |\ p\ | \lnot \phi\ |\ \phi \lor \phi\ |\ \phi \land \phi\ |\ \phi \to \phi\ |\ \phi \iff \phi$$

We can now construct propositions such as: $$(p\land q) \to (\lnot r \lor q)$$ (if $$p$$ and $$q$$ then not $$r$$ or $$q$$).